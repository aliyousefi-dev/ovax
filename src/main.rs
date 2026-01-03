mod utils;
mod scan;
mod cli;
mod preview; // Import your new preview.rs logic
mod thumb;
mod sprite;
mod filehash;

use std::path::Path;
use clap::Parser;
use cli::{Cli, Commands};
use std::{fs, time::Instant};
use std::io::{self, Write};
use serde_json::Value;
use encoding_rs::{WINDOWS_1252, UTF_16LE};

fn main() {
    // Initialize FFmpeg
    ffmpeg_next::init().expect("Failed to initialize FFmpeg");

    let cli = Cli::parse();

    match &cli.command {
     Commands::Scan { path, simple } => {
    if !(*simple) {
        let videos = scan::detail::collect_videos(path);
        utils::print_json(&videos, cli.verbose);
    } else {
        let videos = scan::videos::scanvideos(path);

            let json_output = serde_json::json!({
        "files": videos
    });
    
      // Serialize the video list as pretty JSON
    match serde_json::to_string_pretty(&json_output) {
        Ok(json_string) => {
            let stdout = io::stdout();
            let mut handle = io::BufWriter::new(stdout.lock()); // Lock and buffer

            // Write the JSON output to stdout
            if let Err(e) = writeln!(handle, "{}", json_string) {
                eprintln!("Error writing to stdout: {}", e);
            }

            // Ensure the buffer is flushed after writing
            handle.flush().unwrap();
        }
        Err(e) => {
            eprintln!("Error serializing JSON: {}", e);
            std::process::exit(1);
        }
    }
    }
}
        Commands::Thumb { input, output } => {
            let input_path = Path::new(input);
            let output_path = Path::new(output);

            match thumb::frame::extract_middle_frame(input_path, output_path) {
                Ok(_) => {
                    let res = serde_json::json!({"status": "success", "output": output});
                    utils::print_json(&res, cli.verbose);
                }
                Err(e) => {
                    let res = serde_json::json!({"status": "error", "message": e});
                    utils::print_json(&res, cli.verbose);
                    std::process::exit(1);
                }
            }
        }

        Commands::Preview { input, output, start, duration } => {
            let input_path = Path::new(input);
            let output_path = Path::new(output);

            match preview::webm::generate_preview(input_path, output_path, *start, *duration) {
                Ok(_) => {
                    let res = serde_json::json!({"status": "success", "output": output});
                    utils::print_json(&res, cli.verbose);
                }
                Err(e) => {
                    let res = serde_json::json!({"status": "error", "message": e});
                    utils::print_json(&res, cli.verbose);
                    std::process::exit(1);
                }
            }
        }

         Commands::Keyframes { input } => {
            let input_path = Path::new(input);
            match sprite::keyscan::find_keyframes(input_path) {
                Ok(result) => {
                    // The `result` is already the struct we want to serialize.
                    utils::print_json(&result, cli.verbose);
                }
                Err(e) => {
                    let res = serde_json::json!({"status": "error", "message": e});
                    utils::print_json(&res, cli.verbose);
                    std::process::exit(1);
                }
            }
        }
                
        Commands::Sprite { input, output_dir, rows, cols, width, height } => {
            let start = Instant::now();
            let input_path = Path::new(input);
            let output_path = Path::new(output_dir);
            let temp_dir = output_path.join("temp_frames_cache");

            // Ensure output directory exists
            let _ = std::fs::create_dir_all(output_path);

            // 1. Run Extraction and Scanning
            let process_result = (|| -> Result<usize, String> {
                // --- NEW STEP: Scan for Keyframe Data ---
                let scan_result = sprite::keyscan::find_keyframes(input_path)?;
                
                let manifest = sprite::manifest::generate_sprite_manifest(
    &scan_result, *rows, *cols, *width, *height
);

let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();
std::fs::write(output_path.join("sprite.json"), manifest_json).unwrap();

                // Save the JSON data to the output directory
                let json_path = output_path.join("keyframes.json");
                let json_string = serde_json::to_string_pretty(&scan_result)
                    .map_err(|e| format!("JSON serialization error: {}", e))?;
                std::fs::write(json_path, json_string)
                    .map_err(|e| format!("Failed to write keyframes.json: {}", e))?;

                // Step A: Extract keyframes (the actual images)
                let frame_paths = sprite::keyextract::extract_keyframes_to_disk(
                    input_path, 
                    &temp_dir, 
                    *width, 
                    *height
                )?;

                let count = frame_paths.len();

                // Step B: Stitch them together
                sprite::stitch::stitch_frames_into_sprites(
                    &frame_paths, 
                    output_path, 
                    *rows, 
                    *cols, 
                    *width, 
                    *height
                )?;

                Ok(count)
            })();

            // 2. Handle Output (Keep your existing match block here...)
            match process_result {
                Ok(frame_count) => {
                    let duration = start.elapsed();
                    let res = serde_json::json!({
                        "status": "success",
                        "message": format!("Processed {} keyframes into sprite sheets and keyframes.json", frame_count),
                        "directory": output_dir,
                        "elapsed_time_ms": duration.as_millis()
                    });
                    utils::print_json(&res, cli.verbose);
                }
                Err(e) => {
                    let duration = start.elapsed();
                    if temp_dir.exists() {
                        let _ = std::fs::remove_dir_all(&temp_dir);
                    }
                    let res = serde_json::json!({
                        "status": "error",
                        "message": e,
                        "elapsed_time_ms": duration.as_millis()
                    });
                    utils::print_json(&res, cli.verbose);
                    std::process::exit(1);
                }
            }
        }
 Commands::Hash { file } => {
    let start = Instant::now();

    // Read the raw bytes of the file (UTF-16LE encoded)
    let file_content = fs::read(file).expect("Failed to read JSON file");

    // Convert the UTF-16LE bytes to UTF-8
    let (utf8_content, _, had_errors) = UTF_16LE.decode(&file_content);
    if had_errors {
        eprintln!("Warning: There were errors while decoding the UTF-16LE file.");
    }

    // Now we can safely parse the UTF-8 content as JSON
    let files_json: Value = serde_json::from_str(&utf8_content).expect("Failed to parse JSON");

    // Ensure "files" is an array of strings
    let files = files_json["files"]
        .as_array()
        .expect("Expected 'files' to be an array of strings")
        .iter()
        .filter_map(|f| f.as_str().map(|s| s.to_string()))
        .collect::<Vec<String>>();

    // Using the new sha256_multiple_file_hashes function
    match filehash::sha256_multiple_file_hashes(files) {
        Ok(hashes) => {
            let duration = start.elapsed();
            let res = serde_json::json!({
                "status": "success",
                "hashes": hashes,
                "elapsed_time_ms": duration.as_millis()
            });
            utils::print_json(&res, cli.verbose);
        }
        Err(e) => {
            let res = serde_json::json!({
                "status": "error",
                "message": format!("Failed to hash files: {}", e)
            });
            utils::print_json(&res, cli.verbose);
            std::process::exit(1);
        }
    }
}
        Commands::Clean => {
            println!("Cleaning up...");
        }
    }
}