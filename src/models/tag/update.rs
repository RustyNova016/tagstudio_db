use snafu::ResultExt;

use crate::SqlxError;
use crate::Tag;
use crate::client::db::traits::write_conn::WriteConnection;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Tag {
    #[cfg_attr(feature = "hotpath", hotpath::future_fn(log = true))]
    pub async fn update(&self, conn: &mut impl WriteConnection) -> Result<(), SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query!(
            sql = "
            UPDATE `tags` SET 
                `name` = {self.name},
                `shorthand` = {self.shorthand},
                `color_namespace` = {self.color_namespace},
                `color_slug` = {self.color_slug},
                `is_hidden` = {self.is_hidden},
                `is_category` = {self.is_category},
                `icon` = {self.icon},
                `disambiguation_id` = {self.disambiguation_id}
            WHERE `id` = {self.id}
        "
        )
        .execute(conn.conn())
        .await
        .context(SqlxSnafu)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::Tag;
    use crate::tests::fixtures::data::get_test_library;

    #[tokio::test]
    async fn should_update_tag() {
        let lib = get_test_library().await;
        let conn = &mut lib.db.get().await.unwrap();
        let mut cat_tag = Tag::find_by_exact_name(conn, "Cat")
            .await
            .unwrap()
            .pop()
            .unwrap();

        cat_tag.name = "Chat".to_string();
        cat_tag.update(conn).await.unwrap();

        let mut chat_tags = Tag::find_by_exact_name(conn, "Chat").await.unwrap();
        assert_eq!(chat_tags.len(), 1);
        assert_eq!(chat_tags.pop().unwrap().id, cat_tag.id);

        let cat_tags = Tag::find_by_exact_name(conn, "Cat").await.unwrap();
        assert_eq!(cat_tags.len(), 0);
    }
}
