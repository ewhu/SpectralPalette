// src/main.rs
/*
 * Main executable for SpectralPalette
 */

use clap::Parser;
use spectralpalette::{Result, run};

#[derive(Parser)]
#[command(version, about = "SpectralPalette - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
