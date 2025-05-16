mod line;

use anyhow::{Context, Result, anyhow};
use log::LevelFilter;
use regex::Regex;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use tera::Tera;

enum Template<'a> {
    Cached(CachedOutput<'a>),
    Template(TemplateCall),
}

#[derive(Clone, Debug)]
struct CachedOutput<'a> {
    name: &'a str,
    output: String,
}

struct TemplateCall {
    template: String,
    body: Option<String>, // TODO: variables
}

// TODO: pattern to extract whitespace from start and finish of line
//   use in process_line to trim string
//   then, test process_line and incorporate in process

pub struct Patterns {
    pub blank: Regex,
    pub comment: Regex,
    pub template: Regex,
}

pub fn init_patterns() -> Result<Patterns> {
    let blank = Regex::new(r"^\s*$").with_context(|| "Unexpected error creating 'blank' regex")?;
    let comment =
        Regex::new(r"^\s*#.*$").with_context(|| "Unexpected error creating 'comment' regex")?;
    let template = Regex::new(r"^\s*(?<template>[a-zA-Z-]+)(\s+(?<body>.+)?)?\s*$")
        .with_context(|| "Unexpected error creating 'template' regex")?;
    Ok(Patterns {
        blank,
        comment,
        template,
    })
}

// TODO: Arc / multithread?
pub fn process<I, O>(
    patterns: &Patterns,
    tera: &Tera,
    reader: BufReader<I>,
    writer: &mut BufWriter<O>,
) -> Result<()>
where
    I: Sized + Read,
    O: Write,
{
    // TODO: document
    // TODO: add flag
    // TODO: similar flag & pattern for rendering comments
    let newline_output: Option<CachedOutput> =
        match tera.render("_newline", &tera::Context::new()) {
            Ok(s) => Ok(Some(CachedOutput {
                name: "newline",
                output: s,
            })),
            Err(err) => match &err.kind {
                tera::ErrorKind::TemplateNotFound(_) => Ok(None),
                _ => Err(err)
                    .with_context(|| "A newline template was found, but could not be rendered"),
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
        // TODO: provide debug information
        // TODO: test that debug information is printed
        let template = (if patterns.blank.is_match(&*line) {
            Ok(newline_output
                .clone()
                .map(|newline_output| Template::Cached(newline_output)))
        } else if patterns.comment.is_match(&*line) {
            Ok(None)
        } else if let Some(template_call) = process_line(patterns, &line) {
            Ok(Some(Template::Template(template_call)))
        } else {
            // TODO: debug information. line number, etc.
            Err(anyhow!(format!("Unable to process line: {}", line)))
        })?;
        match template {
            Some(Template::Cached(cached_output)) => {
                writer
                    .write(cached_output.output.as_ref())
                    .with_context(|| {
                        format!(
                            "Failed to render cached output for template {}",
                            cached_output.name
                        )
                    })?;
                Ok(())
            }
            Some(Template::Template(TemplateCall { template, body })) => {
                let mut context = tera::Context::new();
                // TODO: debug information
                let template_str = format!("template {}", template);
                // TODO: variables
                if let Some(body) = body {
                    context
                        .try_insert("body", &body)
                        .with_context(|| format!("Failed to create context for {}", template_str))?
                };
                let output: String = tera
                    .render(&*template, &context)
                    .with_context(|| format!("Failed to render {}", template_str))?;
                writer
                    .write(output.as_ref())
                    .with_context(|| format!("Error writing {}", template_str))?;

                // TODO: extra newlines are being added.
                //   see last line of example PDF.
                match newline_output {
                    Some(ref newline_output) => writer
                        .write(newline_output.output.as_ref())
                        .map(|_| ())
                        .with_context(|| {
                            format!("Failed to render newline after {}", template_str)
                        }),
                    None => anyhow::Ok(()),
                }?;

                anyhow::Ok(())
            }
            None => Ok(()),
        }?
    }

    Ok(())
}

fn process_line(patterns: &Patterns, line: &str) -> Option<TemplateCall> {
    patterns.template.captures(line).and_then(|captures| {
        captures.name("template").map(|template| TemplateCall {
            template: template.as_str().to_string(),
            body: captures.name("body").map(|body| body.as_str().to_string()),
        })
    })
}

#[test]
fn process_line_test() {
    let line = "prereqs images/";
    let patterns = init_patterns().unwrap();
    let result = process_line(&patterns, line);
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(result.template, "prereqs".to_owned());
    assert!(result.body.is_some());
    assert_eq!(result.body.unwrap(), "images/".to_owned())
}

#[test]
fn process_line_without_body_test() {
    let line = "postreqs";
    let patterns = init_patterns().unwrap();
    let result = process_line(&patterns, line);
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(result.template, "postreqs".to_owned());
    assert!(result.body.is_none());
}
