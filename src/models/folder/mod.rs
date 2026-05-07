use std::path::PathBuf;

use sqlx::prelude::FromRow;

pub mod insert;
pub mod select;

/// Represent a root folder of the library
#[derive(Debug, FromRow, Clone, PartialEq, Eq)]
pub struct Folder {
    pub id: i64,
    /// The full path of the folder
    pub path: String,
    pub uuid: String,
}

impl Folder {
    pub fn path_as_pathbuf(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}
