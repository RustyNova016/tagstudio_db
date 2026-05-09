use snafu::ResultExt;

use crate::Folder;
use crate::SqlxError;
use crate::client::db::traits::read_conn::ReadConnection;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Folder {
    /// Get the row by its id
    pub async fn find_by_id(
        conn: &mut impl ReadConnection,
        id: i64,
    ) -> Result<Option<Self>, SqlxError> {
        let sql;

        sea_query::sqlx::sqlite::query_as!(sql = "SELECT * FROM `folders` WHERE `id` = {id}")
            .fetch_optional(conn.conn())
            .await
            .context(SqlxSnafu)
    }

    pub async fn find_by_path(
        conn: &mut impl ReadConnection,
        path: &str,
    ) -> Result<Option<Self>, SqlxError> {
        let sql;

        sea_query::sqlx::sqlite::query_as!(sql = "SELECT * FROM `folders` WHERE path = {path}")
            .fetch_optional(conn.conn())
            .await
            .context(SqlxSnafu)
    }
}
