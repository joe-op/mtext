use anyhow::Result;
use clap::Parser;

/// Format a source file using templates
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The source file to process
    #[arg(short, long)]
    source: String,
    /// The directory containing the Tera templates used for formatting
    #[arg(short, long)]
    template_directory: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    Ok(())
}
