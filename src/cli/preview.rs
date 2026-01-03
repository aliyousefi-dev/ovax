// cli/preview.rs
use std::path::Path;
use crate::preview::webm;
use crate::utils;
use std::io::{self};

pub fn execute_preview(input: &str, output: &str, start: &f64, duration: &f64) -> io::Result<()> {
    let input_path = Path::new(input);
    let output_path = Path::new(output);

    // Call the `generate_preview` function from the `webm` module
    match webm::generate_preview(input_path, output_path, *start, *duration) {
        Ok(_) => {
            let res = serde_json::json!({
                "status": "success",
                "output": output
            });
            utils::print_json(&res, true); // Assuming verbose is true for simplicity
        }
        Err(e) => {
            let res = serde_json::json!({
                "status": "error",
                "message": e
            });
            utils::print_json(&res, true);
            std::process::exit(1);
        }
    }

    Ok(())
}
