use parse_path::parse_path;
use process::process;
use process::Configuration;
use std::path::PathBuf;

mod config;
mod error;
mod format_target_name;
mod git;
mod parse_path;
mod process;

fn main() {
    let config = Configuration {
        repo_path: PathBuf::from("/mnt/c/Users/josef/source/eurowag/Aequitas"),
        root_directory: "Database/Migrates".to_string(),
        target_branch: "develop".to_string(),
        source_branch: "feature/T023-5105".to_string(),
        extension_filter: Some("sql".to_string()),
        target_directory_filter: None,
        source_directory_filter: Some(PathBuf::from("Database/Migrates/new")),
    };

    let result = process(config, chrono::offset::Local::now().date_naive()).unwrap();

    for (source, target) in result {
        println!("{} -> {}", source, target);
    }
}
