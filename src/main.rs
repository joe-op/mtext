mod process;

use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufWriter, Stdout, Write};

/// Format a source file using templates
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The source file to process
    #[arg(short, long)]
    source: String,
    /// The output file or stdout, default stdout
    #[arg(short, long, default_value = "-")]
    output: String,
    /// The directory containing the Tera templates used for formatting
    #[arg(short, long)]
    template_directory: String,
}

fn stdout_writer() -> BufWriter<Stdout> {
    BufWriter::new(std::io::stdout())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // TODO: evaluate r/w open strategy
    let f = File::open(args.source)?;
    let reader = BufReader::new(f);
    if args.output == "-" {
        let writer = BufWriter::new(std::io::stdout());
        process::process(reader, &writer)?;
    } else {
        let f = File::open(args.output)?;
        let writer = BufWriter::new(f);
        process::process(reader, &writer);
    }

    Ok(())
}
