use std::path::PathBuf;

/// Represent a path in the library
pub struct LibraryPath {
    pub folder_path: PathBuf,
    pub relative_path: PathBuf,
}

impl LibraryPath {
    pub fn as_fs_path(&self) -> PathBuf {
        self.folder_path.join(&self.relative_path)
    }

    pub fn folder_path_as_string(&self) -> String {
        self.folder_path.to_string_lossy().to_string()
    }

    pub fn relative_path_as_string(&self) -> String {
        self.relative_path.to_string_lossy().to_string()
    }
}
