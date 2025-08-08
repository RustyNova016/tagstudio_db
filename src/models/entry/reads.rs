use futures::Stream;

use crate::Entry;

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
    ) -> Result<Vec<Self>, crate::Error> {
        Ok(sqlx::query_as!(
            Self,
            "
            SELECT `entries`.* 
            FROM `entries`
            WHERE `entries`.`filename` = ?",
            name
        )
        .fetch_all(conn)
        .await?)
    }
}
