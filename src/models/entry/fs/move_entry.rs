use std::io;

use filium::path::PathExt;
use sequelles::table::update::UpdateSelf;
use snafu::ResultExt;
use sqlx::Acquire;

use crate::Entry;
use crate::SqlxError;
use crate::models::entry::EntrySqlError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::library_path::LibraryPath;

impl Entry {
    /// Move an entry in the library and on disk. This takes in a global path.
    ///
    /// This function is really conservative, and won't delete any file if unsure.
    /// The only time a file is deleted, is when there's an entry less file at the destination, and both have the same hash
    pub async fn move_entry(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        new_path: &LibraryPath,
    ) -> Result<(), MoveEntryError> {
        // Start a transaction to prevent concurent writes
        let mut trans = conn
            .begin()
            .await
            .context(SqlxSnafu)
            .context(DatabaseSnafu)?;

        // Check if we aren't changing the library folder
        let prev_path = self
            .get_library_path(&mut trans)
            .await
            .context(DatabaseSnafu)?;

        if prev_path.folder_path != new_path.folder_path {
            return InvalidFolderSnafu.fail();
        }

        // Check if there's already an entry
        let other_entries = Self::find_by_library_path(&mut trans, new_path)
            .await
            .context(DatabaseSnafu)?;
        if !other_entries.is_empty() {
            return EntryPresentSnafu { other_entries }.fail();
        }

        // No entry. Let's start the move
        let new_relatve_path = new_path.relative_path.to_string_lossy().to_string();

        // Let's move the file, and overwrite if there's the same hash. This allows for non entry files to be merged in case of a manual copy.
        // Since the file is the same, and there's no database entry about it, this means that there's no data lost (Maybe just Reflinks on BTRFS but shhh)
        if !prev_path
            .as_fs_path()
            .smart_move(&new_path.as_fs_path())
            .context(MoveSnafu)?
        {
            // NOT MOVED! There is a file there that is blocking us. Let's return an error
            return DestinationOccupiedSnafu.fail();
        }

        // We have moved! Now let's update `self` and commit the changes to the database
        self.path = new_relatve_path;
        self.update_self(&mut trans).await.context(EntrySqlSnafu)?;

        trans
            .commit()
            .await
            .context(SqlxSnafu)
            .context(DatabaseSnafu)?;
        Ok(())
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum MoveEntryError {
    /// There is already an entry at the target location
    EntryPresent {
        other_entries: Vec<Entry>,
    },

    DatabaseError {
        source: SqlxError,
    },

    EntrySqlError {
        source: EntrySqlError,
    },

    MoveError {
        source: io::Error,
    },

    /// Couldn't move the file due to another being in the target path
    DestinationOccupied,

    /// The entry's library folder cannot be changed (Yet)
    InvalidFolder,
}
