use crate::error::ParseScriptError;
use crate::{config::PathConfig, git::find_files_in_branch};
use crate::{format_target_name::format_target_name, parse_path};

use regex::Regex;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

pub struct Configuration {
    pub repo_path: PathBuf,
    pub root_directory: String,
    pub target_branch: String,
    pub source_branch: String,
    pub extension_filter: Option<String>,
    pub target_directory_filter: Option<PathBuf>,
    pub source_directory_filter: Option<PathBuf>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            repo_path: PathBuf::new(),
            root_directory: String::new(),
            target_branch: String::new(),
            source_branch: String::new(),
            extension_filter: None,
            target_directory_filter: None,
            source_directory_filter: None,
        }
    }
}

pub fn process(config: Configuration) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let target_files = find_files_in_branch(
        config.repo_path.as_path(),
        &config.target_branch,
        config.extension_filter.as_deref(),
        config.target_directory_filter.as_deref(),
    )
    .unwrap();

    let mut paths: Vec<(String, String)> = vec![];

    if let Some(last_target_file) = target_files.last() {
        let source = parse_path(&last_target_file, &config.root_directory).unwrap();

        let source_files = find_files_in_branch(
            config.repo_path.as_path(),
            &config.source_branch,
            config.extension_filter.as_deref(),
            config.source_directory_filter.as_deref(),
        )
        .unwrap();

        let (date, index) = get_code(&last_target_file);

        for (i, source_name) in source_files.iter().enumerate() {
            let real_index = index + i as i64 + 1;
            let target_name = format_target_name(
                Path::new(source.folder.as_str()),
                source.version.as_ref().unwrap(),
                format!("V{}.{:02}", date, real_index).as_str(),
                source_name,
            );

            paths.push((source_name.clone(), target_name.clone()));
        }
    }

    Ok(paths)
}

fn get_code(filename: &str) -> (String, i64) {
    let some = Regex::new(r#"V(\d{8})\.(\d{2})__.+"#).unwrap();

    let caps = some.captures(filename).unwrap();

    let date = caps.get(1).unwrap().as_str();
    let index = caps.get(2).unwrap().as_str();

    (date.to_string(), index.parse::<i64>().unwrap())
}
