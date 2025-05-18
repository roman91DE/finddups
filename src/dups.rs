use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn list_dir(path: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            } else if path.is_dir() {
                files.extend(list_dir(&path));
            }
        }
    }
    files
}

/// Finds duplicate files in the given directory tree.
pub fn find_duplicates(root: &PathBuf) -> HashMap<u64, Vec<PathBuf>> {
    // TODO: Implement file traversal and hashing
    HashMap::new()
}

/// Deletes the given files.
pub fn delete_files(files: &[PathBuf]) -> std::io::Result<()> {
    for file in files {
        fs::remove_file(file)?;
    }
    Ok(())
}
