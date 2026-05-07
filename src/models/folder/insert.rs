use snafu::ResultExt;

use crate::Folder;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Folder {
    pub async fn insert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql =
                "INSERT OR IGNORE INTO `folders` VALUES (NULL, {self.path}, {self.uuid}) RETURNING *;"
        )
        .fetch_one(conn)
        .await
        .context(SqlxSnafu)
    }
}
