use std::io;
use std::path::Path;

use filium::path::PathExt;
use snafu::ResultExt;
use snafu::Snafu;

use crate::Entry;
use crate::SqlxError;
use crate::models::entry::fs::merge_entry::MergeEntryError;

impl Entry {
    /// Merge another entry into this one, if the file is the same
    pub async fn merge_same_entry(
        &self,
        conn: &mut sqlx::SqliteConnection,
        other: Self,
        library_root: &Path,
    ) -> Result<(), MergeSameEntryError> {
        let self_global_path = self.get_full_path(library_root);
        let other_global_path = other.get_full_path(library_root);

        // Check if the files are the same
        if !self_global_path
            .same_file(&other_global_path)
            .context(FSSnafu)?
        {
            return DifferentFilesSnafu.fail();
        }

        // Same, so we merge
        self.merge_entry(conn, other, library_root)
            .await
            .context(MergeEntrySnafu)?;

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum MergeSameEntryError {
    DatabaseError { source: SqlxError },

    MergeEntryError { source: MergeEntryError },

    FSError { source: io::Error },

    DifferentFiles,
}
