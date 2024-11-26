use crate::config::PathConfig;

use super::parse_path;

#[test]
fn ideal() -> anyhow::Result<()> {
    let result = parse_path("target/dir/test.sql", "target")?;
    assert_eq!(
        result,
        PathConfig::new(
            "target".to_string(),
            Some("dir".to_string()),
            "test.sql".to_string()
        )
    );

    Ok(())
}

#[test]
fn ideal_target_slash1() -> anyhow::Result<()> {
    let result = parse_path("target/dir/test.sql", "target/")?;
    assert_eq!(
        result,
        PathConfig::new(
            "target".to_string(),
            Some("dir".to_string()),
            "test.sql".to_string()
        )
    );

    Ok(())
}

#[test]
fn ideal_target_slash2() -> anyhow::Result<()> {
    let result = parse_path("target/dir/test.sql", "./target")?;
    assert_eq!(
        result,
        PathConfig::new(
            "target".to_string(),
            Some("dir".to_string()),
            "test.sql".to_string()
        )
    );

    Ok(())
}

#[test]
fn ideal_file_slash1() -> anyhow::Result<()> {
    let result = parse_path("./target/dir/test.sql", "target")?;
    assert_eq!(
        result,
        PathConfig::new(
            "target".to_string(),
            Some("dir".to_string()),
            "test.sql".to_string()
        )
    );

    Ok(())
}

#[test]
fn no_version() -> anyhow::Result<()> {
    let result = parse_path("target/test.sql", "target/")?;
    assert_eq!(
        result,
        PathConfig::new("target".to_string(), None, "test.sql".to_string())
    );

    Ok(())
}

#[test]
fn no_version_target_slash() -> anyhow::Result<()> {
    let result = parse_path("target/test.sql", "target")?;
    assert_eq!(
        result,
        PathConfig::new("target".to_string(), None, "test.sql".to_string())
    );

    Ok(())
}

#[test]
fn no_version_file_slash() -> anyhow::Result<()> {
    let result = parse_path("./target/test.sql", "target")?;
    assert_eq!(
        result,
        PathConfig::new("target".to_string(), None, "test.sql".to_string())
    );

    Ok(())
}

#[test]
fn deep_version() -> anyhow::Result<()> {
    let result = parse_path("target/1/1.1/1.1.1/test.sql", "target/")?;
    assert_eq!(
        result,
        PathConfig::new(
            "target".to_string(),
            Some("1/1.1/1.1.1".to_string()),
            "test.sql".to_string()
        )
    );

    Ok(())
}

#[test]
fn deep_version_target_slash() -> anyhow::Result<()> {
    let result = parse_path("target/1/1.1/1.1.1/test.sql", "target")?;
    assert_eq!(
        result,
        PathConfig::new(
            "target".to_string(),
            Some("1/1.1/1.1.1".to_string()),
            "test.sql".to_string()
        )
    );

    Ok(())
}

#[test]
fn deep_version_file_slash() -> anyhow::Result<()> {
    let result = parse_path("./target/1/1.1/1.1.1/test.sql", "target")?;
    assert_eq!(
        result,
        PathConfig::new(
            "target".to_string(),
            Some("1/1.1/1.1.1".to_string()),
            "test.sql".to_string()
        )
    );

    Ok(())
}

#[test]
fn bad_target_folder() {
    let result = parse_path("target/dir/test.sql", "target2");

    assert!(result.is_err());
    let err = result.unwrap_err();

    assert_eq!(
        err.to_string(),
        "Can't find the target folder prefix '\"target2\"' in: \"target/dir/test.sql\""
    );
}
