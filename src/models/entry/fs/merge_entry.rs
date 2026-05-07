use std::backtrace::Backtrace;

use snafu::ResultExt;
use snafu::Snafu;
use sqlx::Acquire;

use crate::Entry;
use crate::SqlxError;
use crate::TextField;
use crate::models::boolean_field::BooleanField;
use crate::models::datetime_field::DatetimeField;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag_entry::TagEntry;

impl Entry {
    /// Merge another entry into self
    pub async fn merge_entry(
        &self,
        conn: &mut sqlx::SqliteConnection,
        other: Self,
    ) -> Result<(), MergeEntryError> {
        let mut trans = conn.begin().await.context(SqlxSnafu).context(SqlSnafu)?;

        TagEntry::replace_entry(&mut *trans, self.id, other.id)
            .await
            .context(SqlSnafu)?;
        TextField::replace_entry(&mut *trans, self.id, other.id)
            .await
            .context(SqlSnafu)?;
        BooleanField::replace_entry(&mut *trans, self.id, other.id)
            .await
            .context(SqlSnafu)?;
        DatetimeField::replace_entry(&mut *trans, self.id, other.id)
            .await
            .context(SqlSnafu)?;

        let other_path = other.get_global_path(&mut trans).await.context(SqlSnafu)?;
        other.delete(&mut *trans).await.context(SqlSnafu)?;

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

    Trash {
        source: trash::Error,
        backtrace: Backtrace,
    },
}
