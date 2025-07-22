use std::fs::rename;
use std::path::Path;
use std::path::PathBuf;

use sqlx::Acquire;

use crate::models::entry::Entry;

impl Entry {
    /// Move the underlying file of the entry somewhere else in the library
    /// This takes in a cannonical path to move the file to.
    pub async fn move_file_from_canon_path(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        new_lib_path: &Path,
    ) -> Result<(), crate::Error> {
        let folder = self.get_folder(&mut *conn).await?;
        let relative_path = new_lib_path
            .strip_prefix(&folder.path)
            .map_err(|_| crate::Error::PathNotInFolder)?;

        self.move_file(conn, &relative_path.to_string_lossy()).await
    }

    /// Move the underlying file of the entry somewhere else in the library
    pub async fn move_file(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        new_lib_path: &str,
    ) -> Result<(), crate::Error> {
        let mut trans = conn.begin().await?;
        let prev_path = self.get_global_path(&mut trans).await?;

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

        rename(prev_path, path)?;

        self.path = new_lib_path.to_string();

        trans.commit().await?;
        Ok(())
    }
}
