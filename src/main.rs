mod utils;
mod scan;
mod cli;
mod preview; 
mod thumb;
mod sprite;
mod checksum;

use clap::{Parser};
use cli::handler::{Cli};
use cli::init::cli_handler; // Import the cli_handler function

fn main() {
    // Initialize FFmpeg
    ffmpeg_next::init().expect("Failed to initialize FFmpeg");

    let cli = Cli::parse();

    // Delegate the command handling to cli_handler
    cli_handler(cli.command, cli.verbose);
}
