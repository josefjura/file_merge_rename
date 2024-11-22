use std::{
    env::set_current_dir,
    fmt::Debug,
    io::{Error, ErrorKind},
    ops::Deref,
    process::{Command, Stdio},
};

fn main() {
    println!("Hello, world!");
}

pub struct OuterConfig {
    pub repository_path: String,
    pub target_branch: String,
    pub target_folder: String,
}

pub struct InnerConfig {
    pub repository_path: String,
    pub source_branch: String,
    pub target_branch: String,
    pub remote_name: String,
    pub target_folder: String,
    pub source_folder: String,
    pub version: Option<String>,
    pub extension: Option<String>,
}

fn fetch_target_folder(
    repository_path: &str,
    remote_name: &str,
    target_branch: &str,
) -> Result<(), Error> {
    set_current_dir(repository_path).unwrap();

    Command::new("git")
        .arg("fetch")
        .arg(format!(
            "{} {}:{}",
            remote_name, target_branch, target_branch
        ))
        .spawn()?;

    Ok(())
}

fn parse_script(file_name: &str, target_folder: &str) {
    let parts: Vec<_> = file_name.split(target_folder).collect();

    let parts: Vec<_> = parts[1].split('/').collect();

    let version = parts[1];
    let script = parts[2];

    println!("Version: {}", version);
    println!("Script: {}", script);
}

fn find_last_script(config: OuterConfig) -> Result<String, Error> {
    set_current_dir(&config.repository_path).unwrap();

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
        .output()
        .expect("Failed to execute grep command");

    let output = String::from_utf8_lossy(&output.stdout).to_string();

    let lines: Vec<_> = output.lines().map(|line| line.to_string()).collect();

    let result = lines.into_iter().last();

    result.ok_or(Error::new(ErrorKind::Other, "No files found"))
}

fn process(config: InnerConfig) {}

#[cfg(test)]
mod tests {
    use crate::OuterConfig;

    #[test]
    fn test_find_last() {
        let config = OuterConfig {
            repository_path: "./".to_string(),
            target_branch: "develop".to_string(),
            target_folder: "sql/migrations".to_string(),
        };

        let result = crate::find_last_script(config);

        match result {
            Ok(file) => assert_eq!("sql/migrations/1/2_test.sql", file),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn parse_script() {
        let config = OuterConfig {
            repository_path: "./".to_string(),
            target_branch: "develop".to_string(),
            target_folder: "sql/migrations".to_string(),
        };

        let result = crate::parse_script("sql/migrations/1/2_test.sql", &config.target_folder);
    }
}
