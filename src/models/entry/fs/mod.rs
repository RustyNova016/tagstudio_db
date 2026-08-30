use std::fs::rename;
use std::path::Path;

use sequelles::table::update::UpdateSelf;

use crate::models::entry::Entry;

pub mod merge_entry;
pub mod merge_same_entry;
pub mod move_entry;
pub mod move_or_merge;
impl Entry {
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
        library_root: &Path,
    ) -> Result<(), crate::Error> {
        if new_lib_path.try_exists()?
            || !Entry::find_by_full_path(conn, new_lib_path, library_root)
                .await?
                .is_empty()
        {
            return Err(crate::Error::DestinationOccupied(
                new_lib_path.to_path_buf(),
            ));
        }

        let new_relative_path = new_lib_path
            .strip_prefix(library_root)
            .map_err(|_| crate::Error::PathNotInFolder)?;

        self.move_file_inner(conn, &new_relative_path.to_string_lossy(), library_root)
            .await
    }

    /// Move the underlying file of the entry somewhere else in the library.
    ///
    /// If the file isn't found on disk, it ignores it and save the new path in the database
    async fn move_file_inner(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        new_lib_path: &str,
        library_root: &Path,
    ) -> Result<(), crate::Error> {
        let prev_path = self.get_full_path(library_root);
        let new_path = library_root.join(new_lib_path);

        if self.exists_on_disk(library_root)? {
            rename(prev_path, new_path)?;
        }

        self.path = new_lib_path.to_string();
        self.update_self(conn).await?;

        Ok(())
    }

    pub fn exists_on_disk(&self, library_root: &Path) -> Result<bool, crate::Error> {
        Ok(self.get_full_path(library_root).try_exists()?)
    }
}
