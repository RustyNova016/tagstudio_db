use snafu::ResultExt as _;
use tracing::debug;

use crate::Entry;
use crate::client::db::traits::write_conn::WriteConnection;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Entry {
    pub async fn insert(&self, conn: &mut impl WriteConnection) -> Result<Self, SqlxError> {
        debug!("Adding entry `{}`", self.path);

        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql = "INSERT INTO `entries` VALUES (NULL,
                {self.folder_id},
                {self.path},
                {self.filename},
                {self.suffix},
                {self.date_created},
                {self.date_modified},
                {self.date_added}
            ) RETURNING *;"
        )
        .fetch_one(conn.conn())
        .await
        .context(SqlxSnafu)
    }
}
