use snafu::ResultExt;
use sqlx::Acquire;

use crate::Entry;
use crate::Tag;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;
use crate::models::tag_entry::TagEntry;
use crate::query::and::QueryAnd;
use crate::query::eq_entry_id::EqEntryId;
use crate::query::eq_tag_string::EqTagString;
use crate::query::trait_entry_filter::EntryFilter as _;
use crate::query::trait_tag_filter::TagFilter as _;

impl Entry {
    /// Add a tag to the entry using its id
    pub async fn add_tag_id(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tag_id: i64,
    ) -> Result<(), SqlxError> {
        TagEntry {
            entry_id: self.id,
            tag_id,
        }
        .insert(conn)
        .await
    }

    /// Add a tag to the entry
    pub async fn add_tag(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tag: &Tag,
    ) -> Result<(), SqlxError> {
        TagEntry {
            entry_id: self.id,
            tag_id: tag.id,
        }
        .insert(conn)
        .await
    }

    /// Add multiple tags to the entry using their ids
    pub async fn add_tag_ids(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tags: &Vec<Tag>,
    ) -> Result<(), SqlxError> {
        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        for tag in tags {
            self.add_tag_id(&mut trans, tag.id).await?;
        }

        trans.commit().await.context(SqlxSnafu)?;

        Ok(())
    }

    /// Add multiple tags to the entry
    pub async fn add_tags(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tags: &Vec<Tag>,
    ) -> Result<(), SqlxError> {
        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        for tag in tags {
            self.add_tag(&mut trans, tag).await?;
        }

        trans.commit().await.context(SqlxSnafu)?;

        Ok(())
    }

    /// Return true if the entry has the exact tag provided. This also checks the tag aliases and shorthand
    pub async fn match_exact_tag(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tag: &str,
    ) -> Result<bool, SqlxError> {
        let search = QueryAnd(
            EqEntryId(self.id),
            EqTagString::from(tag).into_entry_filter(),
        );

        Ok(!search.fetch_all(conn).await?.is_empty())
    }

    /// Return true if the entry has the tag provided, or has a tag that is a children of the tag provided. This also checks the tag aliases and shorthand
    pub async fn match_tag_or_child_of_tag(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tag: &str,
    ) -> Result<bool, SqlxError> {
        let search = QueryAnd(
            EqEntryId(self.id),
            EqTagString::from(tag)
                .add_children_tags()
                .into_entry_filter(),
        );

        Ok(!search.fetch_all(conn).await?.is_empty())
    }

    /// Add an existing tag with this name or insert a new one and add it to the entry
    pub async fn add_tag_string_or_insert(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tag: String,
    ) -> Result<(), SqlxError> {
        let tag = Tag::get_by_name_or_insert_new(conn, tag).await?;

        self.add_tags(conn, &tag).await
    }
}
