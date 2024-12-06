use crate::git::find_files_in_branch;
use crate::{format_target_name::format_target_name, parse_path};

use anyhow::Context;
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

pub fn process(
    config: Configuration,
    today: chrono::NaiveDate,
) -> anyhow::Result<Vec<(String, String)>> {
    let target_files = find_files_in_branch(
        config.repo_path.as_path(),
        &config.target_branch,
        config.extension_filter.as_deref(),
        config.target_directory_filter.as_deref(),
    )
    .with_context(|| format!("Can't open target branch: {0}", &config.target_branch))?;

    let mut paths: Vec<(String, String)> = vec![];

    if let Some(last_target_file) = target_files.last() {
        println!("Last script: {:?}", last_target_file);
        let source = parse_path(last_target_file, &config.root_directory)?;

        println!(
            "Looking for new source files in {:?}",
            &config.source_branch,
        );

        let source_files = find_files_in_branch(
            config.repo_path.as_path(),
            &config.source_branch,
            config.extension_filter.as_deref(),
            config.source_directory_filter.as_deref(),
        )
        .with_context(|| format!("Can't open source branch: {0}", &config.source_branch))?;

        println!("Found {:?} files in source branch", source_files.len());
        let (date, index) = get_code(last_target_file, today)?;

        for (i, source_name) in source_files.iter().enumerate() {
            println!("Processing file: {:?}", source_name);
            let real_index = index + i as i64 + 1;
            let target_name = format_target_name(
                Path::new(source.folder.as_str()),
                source.version.as_deref(),
                format!("V{}.{:02}", date, real_index).as_str(),
                source_name,
            );

            paths.push((source_name.clone(), target_name.clone()));
        }
    }

    Ok(paths)
}

fn get_code(filename: &str, today: chrono::NaiveDate) -> anyhow::Result<(String, i64)> {
    let some = Regex::new(r#"V(\d{8})\.(\d{2})__.+"#).unwrap();

    let caps = some
        .captures(filename)
        .with_context(|| format!("Can't extract the date and index from: {:?}", filename))?;

    let date = caps.get(1).unwrap().as_str();
    let index = caps.get(2).unwrap().as_str();

    let today_code = today.format("%Y%m%d").to_string();

    let date_parsed = chrono::NaiveDate::parse_from_str(date, "%Y%m%d").unwrap();

    let result = if today > date_parsed {
        (today_code, 0)
    } else {
        (date.to_string(), index.parse::<i64>().unwrap())
    };

    Ok(result)
}
