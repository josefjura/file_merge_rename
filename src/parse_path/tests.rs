use crate::{config::PathConfig, error::ParseScriptError};

use super::parse_path;

#[test]
fn ideal() {
    let result = parse_path("target/dir/test.sql", "target");
    assert_eq!(
        result,
        Ok(PathConfig::new(
            "target".to_string(),
            Some("dir".to_string()),
            "test.sql".to_string()
        ))
    );
}

#[test]
fn ideal_target_slash1() {
    let result = parse_path("target/dir/test.sql", "target/");
    assert_eq!(
        result,
        Ok(PathConfig::new(
            "target".to_string(),
            Some("dir".to_string()),
            "test.sql".to_string()
        ))
    );
}

#[test]
fn ideal_target_slash2() {
    let result = parse_path("target/dir/test.sql", "./target");
    assert_eq!(
        result,
        Ok(PathConfig::new(
            "target".to_string(),
            Some("dir".to_string()),
            "test.sql".to_string()
        ))
    );
}

#[test]
fn ideal_file_slash1() {
    let result = parse_path("./target/dir/test.sql", "target");
    assert_eq!(
        result,
        Ok(PathConfig::new(
            "target".to_string(),
            Some("dir".to_string()),
            "test.sql".to_string()
        ))
    );
}

#[test]
fn no_version() {
    let result = parse_path("target/test.sql", "target/");
    assert_eq!(
        result,
        Ok(PathConfig::new(
            "target".to_string(),
            None,
            "test.sql".to_string()
        ))
    );
}

#[test]
fn no_version_target_slash() {
    let result = parse_path("target/test.sql", "target");
    assert_eq!(
        result,
        Ok(PathConfig::new(
            "target".to_string(),
            None,
            "test.sql".to_string()
        ))
    );
}

#[test]
fn no_version_file_slash() {
    let result = parse_path("./target/test.sql", "target");
    assert_eq!(
        result,
        Ok(PathConfig::new(
            "target".to_string(),
            None,
            "test.sql".to_string()
        ))
    );
}

#[test]
fn deep_version() {
    let result = parse_path("target/1/1.1/1.1.1/test.sql", "target/");
    assert_eq!(
        result,
        Ok(PathConfig::new(
            "target".to_string(),
            Some("1/1.1/1.1.1".to_string()),
            "test.sql".to_string()
        ))
    );
}

#[test]
fn deep_version_target_slash() {
    let result = parse_path("target/1/1.1/1.1.1/test.sql", "target");
    assert_eq!(
        result,
        Ok(PathConfig::new(
            "target".to_string(),
            Some("1/1.1/1.1.1".to_string()),
            "test.sql".to_string()
        ))
    );
}

#[test]
fn deep_version_file_slash() {
    let result = parse_path("./target/1/1.1/1.1.1/test.sql", "target");
    assert_eq!(
        result,
        Ok(PathConfig::new(
            "target".to_string(),
            Some("1/1.1/1.1.1".to_string()),
            "test.sql".to_string()
        ))
    );
}

#[test]
fn bad_target_folder() {
    let result = parse_path("target/dir/test.sql", "target2");
    assert_eq!(result, Err(ParseScriptError::BadTargetFolder));
}
