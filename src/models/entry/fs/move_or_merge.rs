use snafu::ResultExt;
use snafu::Snafu;

use crate::Entry;
use crate::models::entry::fs::merge_same_entry::MergeSameEntryError;
use crate::models::entry::fs::move_entry::MoveEntryError;
use crate::models::library_path::LibraryPath;

impl Entry {
    /// Move the entry to a new location, but if that new location already contains an entry, merge them if they are the same file
    pub async fn move_or_merge_same(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        new_path: &LibraryPath,
    ) -> Result<(), MoveOrMergeSameError> {
        match self.move_entry(conn, new_path).await {
            // It worked!
            Ok(_) => Ok(()),

            // Already had an entry? Let's merge it
            Err(MoveEntryError::EntryPresent { other_entries }) => {
                let Some(other) = other_entries else {
                    return Ok(());
                };

                let this = self.clone();

                other
                    .merge_same_entry(conn, this)
                    .await
                    .context(MergeSameEntrySnafu)?;

                *self = other;
                Ok(())
            }

            // Other error
            Err(err) => Err(err).context(MoveEntrySnafu),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum MoveOrMergeSameError {
    MoveEntryError { source: MoveEntryError },

    MergeSameEntryError { source: MergeSameEntryError },

    MultipleTargetEntries { other_entries: Vec<Entry> },
}
