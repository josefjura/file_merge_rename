use std::path::PathBuf;

use crate::git::tests::internal::create_test_repository;

use super::{process, Configuration};

#[test]
fn test_find_files_in_branch_one_file() {
    let (temp_dir, _) = create_test_repository(
        vec!["db/migrate/1/V20240101.01__file1.txt".to_string()],
        vec!["db/migrate/new/01__file_of_my_life.txt".to_string()],
    );

    let config = Configuration {
        repo_path: temp_dir.path().to_path_buf(),
        root_directory: "db/migrate".to_string(),
        target_branch: "master".to_string(),
        source_branch: "develop".to_string(),
        extension_filter: Some("txt".to_string()),
        target_directory_filter: None,
        source_directory_filter: Some(PathBuf::from("db/migrate/new")),
    };

    let master_files =
        process(config, chrono::NaiveDate::from_ymd_opt(2024, 1, 2).unwrap()).unwrap();

    assert_eq!(master_files.len(), 1);
    assert_eq!(master_files[0].0, "db/migrate/new/01__file_of_my_life.txt");
    assert_eq!(
        master_files[0].1,
        "db/migrate/1/V20240102.01__file_of_my_life.txt"
    );

    temp_dir.close().unwrap();
}

#[test]
fn test_find_files_in_branch_one_file_same_day() {
    let (temp_dir, _) = create_test_repository(
        vec!["db/migrate/1/V20240101.01__file1.txt".to_string()],
        vec!["db/migrate/new/01__file_of_my_life.txt".to_string()],
    );

    let config = Configuration {
        repo_path: temp_dir.path().to_path_buf(),
        root_directory: "db/migrate".to_string(),
        target_branch: "master".to_string(),
        source_branch: "develop".to_string(),
        extension_filter: Some("txt".to_string()),
        target_directory_filter: None,
        source_directory_filter: Some(PathBuf::from("db/migrate/new")),
    };

    let master_files =
        process(config, chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();

    assert_eq!(master_files.len(), 1);
    assert_eq!(master_files[0].0, "db/migrate/new/01__file_of_my_life.txt");
    assert_eq!(
        master_files[0].1,
        "db/migrate/1/V20240101.02__file_of_my_life.txt"
    );

    temp_dir.close().unwrap();
}

#[test]
fn test_find_files_in_branch_two_files_index_off_one() {
    let (temp_dir, _) = create_test_repository(
        vec![
            "db/migrate/1/V20240101.01__file1.txt".to_string(),
            "db/migrate/1/V20240101.02__file2.txt".to_string(),
        ],
        vec![
            "db/migrate/new/01__file3.txt".to_string(),
            "db/migrate/new/02__file4.txt".to_string(),
        ],
    );

    let config = Configuration {
        repo_path: temp_dir.path().to_path_buf(),
        root_directory: "db/migrate".to_string(),
        target_branch: "master".to_string(),
        source_branch: "develop".to_string(),
        extension_filter: Some("txt".to_string()),
        target_directory_filter: None,
        source_directory_filter: Some(PathBuf::from("db/migrate/new")),
    };

    let master_files =
        process(config, chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();

    assert_eq!(master_files.len(), 2);
    assert_eq!(master_files[0].0, "db/migrate/new/01__file3.txt");
    assert_eq!(master_files[0].1, "db/migrate/1/V20240101.03__file3.txt");
    assert_eq!(master_files[1].0, "db/migrate/new/02__file4.txt");
    assert_eq!(master_files[1].1, "db/migrate/1/V20240101.04__file4.txt");

    temp_dir.close().unwrap();
}

#[test]
fn test_find_files_in_branch_bad_code() {
    let (temp_dir, _) = create_test_repository(
        vec![
            "db/migrate/1/V20231231__file1.txt".to_string(),
            "db/migrate/1/V20240101.02__file1.txt".to_string(),
        ],
        vec!["db/migrate/new/01__file_of_my_life.txt".to_string()],
    );

    let config = Configuration {
        repo_path: temp_dir.path().to_path_buf(),
        root_directory: "db/migrate".to_string(),
        target_branch: "master".to_string(),
        source_branch: "develop".to_string(),
        extension_filter: Some("txt".to_string()),
        target_directory_filter: None,
        source_directory_filter: Some(PathBuf::from("db/migrate/new")),
    };

    let master_files =
        process(config, chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();

    assert_eq!(master_files.len(), 1);
    assert_eq!(master_files[0].0, "db/migrate/new/01__file_of_my_life.txt");
    assert_eq!(
        master_files[0].1,
        "db/migrate/1/V20240101.03__file_of_my_life.txt"
    );

    temp_dir.close().unwrap();
}

#[test]
fn test_find_files_in_branch_last_file_bad_code() {
    let (temp_dir, _) = create_test_repository(
        vec!["db/migrate/1/V20240101__file1.txt".to_string()],
        vec!["db/migrate/new/01__file_of_my_life.txt".to_string()],
    );

    let config = Configuration {
        repo_path: temp_dir.path().to_path_buf(),
        root_directory: "db/migrate".to_string(),
        target_branch: "master".to_string(),
        source_branch: "develop".to_string(),
        extension_filter: Some("txt".to_string()),
        target_directory_filter: None,
        source_directory_filter: Some(PathBuf::from("db/migrate/new")),
    };

    let master_files = process(config, chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());

    assert!(master_files.is_err());

    let err = master_files.unwrap_err();
    assert_eq!(
        err.to_string(),
        "Can't extract the date and index from: \"db/migrate/1/V20240101__file1.txt\""
    );

    temp_dir.close().unwrap();
}

#[test]
fn test_find_files_in_branch_non_existing_root() {
    let (temp_dir, _) = create_test_repository(
        vec!["db/migrate/1/V20240101__file1.txt".to_string()],
        vec!["db/migrate/new/01__file_of_my_life.txt".to_string()],
    );

    let config = Configuration {
        repo_path: temp_dir.path().to_path_buf(),
        root_directory: "bad_root".to_string(),
        target_branch: "master".to_string(),
        source_branch: "develop".to_string(),
        extension_filter: Some("txt".to_string()),
        target_directory_filter: None,
        source_directory_filter: Some(PathBuf::from("db/migrate/new")),
    };

    let master_files = process(config, chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());

    assert!(master_files.is_err());

    let err = master_files.unwrap_err();
    assert_eq!(
        err.to_string(),
        "Can't find the target folder prefix '\"bad_root\"' in: \"db/migrate/1/V20240101__file1.txt\""
    );

    temp_dir.close().unwrap();
}

#[test]
fn test_find_files_in_branch_non_existing_target_branch() {
    let (temp_dir, _) = create_test_repository(
        vec!["db/migrate/1/V20240101__file1.txt".to_string()],
        vec!["db/migrate/new/01__file_of_my_life.txt".to_string()],
    );

    let config = Configuration {
        repo_path: temp_dir.path().to_path_buf(),
        root_directory: "db/migrate".to_string(),
        target_branch: "bad_branch".to_string(),
        source_branch: "develop".to_string(),
        extension_filter: Some("txt".to_string()),
        target_directory_filter: None,
        source_directory_filter: Some(PathBuf::from("db/migrate/new")),
    };

    let master_files = process(config, chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());

    assert!(master_files.is_err());

    let err = master_files.unwrap_err();
    assert_eq!(err.to_string(), "Can't open target branch: bad_branch");

    temp_dir.close().unwrap();
}

#[test]
fn test_find_files_in_branch_non_existing_source_branch() {
    let (temp_dir, _) = create_test_repository(
        vec!["db/migrate/1/V20240101__file1.txt".to_string()],
        vec!["db/migrate/new/01__file_of_my_life.txt".to_string()],
    );

    let config = Configuration {
        repo_path: temp_dir.path().to_path_buf(),
        root_directory: "db/migrate".to_string(),
        target_branch: "master".to_string(),
        source_branch: "bad_branch".to_string(),
        extension_filter: Some("txt".to_string()),
        target_directory_filter: None,
        source_directory_filter: Some(PathBuf::from("db/migrate/new")),
    };

    let master_files = process(config, chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());

    assert!(master_files.is_err());

    let err = master_files.unwrap_err();
    assert_eq!(err.to_string(), "Can't open source branch: bad_branch");

    temp_dir.close().unwrap();
}
