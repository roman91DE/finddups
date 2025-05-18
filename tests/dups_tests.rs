use finddups::dups;
use std::fs::File;
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
        // pub subdir: std::path::PathBuf,
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
                // subdir,
                subfile,
                dup1,
                dup2,
            }
        }
        pub fn new_without_duplicates() -> Self {
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
            std::fs::File::create(&subdir.join("file3.txt"))
                .unwrap()
                .write_all(b"jkl")
                .unwrap();
            std::fs::File::create(&dup1)
                .unwrap()
                .write_all(b"mno")
                .unwrap();
            std::fs::File::create(&dup2)
                .unwrap()
                .write_all(b"pqr")
                .unwrap();
            Self {
                root,
                file1,
                file2,
                // subdir,
                subfile,
                dup1,
                dup2,
            }
        }
    }

    #[test]
    fn test_find_duplicates_empty() {
        let dir = tempdir().unwrap();
        let result = dups::find_duplicates(&dir.path().to_path_buf(), 1, false, false);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_duplicates_with_dups() {
        let td = TestDir::new_with_duplicates();
        // Use max_depth=0, include_hidden=true to match the file creation and ensure all files are found
        let result = dups::find_duplicates(&td.root.path().to_path_buf(), 0, true, false);
        assert_eq!(result.len(), 1, "Expected one group of duplicates");
        let files = result.values().next().unwrap();
        assert_eq!(files.len(), 2, "Expected two duplicate files");
        assert!(files.contains(&td.dup1));
        assert!(files.contains(&td.dup2));
        assert!(!files.contains(&td.file1));
        assert!(!files.contains(&td.file2));
        assert!(!files.contains(&td.subfile));
    }

    #[test]
    fn test_find_duplicates_without_dups() {
        let td = TestDir::new_without_duplicates();
        let result = dups::find_duplicates(&td.root.path().to_path_buf(), 1, true, false);
        assert!(result.is_empty(), "Expected no duplicates");
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
        let files = dups::list_dir(&dir.path().to_path_buf(), 0, true);
        assert!(files.is_empty(), "Expected no files in empty dir");
    }

    #[test]
    fn test_list_dir_with_files_and_subdirs() {
        let td = TestDir::new_with_duplicates();
        let mut files = dups::list_dir(&td.root.path().to_path_buf(), 0, true);
        files.sort();
        let mut expected = vec![td.file1, td.file2, td.subfile, td.dup1, td.dup2];
        expected.sort();
        assert_eq!(files, expected, "Files found do not match expected");
    }

    #[test]
    fn test_multithreaded_vs_singlethreaded_equivalence() {
        let td = TestDir::new_with_duplicates();
        let mt = dups::find_duplicates(&td.root.path().to_path_buf(), 0, true, false);
        let st = dups::find_duplicates(&td.root.path().to_path_buf(), 0, true, true);
        assert_eq!(
            mt, st,
            "Multithreaded and single-threaded results should be identical"
        );
    }
}
