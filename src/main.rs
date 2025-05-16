mod process;
mod template;

use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufWriter};

/// Format a source file using templates
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The source file to process
    #[arg(short, long)]
    source: String,
    /// The output file or stdout (-)
    #[arg(short, long, default_value = "-")]
    output: String,
    /// The directory containing the Tera templates used for formatting
    #[arg(short, long)]
    template_directory: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    sensible_env_logger::init!();

    // TODO: evaluate r/w open strategy
    let f = File::open(args.source)?;
    let reader = BufReader::new(f);
    let tera = template::initialize(&args.template_directory)?;
    let process_patterns = process::init_patterns()?;
    if args.output == "-" {
        let mut writer = BufWriter::new(std::io::stdout());
        process::process(&process_patterns, &tera, reader, &mut writer)?;
    } else {
        let f = File::open(args.output)?;
        let mut writer = BufWriter::new(f);
        process::process(&process_patterns, &tera, reader, &mut writer)?;
    }

    Ok(())
}
