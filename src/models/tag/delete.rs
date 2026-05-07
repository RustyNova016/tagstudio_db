use snafu::ResultExt;
use sqlx::Acquire;

use crate::SqlxError;
use crate::Tag;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag_alias::TagAlias;
use crate::models::tag_entry::TagEntry;
use crate::models::tag_parent::TagParent;

impl Tag {
    /// Delete the tag and cascade the deletion
    pub async fn delete(self, conn: &mut sqlx::SqliteConnection) -> Result<(), SqlxError> {
        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        TagAlias::delete_by_tag_id(&mut trans, self.id).await?;
        TagEntry::delete_by_tag_id(&mut trans, self.id).await?;
        TagParent::delete_by_tag_id(&mut trans, self.id).await?;

        let sql;
        sea_query::sqlx::sqlite::query!(sql = "DELETE FROM `tags` WHERE `id` = {self.id}")
            .execute(&mut *trans)
            .await
            .context(SqlxSnafu)?;

        trans.commit().await.context(SqlxSnafu)?;
        Ok(())
    }
}
