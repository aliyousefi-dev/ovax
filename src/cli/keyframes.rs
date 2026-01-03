use crate::sprite;
use crate::utils;
use std::path::Path;
use std::io::{self};

pub fn execute_keyframes(input: &str) -> io::Result<()> {
    let input_path = Path::new(input);
    match sprite::keyscan::find_keyframes(input_path) {
        Ok(result) => {
            // The `result` is already the struct we want to serialize.
            utils::print_json(&result, true); // Assuming verbose is true for simplicity
        }
        Err(e) => {
            let res = serde_json::json!({"status": "error", "message": e});
            utils::print_json(&res, true);
            std::process::exit(1);
        }
    }

    Ok(())
}