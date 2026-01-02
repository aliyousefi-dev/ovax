use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ova-rust", version = "0.1.0", about = "Video Engine")]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan a directory for videos and return JSON
    Scan {
        path: String,
        #[arg(short, long)] // This makes it -t or --thumb
        simple: bool,
    },
    /// Extract a thumbnail from the middle of a video
    Thumb {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: String,
    },
    /// Create a short video preview clip (WebM)
    Preview {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: String,
        #[arg(short, long, default_value_t = 0.0)]
        start: f64,
        #[arg(short, long, default_value_t = 5.0)]
        duration: f64,
    },

    Sprite {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output_dir: String,
        #[arg(long, default_value_t = 5)]
        rows: u32,
        #[arg(long, default_value_t = 5)]
        cols: u32,
        #[arg(long, default_value_t = 160)]
        width: u32,
        #[arg(long, default_value_t = 90)]
        height: u32,
    },
    Hash {
        /// Path to the video file
        input: String,
    },
    Keyframes {
    #[arg(short, long, help = "Path to the input video file")]
    input: String,
},
    /// Placeholder for other tasks
    Clean,
}