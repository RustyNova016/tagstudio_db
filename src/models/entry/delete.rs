use snafu::ResultExt;

use crate::Entry;
use crate::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Entry {
    /// Delete this entry
    pub async fn delete(self, conn: &mut sqlx::SqliteConnection) -> Result<(), SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query!(sql = "DELETE FROM `entries` WHERE `id` = {self.id}")
            .execute(&mut *conn)
            .await
            .context(SqlxSnafu)?;

        Ok(())
    }
}
