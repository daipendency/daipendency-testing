//! Management of temporary directories.

use std::io::Write;
use std::{fs::File, io::Error, path::PathBuf};

use tempfile::TempDir as UpstreamTempDir;

/// A temporary directory.
pub struct TempDir {
    temp_dir: UpstreamTempDir,
    /// The path to the temporary directory.
    pub path: PathBuf,
}

impl Default for TempDir {
    fn default() -> Self {
        Self::new()
    }
}

impl TempDir {
    pub fn new() -> Self {
        let temp_dir = UpstreamTempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();
        Self { temp_dir, path }
    }

    /// Create a file in the temporary directory.
    pub fn create_file(&self, path: &str, content: &str) -> Result<PathBuf, Error> {
        let file_path = self.temp_dir.path().join(path);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let mut file = File::create(&file_path)?;
        write!(file, "{}", content)?;
        Ok(file_path)
    }
}
