use finddups::dups;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a temp dir with a custom structure for deduplication tests.
    pub struct TestDir {
        pub root: tempfile::TempDir,
        pub file1: std::path::PathBuf,
        pub file2: std::path::PathBuf,
        pub subdir: std::path::PathBuf,
        pub subfile: std::path::PathBuf,
        pub dup1: std::path::PathBuf,
        pub dup2: std::path::PathBuf,
    }

    impl TestDir {
        /// Creates a temp dir with files and subdirs, including duplicates.
        pub fn new_with_duplicates() -> Self {
            let root = tempfile::tempdir().unwrap();
            let file1 = root.path().join("file1.txt");
            let file2 = root.path().join("file2.txt");
            let subdir = root.path().join("subdir");
            let subfile = subdir.join("subfile.txt");
            let dup1 = root.path().join("dup1.txt");
            let dup2 = subdir.join("dup2.txt");
            // Create files and subdir
            std::fs::File::create(&file1)
                .unwrap()
                .write_all(b"abc")
                .unwrap();
            std::fs::File::create(&file2)
                .unwrap()
                .write_all(b"def")
                .unwrap();
            std::fs::create_dir(&subdir).unwrap();
            std::fs::File::create(&subfile)
                .unwrap()
                .write_all(b"ghi")
                .unwrap();
            // Create duplicate files (same content)
            std::fs::File::create(&dup1)
                .unwrap()
                .write_all(b"DUPLICATE")
                .unwrap();
            std::fs::File::create(&dup2)
                .unwrap()
                .write_all(b"DUPLICATE")
                .unwrap();
            Self {
                root,
                file1,
                file2,
                subdir,
                subfile,
                dup1,
                dup2,
            }
        }
    }

    #[test]
    fn test_find_duplicates_empty() {
        let dir = tempdir().unwrap();
        let result = dups::find_duplicates(&dir.path().to_path_buf());
        assert!(result.is_empty());
    }

    #[test]
    fn test_delete_files() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testfile.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "hello").unwrap();
        assert!(file_path.exists());
        dups::delete_files(&[file_path.clone()]).unwrap();
        assert!(!file_path.exists());
    }

    #[test]
    fn test_list_dir_empty() {
        let dir = tempdir().unwrap();
        let files = dups::list_dir(&dir.path().to_path_buf());
        assert!(files.is_empty(), "Expected no files in empty dir");
    }

    #[test]
    fn test_list_dir_with_files_and_subdirs() {
        let td = TestDir::new_with_duplicates();
        let mut files = dups::list_dir(&td.root.path().to_path_buf());
        files.sort();
        let mut expected = vec![td.file1, td.file2, td.subfile, td.dup1, td.dup2];
        expected.sort();
        assert_eq!(files, expected, "Files found do not match expected");
    }
}
