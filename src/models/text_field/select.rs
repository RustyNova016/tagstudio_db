use snafu::ResultExt;

use crate::SqlxError;
use crate::TextField;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl TextField {
    pub async fn find_by_entry(
        conn: &mut sqlx::SqliteConnection,
        entry_id: i64,
    ) -> Result<Vec<Self>, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql = "SELECT `text_fields`.*
            FROM `entries` 
                INNER JOIN `text_fields` ON `text_fields`.`entry_id` = `entries`.`id`
            WHERE
                `entries`.`id` = {entry_id}"
        )
        .fetch_all(conn)
        .await
        .context(SqlxSnafu)
    }
}
