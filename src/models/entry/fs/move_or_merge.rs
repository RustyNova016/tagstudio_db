use std::path::Path;

use snafu::ResultExt;
use snafu::Snafu;

use crate::Entry;
use crate::models::entry::fs::merge_same_entry::MergeSameEntryError;
use crate::models::entry::fs::move_entry::MoveEntryError;

impl Entry {
    /// Move the entry to a new location, but if that new location already contains an entry, merge them if they are the same file
    pub async fn move_or_merge_same(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        new_path: &Path,
        library_root: &Path,
    ) -> Result<(), MoveOrMergeSameError> {
        match self.move_entry(conn, new_path, library_root).await {
            // It worked!
            Ok(_) => Ok(()),

            // Already had an entry? Let's merge it
            Err(MoveEntryError::EntryPresent { mut other_entries }) => {
                if other_entries.len() > 1 {
                    return MultipleTargetEntriesSnafu { other_entries }.fail();
                }

                let Some(other) = other_entries.pop() else {
                    return Ok(());
                };

                let this = self.clone();

                other
                    .merge_same_entry(conn, this, library_root)
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
