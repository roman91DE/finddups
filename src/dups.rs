use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

pub fn list_dir(path: &PathBuf, max_depth: usize, include_hidden: bool) -> Vec<PathBuf> {
    fn is_hidden(path: &PathBuf) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with('.'))
            .unwrap_or(false)
    }
    fn inner(
        path: &PathBuf,
        depth: usize,
        max_depth: usize,
        include_hidden: bool,
        files: &mut Vec<PathBuf>,
    ) {
        if max_depth > 0 && depth > max_depth {
            return;
        }
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if !include_hidden && is_hidden(&path) {
                    continue;
                }
                if path.is_file() {
                    files.push(path);
                } else if path.is_dir() {
                    inner(&path, depth + 1, max_depth, include_hidden, files);
                }
            }
        }
    }
    let mut files = Vec::new();
    inner(path, 1, max_depth, include_hidden, &mut files);
    files
}

/// Finds duplicate files in the given directory tree.
pub fn find_duplicates(
    root: &PathBuf,
    max_depth: usize,
    include_hidden: bool,
) -> HashMap<String, Vec<PathBuf>> {
    let files = list_dir(root, max_depth, include_hidden);
    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    // First group by file size
    for file in files {
        if let Ok(metadata) = fs::metadata(&file) {
            let size = metadata.len();
            size_map.entry(size).or_insert_with(Vec::new).push(file);
        }
    }
    // Now, for each group with more than one file, hash contents
    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for files in size_map.values().filter(|v| v.len() > 1) {
        let mut hashes: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for file in files {
            if let Ok(mut f) = fs::File::open(file) {
                let mut hasher = Sha256::new();
                let mut buf = [0u8; 8192];
                loop {
                    let n = match f.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };
                    hasher.update(&buf[..n]);
                }
                let hash = format!("{:x}", hasher.finalize());
                hashes
                    .entry(hash)
                    .or_insert_with(Vec::new)
                    .push(file.clone());
            }
        }
        for (hash, group) in hashes {
            if group.len() > 1 {
                hash_map.insert(hash, group);
            }
        }
    }
    hash_map
}

/// Deletes the given files.
pub fn delete_files(files: &[PathBuf]) -> std::io::Result<()> {
    for file in files {
        fs::remove_file(file)?;
    }
    Ok(())
}
