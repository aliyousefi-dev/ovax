use std::path::{Path};
use std::fs;
use std::time::Instant;
use serde_json;
use crate::sprite::{keyscan, manifest, keyextract, stitch};
use crate::utils;

pub fn execute_sprite(input: &str, output_dir: &str, rows: &u32, cols: &u32, width: &u32, height: &u32, verbose: bool) -> Result<(), String> {
    let start = Instant::now();
    let input_path = Path::new(input);
    let output_path = Path::new(output_dir);
    let temp_dir = output_path.join("temp_frames_cache");

    // Ensure output directory exists
    let _ = fs::create_dir_all(output_path);

    // 1. Run Extraction and Scanning
    let process_result = (|| -> Result<usize, String> {
        // --- NEW STEP: Scan for Keyframe Data ---
        let scan_result = keyscan::find_keyframes(input_path)?;

        let manifest = manifest::generate_sprite_manifest(
            &scan_result, *rows, *cols, *width, *height
        );

        let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();
        fs::write(output_path.join("sprite.json"), manifest_json).unwrap();

        // Save the JSON data to the output directory
        let json_path = output_path.join("keyframes.json");
        let json_string = serde_json::to_string_pretty(&scan_result)
            .map_err(|e| format!("JSON serialization error: {}", e))?;
        fs::write(json_path, json_string)
            .map_err(|e| format!("Failed to write keyframes.json: {}", e))?;

        // Step A: Extract keyframes (the actual images)
        let frame_paths = keyextract::extract_keyframes_to_disk(
            input_path,
            &temp_dir,
            *width,
            *height
        )?;

        let count = frame_paths.len();

        // Step B: Stitch them together
        stitch::stitch_frames_into_sprites(
            &frame_paths,
            output_path,
            *rows,
            *cols,
            *width,
            *height
        )?;

        Ok(count)
    })();

    // 2. Handle Output (similar to your existing code)
    match process_result {
        Ok(frame_count) => {
            let duration = start.elapsed();
            let res = serde_json::json!({
                "status": "success",
                "message": format!("Processed {} keyframes into sprite sheets and keyframes.json", frame_count),
                "directory": output_dir,
                "elapsed_time_ms": duration.as_millis()
            });
            utils::print_json(&res, verbose);
        }
        Err(e) => {
            let duration = start.elapsed();
            if temp_dir.exists() {
                let _ = fs::remove_dir_all(&temp_dir);
            }
            let res = serde_json::json!({
                "status": "error",
                "message": e,
                "elapsed_time_ms": duration.as_millis()
            });
            utils::print_json(&res, verbose);
            std::process::exit(1);
        }
    }

    Ok(())
}
