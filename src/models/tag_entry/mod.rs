use snafu::ResultExt as _;
use tracing::debug;

use crate::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

pub struct TagEntry {
    pub tag_id: i64,
    pub entry_id: i64,
}

impl TagEntry {
    pub async fn insert(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), SqlxError> {
        debug!("Adding tag {} to entry {}", self.tag_id, self.entry_id);

        sqlx::query("INSERT OR IGNORE INTO `tag_entries`(entry_id, tag_id) VALUES (?, ?)")
            .bind(self.entry_id)
            .bind(self.tag_id)
            .execute(conn)
            .await
            .context(SqlxSnafu)?;

        Ok(())
    }
}
