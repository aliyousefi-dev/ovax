use std::fs;
use std::time::Instant;
use serde_json::Value;
use encoding_rs::UTF_16LE;
use crate::checksum;
use crate::utils;

pub fn execute_hash(file: &str, verbose: bool) -> Result<(), String> {
    let start = Instant::now();

    // Read the raw bytes of the file (UTF-16LE encoded)
    let file_content = fs::read(file).map_err(|e| format!("Failed to read file: {}", e))?;

    // Convert the UTF-16LE bytes to UTF-8
    let (utf8_content, _, had_errors) = UTF_16LE.decode(&file_content);
    if had_errors {
        eprintln!("Warning: There were errors while decoding the UTF-16LE file.");
    }

    // Parse the UTF-8 content as JSON
    let files_json: Value = serde_json::from_str(&utf8_content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Ensure "files" is an array of strings
    let files = files_json["files"]
        .as_array()
        .ok_or("Expected 'files' to be an array of strings")?
        .iter()
        .filter_map(|f| f.as_str().map(|s| s.to_string()))
        .collect::<Vec<String>>();

    // Call the sha256_multiple_file_hashes function
    match checksum::sha256::sha256_multiple_file_hashes(files) {
        Ok(hashes) => {
            let duration = start.elapsed();
            let res = serde_json::json!({
                "status": "success",
                "hashes": hashes,
                "elapsed_time_ms": duration.as_millis()
            });
            utils::print_json(&res, verbose);
        }
        Err(e) => {
            let res = serde_json::json!({
                "status": "error",
                "message": format!("Failed to hash files: {}", e)
            });
            utils::print_json(&res, verbose);
            std::process::exit(1);
        }
    }

    Ok(())
}
