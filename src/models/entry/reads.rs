use futures::Stream;
use snafu::ResultExt as _;

use crate::Entry;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Entry {
    pub fn stream_entries(
        conn: &mut sqlx::SqliteConnection,
    ) -> std::pin::Pin<Box<dyn Stream<Item = Result<Entry, sqlx::Error>> + Send + '_>> {
        sqlx::query_as!(Self, "SELECT * FROM `entries`").fetch(conn)
    }

    /// Get the entry by its filename
    pub async fn find_by_filename(
        conn: &mut sqlx::SqliteConnection,
        name: &str,
    ) -> Result<Vec<Self>, SqlxError> {
        sqlx::query_as!(
            Self,
            "
            SELECT `entries`.* 
            FROM `entries`
            WHERE `entries`.`filename` = ?",
            name
        )
        .fetch_all(conn)
        .await
        .context(SqlxSnafu)
    }
}
