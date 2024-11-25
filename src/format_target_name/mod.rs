use regex::Regex;
use std::path::Path;

#[cfg(test)]
mod tests;

pub fn format_target_name(
    root: &Path,
    version_path: &str,
    order_code: &str,
    filename: &str,
) -> String {
    let file_name = Path::new(filename)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let regex = Regex::new(r#"^\d\d_{2,}"#).unwrap();

    let file_name = regex.replace(&file_name, "");

    Path::join(root, &version_path)
        .join(format!("{}__{}", order_code, file_name))
        .to_string_lossy()
        .to_string()
}

#[test]
fn format_source_move() {
    let root = Path::new("/mnt/c/Users/josef/source/eurowag/Aequitas");
    let version_path = "Database/Migrates/new";
    let order_code = "V20210802.01";
    let filename = "Database/Migrates/new/01__test.sql";

    let result = format_target_name(root, version_path, order_code, filename);

    assert_eq!(
        result,
        "/mnt/c/Users/josef/source/eurowag/Aequitas/Database/Migrates/new/V20210802.01__test.sql"
    );
}

#[test]
fn format_source_move_no_source_order() {
    let root = Path::new("/mnt/c/Users/josef/source/eurowag/Aequitas");
    let version_path = "Database/Migrates/new";
    let order_code = "V20210802.01";
    let filename = "Database/Migrates/new/test.sql";

    let result = format_target_name(root, version_path, order_code, filename);

    assert_eq!(
        result,
        "/mnt/c/Users/josef/source/eurowag/Aequitas/Database/Migrates/new/V20210802.01__test.sql"
    );
}

#[test]
fn format_source_move_version() {
    let root = Path::new("/mnt/c/Users/josef/source/eurowag/Aequitas");
    let version_path = "";
    let order_code = "V20210802.01";
    let filename = "Database/Migrates/new/01__test.sql";

    let result = format_target_name(root, version_path, order_code, filename);

    assert_eq!(
        result, "/mnt/c/Users/josef/source/eurowag/Aequitas/V20210802.01__test.sql",
        "Should return the correct path on ordered file"
    );
    let filename = "Database/Migrates/new/test.sql";
    let result = format_target_name(root, version_path, order_code, filename);

    assert_eq!(
        result, "/mnt/c/Users/josef/source/eurowag/Aequitas/V20210802.01__test.sql",
        "Should return the correct path on unordered file"
    );
}
