use std::fs::rename;
use std::path::Path;
use std::path::PathBuf;

use sqlx::Acquire;

use crate::models::entry::Entry;

impl Entry {
    /// Move the underlying file of the entry somewhere else in the library
    ///
    /// This takes in a cannonical path to move the file to.
    ///
    /// If the file isn't found on disk, it ignores it and save the new path in the database
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

        let mut trans = conn.begin().await?;
        sqlx::query!(
            "UPDATE `entries` SET `path` = ? WHERE `id` = ?",
            new_lib_path,
            self.id
        )
        .execute(&mut *trans)
        .await?;

        let root_path = self.get_folder(&mut trans).await?.path;
        let mut path = PathBuf::from(root_path);
        path.push(new_lib_path);

        if self.exists_on_disk(&mut trans).await? {
            rename(prev_path, path)?;
        }

        self.path = new_lib_path.to_string();

        trans.commit().await?;
        Ok(())
    }

    pub async fn exists_on_disk(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<bool, crate::Error> {
        Ok(self.get_global_path(conn).await?.try_exists()?)
    }
}
