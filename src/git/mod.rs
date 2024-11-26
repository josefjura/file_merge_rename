use gix::bstr::ByteSlice; // For handling byte slices in filenames
use gix::objs::tree::EntryKind; // For differentiating between blobs and trees
use gix::prelude::FindExt; // For `find_tree`

use std::path::{Path, PathBuf};

#[cfg(test)]
pub mod tests;

pub fn find_files_in_branch(
    repo_path: &Path,
    branch: &str,
    extension: Option<&str>,
    directory: Option<&Path>,
) -> anyhow::Result<Vec<String>> {
    // Open the repository
    let repo = gix::open(repo_path)?;

    // Resolve the branch to its latest commit
    let mut reference = repo.find_reference(branch)?;
    let commit = reference.peel_to_commit()?;
    let tree_id = commit.tree_id()?;

    // Load the tree object
    let mut buffer = Vec::new(); // Mutable buffer for reuse
    let tree = repo.objects.find_tree(&tree_id, &mut buffer)?;

    // Collect matching files
    let mut matching_files = Vec::new();

    // Start recursive traversal
    traverse_tree(
        &tree,
        extension,
        directory.map(PathBuf::from),
        String::new(),
        &mut matching_files,
        &repo,
    )?;

    Ok(matching_files)
}

fn traverse_tree(
    tree: &gix::objs::TreeRef<'_>,
    extension: Option<&str>,
    directory: Option<PathBuf>,
    current_path: String,
    matching_files: &mut Vec<String>,
    repo: &gix::Repository,
) -> anyhow::Result<()> {
    for entry in &tree.entries {
        // Compute the full path for the current entry
        let entry_path = if current_path.is_empty() {
            entry.filename.to_str_lossy().to_string()
        } else {
            format!("{}/{}", current_path, entry.filename.to_str_lossy())
        };

        match entry.mode.kind() {
            EntryKind::Tree => {
                // Always recurse into subtrees
                let mut buffer = Vec::new();
                let subtree = repo.objects.find_tree(&entry.oid, &mut buffer)?;
                traverse_tree(
                    &subtree,
                    extension,
                    directory.clone(),
                    entry_path,
                    matching_files,
                    repo,
                )?;
            }
            EntryKind::Blob | EntryKind::BlobExecutable => {
                // Check if the file matches the directory filter (if specified)
                if let Some(ref dir_filter) = directory {
                    let entry_path_as_path = Path::new(&entry_path);
                    if !entry_path_as_path.starts_with(dir_filter) {
                        continue;
                    }
                }

                // Check if the file matches the extension
                if Path::new(&entry_path).extension().map_or(false, |ext| {
                    extension.is_none() || ext == extension.unwrap()
                }) {
                    matching_files.push(entry_path);
                }
            }
            _ => {} // Ignore other types (e.g., symbolic links)
        }
    }
    Ok(())
}
