use snafu::ResultExt as _;
use tracing::debug;

use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

/// An alias of a tag
pub struct TagAlias {
    pub id: i64,
    pub name: String,
    pub tag_id: i64,
}

impl TagAlias {
    /// Fetch the alias by its name and tag.
    ///
    /// ⚠️ Returns a vec as an alias's uniqueness isn't enforced
    pub async fn find_by_name(
        conn: &mut sqlx::SqliteConnection,
        name: &str,
        tag_id: i64,
    ) -> Result<Vec<Self>, SqlxError> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM `tag_aliases` WHERE `name` = ? AND `tag_id` = ?",
            name,
            tag_id
        )
        .fetch_all(conn)
        .await
        .context(SqlxSnafu)
    }

    /// Insert a new alias for a tag.
    ///
    /// ⚠️ This enforces the alias's uniqueness in client code, as there is no unique constraint in the database
    pub async fn insert_tag_alias(
        conn: &mut sqlx::SqliteConnection,
        name: &str,
        tag_id: i64,
    ) -> Result<(), SqlxError> {
        if name.is_empty() {
            return Ok(());
        }

        if TagAlias::find_by_name(conn, name, tag_id).await?.is_empty() {
            debug!("Adding Alias `{name}` to tag `{}`", tag_id);
            sqlx::query!(
                "INSERT INTO `tag_aliases` VALUES (NULL, ?, ?)",
                name,
                tag_id
            )
            .execute(conn)
            .await
            .context(SqlxSnafu)?;
        } else {
            debug!("Ignoring alias addition {name}");
        }

        Ok(())
    }
}
