use std::path::PathBuf;

use clap::{Args, Parser};

use crate::process::Configuration;

#[derive(Parser, Debug)]
#[command(
    name = "Script Name Rename",
    version,
    about,
    long_about = "Git merging utility for serialized files"
)]
pub struct AppArgs {
    #[command(flatten)]
    pub opts: Settings,
}

#[derive(Debug, Args)]
pub struct Settings {
    /// Path to the repository (defaults to '.')
    #[arg(long = "repository", short = 'r')]
    pub repo_path: Option<String>,
    /// Path to script folder inside the repository
    #[arg(long = "root-dir", short = 'd')]
    pub root_dir: String,

    /// Path to script folder inside the repository
    #[arg(long = "target", short = 't')]
    pub target_branch: String,
    /// Path to script folder inside the repository
    #[arg(long = "source", short = 's')]
    pub source_branch: String,

    /// Extension filter for the file search
    #[arg(long = "ext", short = 'e')]
    pub extension: String,

    /// Path to script folder inside the repository
    #[arg(long = "target-filter")]
    pub target_directory_filter: Option<String>,
    /// Path to script folder inside the repository
    #[arg(long = "source-filter")]
    pub source_directory_filter: Option<String>,
}

impl From<AppArgs> for Configuration {
    fn from(args: AppArgs) -> Self {
        Configuration {
            repo_path: args
                .opts
                .repo_path
                .unwrap_or_else(|| ".".to_string())
                .into(),
            root_directory: args.opts.root_dir,
            target_branch: args.opts.target_branch,
            source_branch: args.opts.source_branch,
            extension_filter: Some(args.opts.extension),
            target_directory_filter: args.opts.target_directory_filter.map(PathBuf::from),
            source_directory_filter: args.opts.source_directory_filter.map(PathBuf::from),
        }
    }
}
