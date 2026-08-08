use std::backtrace::Backtrace;
use std::path::Path;

use sequelles::Delete;
use snafu::ResultExt;
use snafu::Snafu;
use sqlx::Acquire;

use crate::Entry;
use crate::SqlxError;
use crate::TextField;
use crate::models::datetime_field::DatetimeField;
use crate::models::entry::EntrySqlError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag_entry::TagEntry;

impl Entry {
    /// Merge another entry into self
    pub async fn merge_entry(
        &self,
        conn: &mut sqlx::SqliteConnection,
        other: Self,
        library_root: &Path,
    ) -> Result<(), MergeEntryError> {
        let mut trans = conn.begin().await.context(SqlxSnafu).context(SqlSnafu)?;

        TagEntry::replace_entry(&mut trans, self.id, other.id)
            .await
            .context(SqlSnafu)?;
        TextField::replace_entry(&mut trans, self.id, other.id)
            .await
            .context(SqlSnafu)?;
        DatetimeField::replace_entry(&mut trans, self.id, other.id)
            .await
            .context(SqlSnafu)?;

        let other_path = other.get_full_path(library_root);
        other.delete(&mut trans).await.context(EntrySqlSnafu)?;

        if other_path.exists() {
            trash::delete(other_path).context(TrashSnafu)?;
        }

        trans.commit().await.context(SqlxSnafu).context(SqlSnafu)?;
        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum MergeEntryError {
    Sql {
        #[snafu(backtrace)]
        source: SqlxError,
    },

    EntrySqlError {
        source: EntrySqlError,
    },

    Trash {
        source: trash::Error,
        backtrace: Backtrace,
    },
}
