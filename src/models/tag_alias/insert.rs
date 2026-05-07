use snafu::ResultExt;

use crate::TagAlias;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl TagAlias {
    pub async fn insert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql =
                "INSERT OR IGNORE INTO `tag_aliases` VALUES (NULL, {self.name}, {self.tag_id}) RETURNING *;"
        )
        .fetch_one(conn)
        .await
        .context(SqlxSnafu)
    }
}
