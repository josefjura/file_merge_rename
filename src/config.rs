pub struct OuterConfig {
    pub repository_path: String,
    pub target_branch: String,
    pub target_folder: String,
}

impl OuterConfig {
    pub fn default() -> Self {
        Self {
            repository_path: ".".to_string(),
            target_branch: "master".to_string(),
            target_folder: "sql/migrations".to_string(),
        }
    }
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

#[derive(Debug, PartialEq)]
pub struct PathConfig {
    pub folder: String,
    pub version: Option<String>,
    pub file_name: String,
}

impl PathConfig {
    pub fn new(folder: String, version: Option<String>, file_name: String) -> Self {
        Self {
            folder,
            version,
            file_name,
        }
    }
}
