use anyhow::{Context, Result, anyhow};

use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub fn process<I, O>(reader: BufReader<I>, writer: &BufWriter<O>) -> Result<()>
where
    I: Sized + Read,
    O: ?Sized + Write,
{
    for line in reader.lines() {
        let line = line.with_context(|| "I/O error while reading input")?;
        if !line.trim().is_empty() {
            // TODO: provide debug information
            // TODO: test that debug information is printed
            let template_name = line
                .splitn(1, " ")
                .into_iter()
                .next()
                .ok_or(anyhow!("Failed to parse line"))?;
            println!("{}", template_name);
        }
    }

    Ok(())
}
