use anyhow::{Context, Result};
use log::LevelFilter;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use tera::Tera;

// TODO: Arc / multithread?
pub fn process<I, O>(tera: &Tera, reader: BufReader<I>, writer: &mut BufWriter<O>) -> Result<()>
where
    I: Sized + Read,
    O: Write,
{
    // TODO: document
    // TODO: add flag
    // TODO: similar flag & pattern for rendering comments
    let newline_output: Option<String> = match tera.render("_newline", &tera::Context::new()) {
        Ok(s) => Ok(Some(s)),
        Err(err) => match &err.kind {
            tera::ErrorKind::TemplateNotFound(_) => Ok(None),
            _ => {
                Err(err).with_context(|| "A newline template was found, but could not be rendered")
            }
        },
    }?;

    if log::max_level() <= LevelFilter::Debug {
        let mut template_names = tera
            .get_template_names()
            .filter(|template_name| !template_name.ends_with(".tera"))
            .collect::<Vec<_>>();
        template_names.sort();
        log::debug!(
            "Running with templates:\n - {}",
            template_names.join("\n - ")
        );
    }

    for line in reader.lines() {
        let line = line.with_context(|| "I/O error while reading input")?;
        if !line.trim().is_empty() {
            // TODO: provide debug information
            // TODO: test that debug information is printed
            let mut template_and_string_iterator = line.splitn(2, ' ');
            let template_name = template_and_string_iterator.next();

            let template_and_body: Option<(&str, Option<&str>)> =
                template_name.and_then(|template_name| {
                    let template_name = template_name.trim();
                    if template_name == "" || template_name == "#" {
                        None
                    } else {
                        Some((template_name, template_and_string_iterator.next()))
                    }
                });

            match template_and_body {
                Some((template, body)) => {
                    let mut context = tera::Context::new();
                    let template_with_body_str =
                        format!("template {} with body {}", template, body.unwrap_or(""));
                    // TODO: variables
                    if let Some(body) = body {
                        context.try_insert("body", body).with_context(|| {
                            format!("Failed to create context for {}", template_with_body_str)
                        })?
                    };
                    let output: String = tera
                        .render(template, &context)
                        .with_context(|| format!("Failed to render {}", template_with_body_str))?;
                    writer
                        .write(output.as_ref())
                        .with_context(|| format!("Error writing {}", template_with_body_str))?;

                    match newline_output {
                        Some(ref newline_output) => writer
                            .write(newline_output.as_ref())
                            .map(|_| ())
                            .with_context(|| {
                                format!("Failed to render newline after {}", template_with_body_str)
                            }),
                        None => anyhow::Ok(()),
                    }?;

                    anyhow::Ok(())
                }
                None => Ok(()),
            }?
        }
    }

    Ok(())
}
