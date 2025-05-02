use anyhow::{Context, Result};
use tera::Tera;
use walkdir::WalkDir;

pub fn initialize(dir: &str) -> Result<Tera> {
    let mut tera = Tera::new(format!("{}/**/*", dir).as_str())
        .with_context(|| format!("Failed to initialize Tera with template directory {}", dir))?;

    let template_file_path_and_names: Vec<(String, Option<String>)> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|dir_entry| {
            dir_entry
                .ok()
                // allow symlinks?
                .filter(|dir_entry| dir_entry.file_type().is_file())
                .and_then(|dir_entry| {
                    let path = dir_entry.path().to_str();
                    let file_name_tera = dir_entry
                        .file_name()
                        .to_os_string()
                        .into_string()
                        .ok()
                        .filter(|file_name| file_name.to_ascii_lowercase().ends_with(".tera"));
                    match (path, file_name_tera) {
                        (Some(path), Some(file_name_tera)) => {
                            Some((path.to_owned(), file_name_tera))
                        }
                        _ => None,
                    }
                })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(path, file_name)| {
            (
                path.to_owned(),
                Some(file_name_to_template_name(&file_name).clone()).clone(),
            )
        })
        .collect::<Vec<_>>();

    tera.add_template_files(template_file_path_and_names)?;
    let tera = tera;

    Ok(tera)
}

fn file_name_to_template_name(file_name: &str) -> String {
    file_name
        .rsplitn(3, '.')
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .next()
        .map(|f| f.to_owned())
        .unwrap_or(file_name.to_owned())
}
