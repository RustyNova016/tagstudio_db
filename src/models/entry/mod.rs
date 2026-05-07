use core::str::FromStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use chrono::NaiveDateTime;
use sqlx::FromRow;

use crate::models::errors::sqlx_error::SqlxError;
use crate::models::folder::Folder;
use crate::models::library_path::LibraryPath;
use crate::query::eq_absolute_path::EqAbsolutePath;
use crate::query::eq_entry_id::EqEntryId;
use crate::query::trait_entry_filter::EntryFilter;

pub mod delete;
pub mod fs;
pub mod insert;
pub mod relations;
pub mod search;
pub mod select;
pub mod tags;
pub mod update;

#[derive(Debug, FromRow, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: i64,
    pub folder_id: i64,
    pub path: String,
    pub filename: String,
    pub suffix: String,
    pub date_created: Option<NaiveDateTime>,
    pub date_modified: Option<NaiveDateTime>,
    pub date_added: Option<NaiveDateTime>,
}

impl Entry {
    /// Get the row by its id
    pub async fn find_by_id(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, SqlxError> {
        EqEntryId(id).fetch_optional(conn).await
    }

    /// Get the entry by its cannon path (Aka, the library's root path + the file's path in the library)
    pub async fn find_by_cannon_path(
        conn: &mut sqlx::SqliteConnection,
        path: &Path,
    ) -> Result<Vec<Self>, SqlxError> {
        EqAbsolutePath(path.to_string_lossy().to_string())
            .fetch_all(conn)
            .await
    }

    pub async fn get_folder(&self, conn: &mut sqlx::SqliteConnection) -> Result<Folder, SqlxError> {
        Folder::find_by_id(conn, self.folder_id)
            .await
            .transpose()
            .unwrap_or_else(|| panic!("Couldn't find entry's folder! Something went horribly wrong, as every entries should have their own folder. Tried to get folder id: {}", self.id))
    }

    /// Get the path of the file on the filesystem
    #[cfg_attr(feature = "hotpath", hotpath::future_fn(log = true))]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn get_global_path(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<PathBuf, SqlxError> {
        let root_path = self.get_folder(&mut *conn).await?.path;
        let mut path = PathBuf::from(root_path);
        path.push(&self.path);
        Ok(path)
    }

    pub async fn get_library_path(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<LibraryPath, SqlxError> {
        Ok(LibraryPath {
            folder_path: self.get_folder(&mut *conn).await?.path.into(),
            relative_path: PathBuf::from(&self.path),
        })
    }

    /// Get the relative path of the file
    pub fn get_relative_path(&self) -> PathBuf {
        PathBuf::from_str(&self.path).unwrap()
    }

    /// Get the filename of the file. This is more secured than the inner `filename` field as it takes it from the path
    pub fn get_filename(&self) -> Option<OsString> {
        self.get_relative_path()
            .file_name()
            .map(|f| f.to_os_string())
    }
}
