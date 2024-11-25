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
    let mut config = Configuration::default();

    config.repo_path = PathBuf::from("/mnt/c/Users/josef/source/eurowag/Aequitas");
    config.root_directory = "Database/Migrates".to_string();
    config.target_branch = "develop".to_string();
    config.source_branch = "feature/T023-5102".to_string();
    config.extension_filter = Some("sql".to_string());
    config.target_directory_filter = None;
    config.source_directory_filter = Some(PathBuf::from("Database/Migrates/new"));

    let result = process(config).unwrap();

    for (source, target) in result {
        println!("{} -> {}", source, target);
    }
}
