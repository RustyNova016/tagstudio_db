use snafu::ResultExt;

use crate::Entry;
use crate::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::library_path::LibraryPath;

impl Entry {
    pub async fn find_by_library_path(
        conn: &mut sqlx::SqliteConnection,
        path: &LibraryPath,
    ) -> Result<Vec<Self>, SqlxError> {
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
        .fetch_all(&mut *conn)
        .await
        .context(SqlxSnafu)
    }
}
