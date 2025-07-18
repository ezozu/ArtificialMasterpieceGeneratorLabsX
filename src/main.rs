// src/main.rs
/*
 * Main executable for ArtificialMasterpieceGeneratorLabsX
 */

use clap::Parser;
use artificialmasterpiecegeneratorlabsx::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialMasterpieceGeneratorLabsX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
