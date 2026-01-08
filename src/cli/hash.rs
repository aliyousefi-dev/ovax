use std::fs;
use std::time::Instant;
use serde_json::Value;
use encoding_rs::UTF_16LE;
use crate::checksum;
use crate::utils;
use crate::checksum::types::HashResults;

pub fn execute_hash(file: &str, verbose: bool) -> Result<(), String> {
    let start = Instant::now();

    // 1. Read and Decode
    let utf8_content = read_and_decode_file(file)?;

    // 2. Parse JSON
    let files = parse_json_files(&utf8_content)?;

    // 3. Hash Files
    let results = hash_files(files);

    // 4. Output Results
    print_json_output(results, start, verbose)
}

// --- SUB-FUNCTIONS ---

/// Reads raw bytes and converts UTF-16LE to a standard Rust String
fn read_and_decode_file(file: &str) -> Result<String, String> {
    let file_content = fs::read(file).map_err(|e| format!("Failed to read file: {}", e))?;
    
    let (utf8_content, _, had_errors) = UTF_16LE.decode(&file_content);
    if had_errors {
        eprintln!("Warning: There were errors while decoding the UTF-16LE file.");
    }
    
    Ok(utf8_content.to_string())
}

/// Extracts the list of file paths from the JSON string
fn parse_json_files(content: &str) -> Result<Vec<String>, String> {
    let files_json: Value = serde_json::from_str(content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    let file_list = files_json["files"]
        .as_array()
        .ok_or("Expected 'files' to be an array of strings")?
        .iter()
        .filter_map(|f| f.as_str().map(|s| s.to_string()))
        .collect::<Vec<String>>();
        
    Ok(file_list)
}

/// Orchestrates the parallel hashing process
fn hash_files(files: Vec<String>) -> HashResults {
    checksum::mutisha256::sha256_multiple_file_hashes(files)
}

/// Finalizes the report and prints it to the console
fn print_json_output(results: HashResults, start: Instant, verbose: bool) -> Result<(), String> {
    let duration = start.elapsed();
    
    let res = serde_json::json!({
        "status": "success",
        "data": results,
        "elapsed_time_ms": duration.as_millis()
    });

    utils::print_json(&res, verbose);
    Ok(())
}