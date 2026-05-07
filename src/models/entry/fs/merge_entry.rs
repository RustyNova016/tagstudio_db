use std::backtrace::Backtrace;

use snafu::ResultExt;
use snafu::Snafu;
use sqlx::Acquire;

use crate::Entry;
use crate::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Entry {
    /// Merge another entry into self
    pub async fn merge_entry(
        &self,
        conn: &mut sqlx::SqliteConnection,
        other: Self,
    ) -> Result<(), MergeEntryError> {
        let mut trans = conn.begin().await.context(SqlxSnafu).context(SqlSnafu)?;

        let other_path = other.get_global_path(&mut trans).await.context(SqlSnafu)?;

        sqlx::query!(
            "UPDATE OR IGNORE `tag_entries` SET entry_id = $1 WHERE entry_id = $2",
            self.id,
            other.id
        )
        .execute(&mut *trans)
        .await
        .context(SqlxSnafu)
        .context(SqlSnafu)?;
        sqlx::query!(
            "UPDATE OR IGNORE `boolean_fields` SET entry_id = $1 WHERE entry_id = $2",
            self.id,
            other.id
        )
        .execute(&mut *trans)
        .await
        .context(SqlxSnafu)
        .context(SqlSnafu)?;
        sqlx::query!(
            "UPDATE OR IGNORE `datetime_fields` SET entry_id = $1 WHERE entry_id = $2",
            self.id,
            other.id
        )
        .execute(&mut *trans)
        .await
        .context(SqlxSnafu)
        .context(SqlSnafu)?;
        sqlx::query!(
            "UPDATE OR IGNORE `text_fields` SET entry_id = $1 WHERE entry_id = $2",
            self.id,
            other.id
        )
        .execute(&mut *trans)
        .await
        .context(SqlxSnafu)
        .context(SqlSnafu)?;
        sqlx::query!("DELETE FROM `tag_entries` WHERE entry_id = $1", other.id)
            .execute(&mut *trans)
            .await
            .context(SqlxSnafu)
            .context(SqlSnafu)?;
        sqlx::query!("DELETE FROM `boolean_fields` WHERE entry_id = $1", other.id)
            .execute(&mut *trans)
            .await
            .context(SqlxSnafu)
            .context(SqlSnafu)?;
        sqlx::query!(
            "DELETE FROM `datetime_fields` WHERE entry_id = $1",
            other.id
        )
        .execute(&mut *trans)
        .await
        .context(SqlxSnafu)
        .context(SqlSnafu)?;
        sqlx::query!("DELETE FROM `text_fields` WHERE entry_id = $1", other.id)
            .execute(&mut *trans)
            .await
            .context(SqlxSnafu)
            .context(SqlSnafu)?;
        sqlx::query!("DELETE FROM `entries` WHERE id = $1", other.id)
            .execute(&mut *trans)
            .await
            .context(SqlxSnafu)
            .context(SqlSnafu)?;

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
