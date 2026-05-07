use futures::Stream;
use snafu::ResultExt as _;

use crate::Entry;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Entry {
    pub fn stream_entries(
        conn: &mut sqlx::SqliteConnection,
    ) -> std::pin::Pin<Box<dyn Stream<Item = Result<Entry, sqlx::Error>> + Send + '_>> {
        sqlx::query_as("SELECT * FROM `entries`").fetch(conn)
    }

    /// Get the entry by its filename
    pub async fn find_by_filename(
        conn: &mut sqlx::SqliteConnection,
        name: &str,
    ) -> Result<Vec<Self>, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql = "
            SELECT `entries`.* 
            FROM `entries`
            WHERE `entries`.`filename` = {name}"
        )
        .fetch_all(conn)
        .await
        .context(SqlxSnafu)
    }

    /// Get the row by its path
    pub async fn find_by_path(
        conn: &mut sqlx::SqliteConnection,
        path: &str,
    ) -> Result<Vec<Self>, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql = "
            SELECT `entries`.* 
            FROM `entries`
            WHERE `entries`.`path` = {path}"
        )
        .fetch_all(conn)
        .await
        .context(SqlxSnafu)
    }
}
