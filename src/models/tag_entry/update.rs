use snafu::ResultExt;
use sqlx::Acquire;

use crate::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag_entry::TagEntry;

impl TagEntry {
    /// Modify all the tag of an entry to point to another
    pub async fn replace_entry(
        conn: &mut sqlx::SqliteConnection,
        old_entry_id: i64,
        new_entry_id: i64,
    ) -> Result<(), SqlxError> {
        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        // Modify the entry
        let sql;
        sea_query::sqlx::sqlite::query!(
            sql = "UPDATE OR IGNORE `tag_entries` SET entry_id = {new_entry_id} WHERE entry_id = {old_entry_id}"
        )
        .execute(&mut *trans)
        .await
        .context(SqlxSnafu)?;

        // Remove any duplicates that weren't modified
        let sql;
        sea_query::sqlx::sqlite::query!(
            sql = "DELETE FROM `tag_entries` WHERE `entry_id` = {old_entry_id}"
        )
        .execute(&mut *trans)
        .await
        .context(SqlxSnafu)?;

        Ok(())
    }
}
