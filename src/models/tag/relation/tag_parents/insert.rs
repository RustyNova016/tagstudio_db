use sequelles::InsertOrIgnore;
use snafu::ResultExt;
use sqlx::Acquire;
use tracing::debug;

use crate::Tag;
use crate::models::tag::error::TagError;
use crate::models::tag::error::TagParentSnafu;
use crate::models::tag::error::TransactionSnafu;
use crate::models::tag_parent::TagParent;
use crate::models::tag_parent::TagParentSqlError;

impl Tag {
    /// Add a child tag to this tag
    pub async fn add_child(
        &self,
        conn: &mut sqlx::SqliteConnection,
        child_id: i64,
    ) -> Result<Option<TagParent>, TagParentSqlError> {
        debug!(
            "Adding child `{child_id}` to tag `{}` ({})",
            self.name, self.id
        );

        TagParent {
            child_id,
            parent_id: self.id,
        }
        .insert_or_ignore(conn)
        .await
    }

    pub async fn add_children(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tags: &Vec<Tag>,
    ) -> Result<(), TagError> {
        let mut trans = conn.begin().await.context(TransactionSnafu)?;

        for tag in tags {
            self.add_child(&mut trans, tag.id)
                .await
                .context(TagParentSnafu)?;
        }

        trans.commit().await.context(TransactionSnafu)?;

        Ok(())
    }

    pub async fn add_parent(
        &self,
        conn: &mut sqlx::SqliteConnection,
        parent_id: i64,
    ) -> Result<Option<TagParent>, TagParentSqlError> {
        debug!(
            "Adding parent `{parent_id}` to tag `{}` ({})",
            self.name, self.id
        );

        TagParent {
            child_id: self.id,
            parent_id,
        }
        .insert_or_ignore(conn)
        .await
    }

    pub async fn add_parents(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tags: &Vec<Tag>,
    ) -> Result<(), TagError> {
        let mut trans = conn.begin().await.context(TransactionSnafu)?;

        for tag in tags {
            self.add_parent(&mut trans, tag.id)
                .await
                .context(TagParentSnafu)?;
        }

        trans.commit().await.context(TransactionSnafu)?;

        Ok(())
    }
}
