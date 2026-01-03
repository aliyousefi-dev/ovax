use crate::cli::handler::{Commands};
use crate::cli::scan;
use crate::cli::thumb;
use crate::cli::preview;
use crate::cli::sprite;
use crate::cli::hash;
use crate::cli::keyframes;

pub fn cli_handler(command: Commands, verbose: bool) {
    match command {
        Commands::Scan { path, simple } => {
            if let Err(e) = scan::execute_scan(&path, simple, verbose) {
                eprintln!("Error in scan command: {}", e);
            }
        }
        Commands::Thumb { input, output } => {
            if let Err(e) = thumb::execute_thumb(&input, &output) {
                eprintln!("Error in thumb command: {}", e);
            }
        }
        Commands::Preview { input, output, start, duration } => {
            if let Err(e) = preview::execute_preview(&input, &output, &start, &duration) {
                eprintln!("Error in preview command: {}", e);
            }
        }
        Commands::Keyframes { input } => {
            if let Err(e) = keyframes::execute_keyframes(&input) {
                eprintln!("Error in keyframes command: {}", e);
            }
        }
        Commands::Sprite { input, output_dir, rows, cols, width, height } => {
            if let Err(e) = sprite::execute_sprite(&input, &output_dir, &rows, &cols, &width, &height, verbose) {
                eprintln!("Error in sprite command: {}", e);
            }
        }
        Commands::Hash { file } => {
            if let Err(e) = hash::execute_hash(&file, verbose) {
                eprintln!("Error in hash command: {}", e);
            }
        }
        Commands::Clean => {
            println!("Cleaning up...");
        }
    }
}
