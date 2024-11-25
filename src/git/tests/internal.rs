use std::path::Path;

use git2::{build::CheckoutBuilder, Repository, Signature};
use tempdir::TempDir;

use crate::git::find_files_in_branch;

pub fn create_test_repository(
    master_files: Vec<String>,
    develop_files: Vec<String>,
) -> (TempDir, Repository) {
    // Create a temporary directory
    let temp_dir = TempDir::new("test-repo").expect("Failed to create temporary directory");

    // Initialize the repository
    let repo = Repository::init(temp_dir.path()).expect("Failed to initialize repository");

    // Step 1: Create and commit files for the master branch
    {
        let mut index = repo.index().expect("Failed to get repository index");
        for file in &master_files {
            let file_path = temp_dir.path().join(file);
            std::fs::create_dir_all(&file_path.parent().unwrap())
                .expect("Failed to write directories");
            std::fs::write(&file_path, "content").expect("Failed to write file");
            index
                .add_path(Path::new(file))
                .expect("Failed to add file to index");
        }
        index.write().expect("Failed to write index");

        // Write tree and create initial commit
        let tree_id = index.write_tree().expect("Failed to write tree");
        let tree = repo.find_tree(tree_id).expect("Failed to find tree");
        create_commit(&repo, "Initial commit on master", tree, &[]);
    }

    // Step 2: Create and checkout the develop branch
    {
        let master_commit = repo.head().unwrap().peel_to_commit().unwrap();
        repo.branch("develop", &master_commit, false)
            .expect("Failed to create develop branch");

        // Checkout the develop branch
        let mut checkout_opts = CheckoutBuilder::new();
        repo.set_head("refs/heads/develop")
            .expect("Failed to set HEAD to develop branch");
        repo.checkout_head(Some(&mut checkout_opts))
            .expect("Failed to checkout develop branch");
    }

    // Step 3: Create and commit files for the develop branch
    {
        let mut index = repo.index().expect("Failed to get repository index");
        for file in &develop_files {
            let file_path = temp_dir.path().join(file);
            std::fs::create_dir_all(&file_path.parent().unwrap())
                .expect("Failed to write directories");
            std::fs::write(&file_path, "content").expect("Failed to write file");
            index
                .add_path(Path::new(file))
                .expect("Failed to add file to index");
        }
        index.write().expect("Failed to write index");

        // Write tree and create commit on develop branch
        let tree_id = index.write_tree().expect("Failed to write tree");
        let tree = repo.find_tree(tree_id).expect("Failed to find tree");
        let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
        create_commit(&repo, "Commit on develop branch", tree, &[&parent_commit]);
    }

    println!(
        "Temporary test repository created at: {:?}",
        temp_dir.path()
    );

    (temp_dir, repo)
}

fn create_commit(repo: &Repository, message: &str, tree: git2::Tree, parents: &[&git2::Commit]) {
    let sig = Signature::now("script_rename", "script_rename@example.com")
        .expect("Failed to create signature");
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, parents)
        .expect("Failed to create commit");
}

#[test]
fn basic_last_test() {
    let (dir, _) = create_test_repository(
        vec![
            "dir/test1.sql".to_string(),
            "dir/test2.sql".to_string(),
            "dir/test3.sql".to_string(),
            "test4.json".to_string(),
        ],
        vec!["new/test.sql".to_string()],
    );

    let repo_path = dir.path();
    let branch = "master";
    let extension = "sql";

    let result = find_files_in_branch(repo_path, branch, Some(extension), None).unwrap();
    assert!(result.len() > 1);
    assert_eq!(*result.last().unwrap(), "dir/test3.sql".to_string());
    dir.close().unwrap();
}

#[test]
fn basic_test_order() {
    let (dir, _) = create_test_repository(
        vec![
            "dir/test3.sql".to_string(),
            "dir/test2.sql".to_string(),
            "dir/test1.sql".to_string(),
            "test4.json".to_string(),
        ],
        vec!["new/test.sql".to_string()],
    );

    let repo_path = dir.path();
    let branch = "master";
    let extension = "sql";

    let result = find_files_in_branch(repo_path, branch, Some(extension), None).unwrap();
    assert_eq!(3, result.len());
    assert_eq!(*result.last().unwrap(), "dir/test3.sql".to_string());
    dir.close().unwrap();
}

#[test]
fn basic_test_develop() {
    let (dir, _) = create_test_repository(
        vec![
            "dir/test1.sql".to_string(),
            "dir/test2.sql".to_string(),
            "dir/test3.sql".to_string(),
            "test4.json".to_string(),
        ],
        vec!["new/test.sql".to_string()],
    );

    let repo_path = dir.path();
    let branch = "develop";
    let extension = "sql";

    let result = find_files_in_branch(repo_path, branch, Some(extension), None).unwrap();
    assert_eq!(4, result.len());
    assert_eq!(*result.last().unwrap(), "new/test.sql".to_string());
}

#[test]
fn basic_test_develop_folder() {
    let (dir, _) = create_test_repository(
        vec![
            "sql/dir/test1.sql".to_string(),
            "sql/dir/test2.sql".to_string(),
            "sql/dir/test3.sql".to_string(),
            "test4.json".to_string(),
        ],
        vec!["new/test.sql".to_string()],
    );

    let repo_path = dir.path();
    let branch = "develop";
    let extension = "sql";

    let result =
        find_files_in_branch(repo_path, branch, Some(extension), Some(Path::new("new"))).unwrap();
    assert_eq!(1, result.len());
    assert_eq!(*result.last().unwrap(), "new/test.sql".to_string());
}

#[test]
fn basic_test_deep_folder() {
    let (dir, _) = create_test_repository(
        vec![
            "dir/sql/test3.sql".to_string(),
            "dir/sql/test2.sql".to_string(),
            "dir/sql/test1.sql".to_string(),
            "test4.json".to_string(),
        ],
        vec!["dir/new/test.sql".to_string()],
    );

    let repo_path = dir.path();
    let branch = "develop";
    let extension = "sql";

    let result = find_files_in_branch(
        repo_path,
        branch,
        Some(extension),
        Some(Path::new("dir/new")),
    )
    .unwrap();
    assert_eq!(1, result.len());
    assert_eq!(*result.last().unwrap(), "dir/new/test.sql".to_string());
    dir.close().unwrap();
}

// #[test]
// fn basic_test_no_extension() {
//     let (dir, _) = create_test_repository(
//         vec![
//             "dir/test1.sql".to_string(),
//             "dir/test2.sql".to_string(),
//             "dir/test3.sql".to_string(),
//             "test4.json".to_string(),
//         ],
//         vec!["new/test.sql".to_string()],
//     );

//     let repo_path = dir.path();
//     let branch = "master";

//     let result = find_files_in_branch(repo_path, branch, None).unwrap();
//     assert_eq!(result, Some("test4.json".to_string()));
// }
