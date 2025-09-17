use snafu::ResultExt;
use tracing::debug;

use crate::Tag;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Tag {
    /// Insert a new tag in the database
    pub async fn insert_tag(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, SqlxError> {
        debug!("Adding tag `{}`", self.name);

        sqlx::query_as!(
            Self,
            "INSERT INTO `tags` VALUES (NULL, ?, ?, ?, ?, ?, ?, ?) RETURNING *;",
            self.name,
            self.shorthand,
            self.color_namespace,
            self.color_slug,
            self.is_category,
            self.icon,
            self.disambiguation_id
        )
        .fetch_one(conn)
        .await
        .context(SqlxSnafu)
    }

    /// Search a tag by its name or aliases, and if not found, insert it
    pub async fn get_by_name_or_insert_new(
        conn: &mut sqlx::SqliteConnection,
        name: String,
    ) -> Result<Vec<Self>, SqlxError> {
        let tags = Self::find_tag_by_name(conn, name.clone()).await?;

        if !tags.is_empty() {
            return Ok(tags);
        }

        let tag = Self::from(name).insert_tag(conn).await?;
        Ok(vec![tag])
    }
}
