use std::path::{Component, Path, PathBuf};

use crate::config::PathConfig;
use crate::error::ParseScriptError;

#[cfg(test)]
mod tests;

fn normalize_path(path: &str) -> PathBuf {
    Path::new(path)
        .components()
        .filter(|comp| *comp != Component::CurDir) // Remove `./`
        .collect()
}

pub fn parse_path(file_path: &str, root: &str) -> Result<PathConfig, ParseScriptError> {
    // Normalize both the file path and target folder
    let root = normalize_path(root);
    let file_path = normalize_path(file_path);

    // Ensure the file path contains the target folder as a prefix
    let stripped_path = file_path
        .strip_prefix(&root)
        .map_err(|_| ParseScriptError::BadTargetFolder)?;

    // Extract file name
    let file_name = stripped_path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .ok_or(ParseScriptError::NoFileName)?;

    // Extract the remaining path (excluding file name)
    let remaining_path = stripped_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(PathBuf::new);

    // Extract the version as the full remaining path, if any
    let version = if remaining_path.components().count() > 0 {
        Some(remaining_path.to_string_lossy().to_string())
    } else {
        None
    };

    // Build the result
    Ok(PathConfig::new(
        root.to_string_lossy().to_string(),
        version,
        file_name.to_string(),
    ))
}
