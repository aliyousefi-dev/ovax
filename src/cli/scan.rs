// cli/scan.rs
use crate::utils;
use crate::scan::{detail, videos};
use std::{ io::{self, Write}};

pub fn execute_scan(path: &str, simple: bool, verbose: bool) -> io::Result<()> {
    if !simple {
        let videos = detail::collect_videos(path);
        utils::print_json(&videos, verbose);
    } else {
        let videos = videos::scanvideos(path);

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

    Ok(())
}
