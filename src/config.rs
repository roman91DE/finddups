use std::path::PathBuf;

/// Represents the configuration for running the duplicate finder.
pub struct Config {
    pub root_dir: PathBuf,
    pub delete: bool,
}

impl Config {
    pub fn new(root_dir: PathBuf, delete: bool) -> Self {
        Self { root_dir, delete }
    }
}
