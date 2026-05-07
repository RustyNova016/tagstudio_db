use snafu::ResultExt;

use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag_parent::TagParent;

impl TagParent {
    pub async fn insert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql =
                "INSERT OR IGNORE INTO `tag_parents` VALUES ({self.parent_id}, {self.child_id}) RETURNING *;"
        )
        .fetch_one(conn)
        .await
        .context(SqlxSnafu)
    }
}
