use snafu::ResultExt;

use crate::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag_parent::TagParent;

impl TagParent {
    /// Delete all the TagParent that have a specific Tag.id as child or parent
    pub async fn delete_by_tag_id(
        conn: &mut sqlx::SqliteConnection,
        tag_id: i64,
    ) -> Result<(), SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query!(
            sql = "DELETE FROM `tag_parents` WHERE `parent_id` = {tag_id} OR `child_id` = {tag_id}"
        )
        .execute(&mut *conn)
        .await
        .context(SqlxSnafu)?;

        Ok(())
    }
}
