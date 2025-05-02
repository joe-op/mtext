use anyhow::{Context, Result, anyhow};

use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub fn process<I, O>(reader: BufReader<I>, writer: &BufWriter<O>) -> Result<()>
where
    I: Sized + Read,
    O: Write,
{
    for line in reader.lines() {
        let line = line.with_context(|| "I/O error while reading input")?;
        if !line.trim().is_empty() {
            // TODO: provide debug information
            // TODO: test that debug information is printed
            let mut template_and_string_iterator = line.splitn(2, ' ');
            let template_name = template_and_string_iterator.next();

            let template_and_body: Option<(&str, &str)> = template_name.and_then(|template_name| {
                let template_name = template_name.trim();
                if template_name == "" || template_name == "#" {
                    None
                } else {
                    template_and_string_iterator
                        .next()
                        .map(|body| (template_name, body))
                }
            });

            match template_and_body {
                Some((template, body)) => println!("|{}| {}", template, body),
                None => (),
            }

            // load template
            // format using template
            // write
        }
    }

    Ok(())
}
