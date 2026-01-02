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
use std::time::Instant;
use std::io::{self, Write};

fn main() {
    // Initialize FFmpeg
    ffmpeg_next::init().expect("Failed to initialize FFmpeg");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan { path,simple } => {
        if !(*simple) {
                let videos = scan::detail::collect_videos(path);
                utils::print_json(&videos, cli.verbose);
            } else {
let videos = scan::videos::scanvideos(path);

let stdout = io::stdout();
let mut handle = io::BufWriter::new(stdout.lock()); // Lock and buffer

for v in videos {
    // We use writeln! instead of println!
    // It writes to the buffer in memory instead of the screen
    if let Err(e) = writeln!(handle, "{}", v.display()) {
        eprintln!("Error writing to stdout: {}", e);
        break;
    }
}

// The buffer automatically flushes when 'handle' goes out of scope, 
// but you can do it manually too:
handle.flush().unwrap();
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
        Commands::Hash { input } => {
            let start = Instant::now();
            let input_path = Path::new(input);

            // Using the function from your filehash module
            match filehash::sha256_file_hash(input_path) {
                Ok(hash_str) => {
                    let duration = start.elapsed();
                    let res = serde_json::json!({
                        "status": "success",
                        "hash": hash_str,
                        "file": input,
                        "elapsed_time_ms": duration.as_millis()
                    });
                    utils::print_json(&res, cli.verbose);
                }
                Err(e) => {
                    let res = serde_json::json!({
                        "status": "error",
                        "message": format!("Failed to hash file: {}", e)
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