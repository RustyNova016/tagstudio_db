use std::path::PathBuf;

use snafu::ResultExt as _;

use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

/// Represent a root folder of the library
pub struct Folder {
    pub id: i64,
    /// The full path of the folder
    pub path: String,
    pub uuid: String,
}

impl Folder {
    /// Get the row by its id
    pub async fn find_by_id(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, SqlxError> {
        sqlx::query_as!(Self, "SELECT * FROM `folders` WHERE `id` = ?", id)
            .fetch_optional(conn)
            .await
            .context(SqlxSnafu)
    }

    pub fn path_as_pathbuf(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}
