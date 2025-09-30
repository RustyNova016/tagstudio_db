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

#[cfg(test)]
mod test {
    use crate::Tag;
    use crate::tests::fixtures::data::get_test_library;

    #[tokio::test]
    async fn should_delete_tag() {
        let lib = get_test_library().await;
        let conn = &mut lib.db.get().await.unwrap();
        let cat_tag = Tag::find_by_exact_name(conn, "Cat")
            .await
            .unwrap()
            .pop()
            .unwrap();

        assert_eq!(&cat_tag.name, "Cat");

        cat_tag.delete(conn).await.unwrap();

        assert_eq!(
            None,
            Tag::find_by_exact_name(conn, "Cat").await.unwrap().pop()
        );
    }
}
