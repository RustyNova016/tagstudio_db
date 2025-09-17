use snafu::ResultExt as _;
use tracing::debug;

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
        sqlx::query_as!(Self, "SELECT * FROM `tags` WHERE `id` = $1", id)
            .fetch_optional(conn)
            .await
            .context(SqlxSnafu)
    }

    /// Get the tag by its exact name
    pub async fn find_by_exact_name(
        conn: &mut sqlx::SqliteConnection,
        name: &str,
    ) -> Result<Vec<Self>, SqlxError> {
        sqlx::query_as!(Self, "SELECT * FROM `tags` WHERE `name` = $1", name)
            .fetch_all(conn)
            .await
            .context(SqlxSnafu)
    }

    /// Get all the tags that match a string. This means any tag that have the same name, shorthand, or alias
    pub async fn find_tag_by_name(
        conn: &mut sqlx::SqliteConnection,
        name: String,
    ) -> Result<Vec<Tag>, SqlxError> {
        debug!("Searching tag `{name}` by name");

        EqTagString(name).fetch_all(conn).await
    }
}
