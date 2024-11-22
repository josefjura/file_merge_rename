use std::{
    env::{self, set_current_dir},
    fs::File,
    path::Path,
    process::{Command, Stdio},
};

fn main() {
    println!("Hello, world!");
}

pub struct OuterConfig {
    pub repository_path: String,
    pub source_branch: String,
    pub target_branch: String,
    pub remote_name: String,
    pub target_folder: String,
    pub source_folder: String,
    pub version: Option<String>,
}

pub struct InnerConfig {
    pub latest_name: String,
    pub target_folder: String,
    pub source_folder: String,
    pub version: Option<String>,
}

fn find_last(config: OuterConfig) -> String {
    set_current_dir(&config.repository_path).unwrap();
    Command::new("git")
        .arg("fetch")
        .arg(&config.remote_name)
        .arg(format!("{}:{}", config.target_branch, config.target_branch))
        .spawn()
        .expect("Can't fetch target branch");

    let lstree = Command::new("git")
        .arg("ls-tree")
        .arg("-r")
        .arg(config.target_branch)
        .arg("--name-only")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute git command");

    let output = Command::new("grep")
        .arg(&config.target_folder)
        .stdin(lstree.stdout.unwrap())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute grep command");

    "Database/Migrates/db 35/db 35.9/V20241121.01__T023-4884_T023-4883_TRP_Validity_task.sql"
        .to_string()
}

fn process(config: InnerConfig) {}
