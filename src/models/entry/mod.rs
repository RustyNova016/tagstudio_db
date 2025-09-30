use core::str::FromStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use chrono::NaiveDateTime;
use sqlx::FromRow;

pub mod fs;
pub mod insert;
pub mod relations;
pub mod select;
pub mod tags;

#[derive(Debug, FromRow, Clone, PartialEq, Eq, sequelles::Table)]
#[sequelles(db_name = "entries", snafu)]
#[sequelles(sqlite)]
#[sequelles(update, select_unique, delete, insert_struct, upsert)]
#[sequelles(primary_key(key_name = "pk", columns(id)))]
#[sequelles(unique_key(key_name = "path", columns(path)))]
pub struct Entry {
    #[sequelles(auto_increment)]
    pub id: i64,
    pub path: String,
    pub filename: String,
    pub suffix: String,
    pub date_created: Option<NaiveDateTime>,
    pub date_modified: Option<NaiveDateTime>,
    pub date_added: Option<NaiveDateTime>,
}

impl Entry {
    /// Get the full path on the filesystem
    pub fn get_full_path(&self, library_root: &Path) -> PathBuf {
        library_root.join(&self.path)
    }

    /// Get the relative path of the file
    pub fn get_relative_path(&self) -> PathBuf {
        PathBuf::from_str(&self.path).unwrap()
    }

    /// Get the filename of the file. This is more secure than the inner `filename` field as it takes it from the path
    pub fn get_filename(&self) -> Option<OsString> {
        self.get_relative_path()
            .file_name()
            .map(|f| f.to_os_string())
    }
}
