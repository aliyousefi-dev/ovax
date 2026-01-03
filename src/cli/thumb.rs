// cli/thumb.rs
use std::path::Path;
use crate::thumb::frame;
use crate::utils;
use std::io::{self};

pub fn execute_thumb(input: &str, output: &str) -> io::Result<()> {
    let input_path = Path::new(input);
    let output_path = Path::new(output);

    match frame::extract_middle_frame(input_path, output_path) {
        Ok(_) => {
            let res = serde_json::json!({"status": "success", "output": output});
            utils::print_json(&res, true); // Assuming verbose is true for simplicity
        }
        Err(e) => {
            let res = serde_json::json!({"status": "error", "message": e});
            utils::print_json(&res, true);
            std::process::exit(1);
        }
    }

    Ok(())
}
