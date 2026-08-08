use futures::StreamExt;
use futures::stream::BoxStream;
use snafu::ResultExt;

use crate::SqlxError;
use crate::Tag;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Tag {
    pub fn get_parents<'l>(
        &'l self,
        conn: &'l mut sqlx::SqliteConnection,
    ) -> BoxStream<'l, Result<Tag, SqlxError>> {
        sqlx::query_as(
            "
            SELECT `tags`.* 
            FROM `tags` 
                INNER JOIN `tag_parents` ON `tag_parents`.`parent_id` = `tags`.`id`
            WHERE `tag_parents`.`child_id` = $1",
        )
        .bind(self.id)
        .fetch(conn)
        .map(|val| val.context(SqlxSnafu))
        .boxed()
    }

    pub fn get_children<'l>(
        &'l self,
        conn: &'l mut sqlx::SqliteConnection,
    ) -> BoxStream<'l, Result<Tag, SqlxError>> {
        sqlx::query_as(
            "
            SELECT `tags`.* 
            FROM `tags` 
                INNER JOIN `tag_parents` ON `tag_parents`.`child_id` = `tags`.`id`
            WHERE `tag_parents`.`parent_id` = $1",
        )
        .bind(self.id)
        .fetch(conn)
        .map(|val| val.context(SqlxSnafu))
        .boxed()
    }

    pub async fn get_entry_count(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<i64, SqlxError> {
        sqlx::query_scalar(
            "
            SELECT COUNT(*)
            FROM `tag_entries` 
            WHERE `tag_entries`.`tag_id` = $1",
        )
        .bind(self.id)
        .fetch_one(conn)
        .await
        .context(SqlxSnafu)
    }
}
