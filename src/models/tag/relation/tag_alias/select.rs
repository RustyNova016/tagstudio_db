use futures::StreamExt;
use futures::stream::BoxStream;
use snafu::ResultExt;

use crate::SqlxError;
use crate::Tag;
use crate::TagAlias;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Tag {
    pub fn get_aliases<'l>(
        &'l self,
        conn: &'l mut sqlx::SqliteConnection,
    ) -> BoxStream<'l, Result<TagAlias, SqlxError>> {
        sqlx::query_as(
            "
            SELECT `tag_aliases`.* 
            FROM `tags` 
                INNER JOIN `tag_aliases` ON `tag_aliases`.`tag_id` = `tags`.`id`
            WHERE `tags`.`id` = $1",
        )
        .bind(self.id)
        .fetch(conn)
        .map(|val| val.context(SqlxSnafu))
        .boxed()
    }
}
