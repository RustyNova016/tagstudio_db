use std::fs::rename;
use std::path::Path;
use std::path::PathBuf;

use snafu::ResultExt as _;
use sqlx::Acquire;

use crate::models::entry::Entry;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::library_path::LibraryPath;

pub mod merge_entry;
pub mod merge_same_entry;
pub mod move_entry;
pub mod move_or_merge;
impl Entry {
    pub async fn fs_path_to_library_path(
        &self,
        conn: &mut sqlx::SqliteConnection,
        path: &Path,
    ) -> Result<LibraryPath, crate::Error> {
        let folder = self.get_folder(&mut *conn).await?;

        let relative_path = path
            .strip_prefix(&folder.path)
            .map_err(|_| crate::Error::PathNotInFolder)?;

        Ok(LibraryPath {
            folder_path: folder.path_as_pathbuf(),
            relative_path: relative_path.to_path_buf(),
        })
    }

    /// Move the underlying file of the entry somewhere else in the library
    ///
    /// This takes in a cannonical path to move the file to.
    ///
    /// If the file isn't found on disk, it ignores it and save the new path in the database
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn move_file_from_canon_path(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        new_lib_path: &Path,
    ) -> Result<(), crate::Error> {
        let folder = self.get_folder(&mut *conn).await?;

        if new_lib_path.try_exists()?
            || !Entry::find_by_cannon_path(conn, new_lib_path)
                .await?
                .is_empty()
        {
            return Err(crate::Error::DestinationOccupied(
                new_lib_path.to_path_buf(),
            ));
        }

        let relative_path = new_lib_path
            .strip_prefix(&folder.path)
            .map_err(|_| crate::Error::PathNotInFolder)?;

        self.move_file_inner(conn, &relative_path.to_string_lossy())
            .await
    }

    /// Move the underlying file of the entry somewhere else in the library.
    ///
    /// If the file isn't found on disk, it ignores it and save the new path in the database
    async fn move_file_inner(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        new_lib_path: &str,
    ) -> Result<(), crate::Error> {
        let prev_path = self.get_global_path(conn).await?;

        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        let root_path = self.get_folder(&mut trans).await?.path;
        let mut path = PathBuf::from(root_path);
        path.push(new_lib_path);

        if self.exists_on_disk(&mut trans).await? {
            rename(prev_path, path)?;
        }

        self.path = new_lib_path.to_string();
        self.update(&mut *trans).await?;

        trans.commit().await.context(SqlxSnafu)?;
        Ok(())
    }

    pub async fn exists_on_disk(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<bool, crate::Error> {
        Ok(self.get_global_path(conn).await?.try_exists()?)
    }
}
