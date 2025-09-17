use snafu::ResultExt;

use crate::models::entry::Entry;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

pub struct TextField {
    pub value: Option<String>,
    pub id: i64,
    pub type_key: String,
    pub entry_id: i64,
    pub position: i64,
}

impl TextField {
    pub async fn insert_text_field(
        conn: &mut sqlx::SqliteConnection,
        entry_id: i64,
        type_key: &str,
        value: &str,
    ) -> Result<(), SqlxError> {
        sqlx::query!(
            "INSERT INTO `text_fields` VALUES (?, NULL, ?, ?, 0)",
            value,
            type_key,
            entry_id
        )
        .execute(conn)
        .await
        .context(SqlxSnafu)?;
        Ok(())
    }

    pub async fn get_entry(&self, conn: &mut sqlx::SqliteConnection) -> Result<Entry, SqlxError> {
        Entry::find_by_id(conn, self.entry_id)
            .await
            .transpose()
            .expect("The text field has no associated entry")
    }
}
