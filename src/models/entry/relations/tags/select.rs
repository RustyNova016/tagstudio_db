use snafu::ResultExt;

use crate::Entry;
use crate::SqlxError;
use crate::Tag;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Entry {
    /// Get the tags of the entry
    pub async fn get_tags(&self, conn: &mut sqlx::SqliteConnection) -> Result<Vec<Tag>, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql = "
            SELECT `tags`.* 
            FROM `entries` 
                INNER JOIN `tag_entries` ON `tag_entries`.`entry_id` = `entries`.`id`
                INNER JOIN `tags` ON `tag_entries`.`tag_id` = `tags`.`id`
            WHERE
                `entries`.`id` = {self.id}"
        )
        .fetch_all(conn)
        .await
        .context(SqlxSnafu)
    }
}
