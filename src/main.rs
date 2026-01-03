mod utils;
mod scan;
mod cli;
mod preview; // Import your new preview.rs logic
mod thumb;
mod sprite;
mod filehash;

use clap::Parser;
use cli::cli::{Cli, Commands};



fn main() {
    // Initialize FFmpeg
    ffmpeg_next::init().expect("Failed to initialize FFmpeg");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan { path, simple } => {
            if let Err(e) = cli::scan::execute_scan(path, *simple, cli.verbose) {
                eprintln!("Error in scan command: {}", e);
            }
        }
        Commands::Thumb { input, output } => {
                    if let Err(e) = cli::thumb::execute_thumb (input, output) {
                eprintln!("Error in scan command: {}", e);
            }
        }

        Commands::Preview { input, output, start, duration } => {
            if let Err(e) = cli::preview::execute_preview(input, output, start, duration) {
                eprintln!("Error in preview command: {}", e);
            }
        }

         Commands::Keyframes { input } => {
            if let Err(e) = cli::keyframes::execute_keyframes(input) {
                eprintln!("Error in keyframes command: {}", e);
            }
        }
                
        Commands::Sprite { input, output_dir, rows, cols, width, height } => {
        if let Err(e) = cli::sprite::execute_sprite(input, output_dir, rows, cols, width, height, cli.verbose) {
                eprintln!("Error in sprite command: {}", e);
            }
        }
         Commands::Hash { file } => {
            if let Err(e) = cli::hash::execute_hash(file, cli.verbose) {
                eprintln!("Error in hash command: {}", e);
            }
        }
        Commands::Clean => {
            println!("Cleaning up...");
        }
    }
}