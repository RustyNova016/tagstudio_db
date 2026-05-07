use snafu::ResultExt;
use sqlx::Acquire;

use crate::SqlxError;
use crate::models::boolean_field::BooleanField;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl BooleanField {
    /// Modify the entries to point to another
    pub async fn replace_entry(
        conn: &mut sqlx::SqliteConnection,
        old_entry_id: i64,
        new_entry_id: i64,
    ) -> Result<(), SqlxError> {
        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        // Modify the entry
        let sql;
        sea_query::sqlx::sqlite::query!(
            sql = "UPDATE OR IGNORE `boolean_fields` SET entry_id = {new_entry_id} WHERE entry_id = {old_entry_id}"
        )
        .execute(&mut *trans)
        .await
        .context(SqlxSnafu)?;

        // Remove any duplicates that weren't modified
        let sql;
        sea_query::sqlx::sqlite::query!(
            sql = "DELETE FROM `boolean_fields` WHERE `entry_id` = {old_entry_id}"
        )
        .execute(&mut *trans)
        .await
        .context(SqlxSnafu)?;

        Ok(())
    }
}
