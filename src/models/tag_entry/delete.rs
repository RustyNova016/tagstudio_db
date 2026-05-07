use snafu::ResultExt;

use crate::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag_entry::TagEntry;

impl TagEntry {
    /// Delete all the TagEntries that have a specific Tag.id
    pub async fn delete_by_tag_id(
        conn: &mut sqlx::SqliteConnection,
        tag_id: i64,
    ) -> Result<(), SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query!(
            sql = "DELETE FROM `tag_entries` WHERE `tag_id` = {tag_id}"
        )
        .execute(&mut *conn)
        .await
        .context(SqlxSnafu)?;

        Ok(())
    }
}
