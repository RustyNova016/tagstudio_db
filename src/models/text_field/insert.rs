use snafu::ResultExt;

use crate::TextField;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl TextField {
    pub async fn insert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql =
                "INSERT OR IGNORE INTO `text_fields` VALUES ({self.value}, NULL, {self.type_key}, {self.entry_id}, 0) RETURNING *;"
        )
        .fetch_one(conn)
        .await
        .context(SqlxSnafu)
    }
}
