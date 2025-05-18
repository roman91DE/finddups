use std::path::PathBuf;

/// Represents the configuration for running the duplicate finder.
pub struct Config {
    pub root_dir: PathBuf,
    pub delete: bool,
    pub max_depth: usize, // 0 = unlimited
    pub include_hidden: bool,
}

impl Config {
    pub fn new(root_dir: PathBuf, delete: bool, max_depth: usize, include_hidden: bool) -> Self {
        Self {
            root_dir,
            delete,
            max_depth,
            include_hidden,
        }
    }
}
