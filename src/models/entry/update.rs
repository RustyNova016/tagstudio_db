use snafu::ResultExt;

use crate::Entry;
use crate::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Entry {
    /// Delete this entry
    pub async fn update(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query!(
            sql = "
            UPDATE `entries` SET 
                `folder_id` = {self.folder_id},
                `path` = {self.path},
                `filename` = {self.filename},
                `suffix` = {self.suffix},
                `date_created` = {self.date_created},
                `date_modified` = {self.date_modified},
                `date_added` = {self.date_added}
            WHERE `id` = {self.id}
        "
        )
        .execute(&mut *conn)
        .await
        .context(SqlxSnafu)?;

        Ok(())
    }
}
