mod process;

use anyhow::Result;
use clap::Parser;

/// Format a source file using templates
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The source file to process
    #[arg(short, long)]
    source: String,
    /// The output file or stdout, default stdout
    #[arg(short, long, default_value="-")]
    output: String,
    /// The directory containing the Tera templates used for formatting
    #[arg(short, long)]
    template_directory: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // file from source
    // BufReader from file
    // BufWriter for output
    // process

    Ok(())
}
