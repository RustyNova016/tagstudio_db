use std::path::Path;

use futures::Stream;
use snafu::ResultExt as _;

use crate::Entry;
use crate::client::db::traits::read_conn::ReadConnection;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::library_path::LibraryPath;
use crate::query::eq_absolute_path::EqAbsolutePath;
use crate::query::eq_entry_id::EqEntryId;
use crate::query::trait_entry_filter::QueryEntryFilter;

impl Entry {
    /// Get the row by its id
    pub async fn find_by_id(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, SqlxError> {
        EqEntryId(id).fetch_optional(conn).await
    }

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

    /// Get the entry by its cannon path (Aka, the library's root path + the file's path in the library)
    pub async fn find_by_cannon_path(
        conn: &mut sqlx::SqliteConnection,
        path: &Path,
    ) -> Result<Vec<Self>, SqlxError> {
        EqAbsolutePath(path.to_string_lossy().to_string())
            .fetch_all(conn)
            .await
    }

    pub async fn find_by_library_path(
        conn: &mut impl ReadConnection,
        path: &LibraryPath,
    ) -> Result<Option<Self>, SqlxError> {
        let folder = path.folder_path_as_string();
        let relative = path.relative_path_as_string();

        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql = "
            SELECT `entries`.* 
            FROM `entries`
                INNER JOIN `folders` ON `folders`.`id` = entries.folder_id
            WHERE `folders`.`path` = {folder} AND `entries`.`path` = {relative}
        "
        )
        .fetch_optional(conn.conn())
        .await
        .context(SqlxSnafu)
    }
}
