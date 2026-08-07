use core::fmt::Display;

use snafu::ResultExt as _;
use sqlx::Acquire;
use sqlx::FromRow;
use streamies::TryStreamies as _;

use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag::error::TagError;
use crate::models::tag::error::TagSQLxSnafu;
use crate::models::tag::error::TransactionSnafu;
use crate::query::eq_tag_id::EqTagId;
use crate::query::trait_entry_filter::QueryEntryFilter as _;
use crate::query::trait_tag_filter::TagFilter as _;

pub mod delete;
pub mod error;
pub mod find;
pub mod insert;
pub mod relation;
pub mod update;

#[derive(Debug, FromRow, Clone, PartialEq, Eq)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub shorthand: Option<String>,
    pub color_namespace: Option<String>,
    pub color_slug: Option<String>,
    pub is_category: bool,
    pub is_hidden: bool,
    pub icon: Option<String>,
    pub disambiguation_id: Option<i64>,
}

impl Tag {
    /// Rename the current tag. If not disabled, the old name will be added as an alias
    ///
    /// `self` is not mutated unless the result is `Ok`. So it's safe to use, even after getting an `Err`
    pub async fn rename(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        new_name: &str,
        no_aliasing: bool,
    ) -> Result<(), SqlxError> {
        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        if !no_aliasing {
            self.add_alias(&mut trans, new_name).await?;
        }

        self.name = new_name.to_string();
        self.update(&mut trans).await?;

        trans.commit().await.context(SqlxSnafu)?;
        Ok(())
    }

    /// Merge a tag into self
    pub async fn merge_tag(
        &self,
        conn: &mut sqlx::SqliteConnection,
        other: Self,
    ) -> Result<(), TagError> {
        let mut trans = conn.begin().await.context(TransactionSnafu)?;

        // Add the new tag to the entries with the old tag
        let entries = EqTagId(other.id)
            .into_entry_filter()
            .fetch_all(&mut trans)
            .await
            .context(TagSQLxSnafu)?;
        for entry in entries {
            entry
                .add_tag_id(&mut trans, self.id)
                .await
                .context(TagSQLxSnafu)?;
        }

        // Merge the tag data
        self.add_alias(&mut trans, &other.name)
            .await
            .context(TagSQLxSnafu)?;
        self.add_alias(&mut trans, &other.shorthand.clone().unwrap_or_default())
            .await
            .context(TagSQLxSnafu)?;

        let aliases = other
            .get_aliases(&mut trans)
            .try_collect_vec()
            .await
            .context(TagSQLxSnafu)?;
        for alias in aliases {
            self.add_alias(&mut trans, &alias.name)
                .await
                .context(TagSQLxSnafu)?;
        }

        let parents = other
            .get_parents(&mut trans)
            .try_collect_vec()
            .await
            .context(TagSQLxSnafu)?;
        self.add_parents(&mut trans, &parents).await?;

        let children = other
            .get_children(&mut trans)
            .try_collect_vec()
            .await
            .context(TagSQLxSnafu)?;
        self.add_children(&mut trans, &children).await?;

        other.delete(&mut trans).await.context(TagSQLxSnafu)?;

        trans.commit().await.context(TransactionSnafu)?;
        Ok(())
    }
}

impl<T: Display> From<T> for Tag {
    fn from(value: T) -> Self {
        Self {
            color_namespace: None,
            color_slug: None,
            disambiguation_id: None,
            icon: None,
            id: 0,
            is_category: false,
            name: value.to_string(),
            shorthand: None,
            is_hidden: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::TryStreamExt;

    use crate::tests::fixtures::raw_library::get_empty_library;

    use super::*;

    #[tokio::test]
    pub async fn parenting_tests() {
        let lib = get_empty_library().await;
        let conn = &mut *lib.db.get().await.unwrap();

        let cat = Tag::from("cat").insert_tag(conn).await.unwrap();
        let maxwell = Tag::from("Maxwell").insert_tag(conn).await.unwrap();
        let feline = Tag::from("Feline").insert_tag(conn).await.unwrap();

        cat.add_child(conn, maxwell.id).await.unwrap();
        cat.add_parent(conn, feline.id).await.unwrap();

        assert!(
            cat.get_children(conn)
                .try_any(async |tag| tag.name == "Maxwell")
                .await
                .unwrap()
        );

        assert!(
            cat.get_parents(conn)
                .try_any(async |tag| tag.name == "Feline")
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    pub async fn merge_test() {
        let lib = get_empty_library().await;
        let conn = &mut *lib.db.get().await.unwrap();

        let cat1 = Tag::from("cat").insert_tag(conn).await.unwrap();
        let cat2 = Tag::from("Felix Catus").insert_tag(conn).await.unwrap();
        let feline = Tag::from("Feline").insert_tag(conn).await.unwrap();
        let maxwell = Tag::from("Maxwell").insert_tag(conn).await.unwrap();

        let cat2_id = cat2.id;
        cat2.add_alias(conn, "Chat").await.unwrap();
        cat2.add_child(conn, maxwell.id).await.unwrap();
        cat2.add_parent(conn, feline.id).await.unwrap();

        cat1.merge_tag(conn, cat2).await.unwrap();

        assert!(Tag::find_by_id(conn, cat2_id).await.unwrap().is_none());
        assert!(
            cat1.get_aliases(conn)
                .try_any(async |alias| alias.name == "Chat")
                .await
                .unwrap()
        );
        assert!(
            cat1.get_children(conn)
                .try_any(async |tag| tag.name == "Maxwell")
                .await
                .unwrap()
        );
        assert!(
            cat1.get_parents(conn)
                .try_any(async |tag| tag.name == "Feline")
                .await
                .unwrap()
        );
    }
}
