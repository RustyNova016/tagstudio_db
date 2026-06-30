use snafu::ResultExt as _;
use tracing::debug;

use crate::client::db::traits::read_conn::ReadConnection;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag::Tag;
use crate::query::eq_tag_string::EqTagString;
use crate::query::trait_tag_filter::TagFilter;

impl Tag {
    /// Get the row by its id
    pub async fn find_by_id(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(sql = "SELECT * FROM `tags` WHERE `id` = {id}")
            .fetch_optional(conn)
            .await
            .context(SqlxSnafu)
    }

    /// Get the tag by its exact name
    pub async fn find_by_exact_name(
        conn: &mut sqlx::SqliteConnection,
        name: &str,
    ) -> Result<Vec<Self>, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(sql = "SELECT * FROM `tags` WHERE `name` = {name}")
            .fetch_all(conn)
            .await
            .context(SqlxSnafu)
    }

    /// Get all the tags that match a string. This means any tag that have the same name, shorthand, or alias
    pub async fn find_by_name(
        conn: &mut impl ReadConnection,
        name: String,
    ) -> Result<Vec<Tag>, SqlxError> {
        debug!("Searching tag `{name}` by name");

        EqTagString(name).fetch_all(conn.conn()).await
    }
}

#[cfg(test)]
mod test {
    use crate::Tag;
    use crate::tests::fixtures::data::get_test_library;

    #[tokio::test]
    async fn should_find_tag() {
        let lib = get_test_library().await;
        let conn = &mut lib.db.get().await.unwrap();

        let mut cat_tags = Tag::find_by_exact_name(conn, "Cat").await.unwrap();

        assert_eq!(cat_tags.len(), 1);
        let cat_tag = cat_tags.pop().unwrap();
        assert_eq!(&cat_tag.name, "Cat");

        let cat_id = Tag::find_by_id(conn, cat_tag.id).await.unwrap().unwrap();
        assert_eq!(cat_id.id, cat_tag.id);

        let mut cats_name = Tag::find_by_name(conn, cat_tag.name).await.unwrap();
        assert_eq!(cats_name.len(), 1);
        let cat_name = cats_name.pop().unwrap();
        assert_eq!(&cat_name.name, "Cat");

        let mut cats_alias = Tag::find_by_name(conn, "Kitty".to_string()).await.unwrap();
        assert_eq!(cats_alias.len(), 1);
        let cat_alias = cats_alias.pop().unwrap();
        assert_eq!(&cat_alias.name, "Cat");
    }
}
