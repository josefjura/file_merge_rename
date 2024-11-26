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
