use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use tera::Tera;

// TODO: Arc / multithread?
pub fn process<I, O>(tera: &Tera, reader: BufReader<I>, writer: &mut BufWriter<O>) -> Result<()>
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
                Some((template, body)) => {
                    // load template
                    // format using template
                    // write
                    let mut context = tera::Context::new();
                    context.try_insert("body", body).with_context(|| {
                        format!(
                            "Failed to create context for template {} with body {}",
                            template, body
                        )
                    })?;
                    let output: String = tera.render(template, &context).with_context(|| {
                        format!("Failed to render template {} with body {}", template, body)
                    })?;
                    writer.write(output.as_ref()).with_context(|| {
                        format!("Error writing {} with body {}", template, body)
                    })?;
                    anyhow::Ok(())
                }
                None => Ok(()),
            }?
        }
    }

    Ok(())
}
