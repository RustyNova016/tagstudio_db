use snafu::ResultExt;
use sqlx::Acquire;
use tracing::debug;

use crate::SqlxError;
use crate::Tag;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag_parent::TagParent;

impl Tag {
    /// Add a child tag to this tag
    pub async fn add_child(
        &self,
        conn: &mut sqlx::SqliteConnection,
        child_id: i64,
    ) -> Result<TagParent, SqlxError> {
        debug!(
            "Adding child `{child_id}` to tag `{}` ({})",
            self.name, self.id
        );

        TagParent {
            child_id,
            parent_id: self.id,
        }
        .insert(conn)
        .await
    }

    pub async fn add_children(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tags: &Vec<Tag>,
    ) -> Result<(), SqlxError> {
        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        for tag in tags {
            self.add_child(&mut trans, tag.id).await?;
        }

        trans.commit().await.context(SqlxSnafu)?;

        Ok(())
    }

    pub async fn add_parent(
        &self,
        conn: &mut sqlx::SqliteConnection,
        parent_id: i64,
    ) -> Result<TagParent, SqlxError> {
        debug!(
            "Adding parent `{parent_id}` to tag `{}` ({})",
            self.name, self.id
        );

        TagParent {
            child_id: self.id,
            parent_id,
        }
        .insert(conn)
        .await
    }

    pub async fn add_parents(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tags: &Vec<Tag>,
    ) -> Result<(), SqlxError> {
        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        for tag in tags {
            self.add_parent(&mut trans, tag.id).await?;
        }

        trans.commit().await.context(SqlxSnafu)?;

        Ok(())
    }
}
