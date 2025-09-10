use core::fmt::Display;

use futures::stream::BoxStream;
use sqlx::Acquire;
use sqlx::FromRow;
use streamies::TryStreamies as _;
use tracing::debug;

use crate::models::alias::TagAlias;
use crate::query::eq_tag_id::EqTagId;
use crate::query::trait_entry_filter::EntryFilter as _;
use crate::query::trait_tag_filter::TagFilter as _;

pub mod find;

#[derive(Debug, FromRow, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub shorthand: Option<String>,
    pub color_namespace: Option<String>,
    pub color_slug: Option<String>,
    pub is_category: bool,
    pub icon: Option<String>,
    pub disambiguation_id: Option<i64>,
}

impl Tag {
    /// Insert a new tag in the database
    pub async fn insert_tag(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Self, crate::Error> {
        debug!("Adding tag `{}`", self.name);

        Ok(sqlx::query_as!(
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
        .await?)
    }

    pub async fn get_by_name_or_insert_new(
        conn: &mut sqlx::SqliteConnection,
        name: &str,
    ) -> Result<Vec<Self>, crate::Error> {
        let tags = Self::find_tag_by_name(conn, name).await?;

        if !tags.is_empty() {
            return Ok(tags);
        }

        let tag = Self::from(name).insert_tag(conn).await?;
        Ok(vec![tag])
    }

    /// Rename the current tag. If not disabled, the old name will be added as an alias
    ///
    /// `self` is not mutated unless the result is `Ok`. So it's safe to use, even after getting an `Err`
    pub async fn rename(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        new_name: &str,
        no_aliasing: bool,
    ) -> Result<(), crate::Error> {
        let mut trans = conn.begin().await?;

        if !no_aliasing {
            self.add_alias(&mut trans, new_name).await?;
        }

        sqlx::query!(
            "UPDATE `tags` SET name = $name WHERE id = $id",
            new_name,
            self.id
        )
        .execute(&mut *trans)
        .await?;

        trans.commit().await?;

        self.name = new_name.to_string();
        Ok(())
    }

    /// Merge a tag into self
    pub async fn merge_tag(
        &self,
        conn: &mut sqlx::SqliteConnection,
        other: Self,
    ) -> Result<(), crate::Error> {
        let mut trans = conn.begin().await?;

        // Add the new tag to the entries with the old tag
        let entries = EqTagId(other.id)
            .into_entry_filter()
            .fetch_all(&mut trans)
            .await?;
        for entry in entries {
            entry.add_tag(&mut trans, self.id).await?;
        }

        // Merge the tag data
        self.add_alias(&mut trans, &other.name).await?;
        self.add_alias(&mut trans, &other.shorthand.clone().unwrap_or_default())
            .await?;

        let aliases = other.get_aliases(&mut trans).try_collect_vec().await?;
        for alias in aliases {
            self.add_alias(&mut trans, &alias.name).await?;
        }

        let parents = other.get_parents(&mut trans).try_collect_vec().await?;
        for parent in parents {
            self.add_parent(&mut trans, parent.id).await?;
        }

        let children = other.get_children(&mut trans).try_collect_vec().await?;
        for child in children {
            self.add_child(&mut trans, child.id).await?;
        }

        other.delete(&mut trans).await?;

        trans.commit().await?;
        Ok(())
    }

    pub fn get_aliases<'l>(
        &'l self,
        conn: &'l mut sqlx::SqliteConnection,
    ) -> BoxStream<'l, Result<TagAlias, sqlx::Error>> {
        sqlx::query_as!(
            TagAlias,
            "
            SELECT `tag_aliases`.* 
            FROM `tags` 
                INNER JOIN `tag_aliases` ON `tag_aliases`.`tag_id` = `tags`.`id`
            WHERE `tags`.`id` = $1",
            self.id
        )
        .fetch(conn)
    }

    pub fn get_parents<'l>(
        &'l self,
        conn: &'l mut sqlx::SqliteConnection,
    ) -> BoxStream<'l, Result<Tag, sqlx::Error>> {
        sqlx::query_as!(
            Tag,
            "
            SELECT `tags`.* 
            FROM `tags` 
                INNER JOIN `tag_parents` ON `tag_parents`.`parent_id` = `tags`.`id`
            WHERE `tag_parents`.`child_id` = $1",
            self.id
        )
        .fetch(conn)
    }

    pub fn get_children<'l>(
        &'l self,
        conn: &'l mut sqlx::SqliteConnection,
    ) -> BoxStream<'l, Result<Tag, sqlx::Error>> {
        sqlx::query_as!(
            Tag,
            "
            SELECT `tags`.* 
            FROM `tags` 
                INNER JOIN `tag_parents` ON `tag_parents`.`child_id` = `tags`.`id`
            WHERE `tag_parents`.`parent_id` = $1",
            self.id
        )
        .fetch(conn)
    }

    /// Add an alias to this tag.
    ///
    /// This doesn't duplicate aliases, and prevent adding an alias identical to the name
    pub async fn add_alias(
        &self,
        conn: &mut sqlx::SqliteConnection,
        name: &str,
    ) -> Result<(), crate::Error> {
        if self.name == name {
            debug!("Ignoring alias addition {name}");
            return Ok(());
        }

        TagAlias::insert_tag_alias(conn, name, self.id).await
    }

    pub async fn add_parent(
        &self,
        conn: &mut sqlx::SqliteConnection,
        parent_id: i64,
    ) -> Result<(), crate::Error> {
        debug!(
            "Adding parent `{parent_id}` to tag `{}` ({})",
            self.name, self.id
        );

        sqlx::query!(
            "INSERT OR IGNORE INTO `tag_parents`(parent_id, child_id) VALUES (?, ?)",
            parent_id,
            self.id,
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    pub async fn add_parents(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tags: &Vec<Tag>,
    ) -> Result<(), crate::Error> {
        let mut trans = conn.begin().await?;

        for tag in tags {
            self.add_parent(&mut trans, tag.id).await?;
        }

        trans.commit().await?;

        Ok(())
    }

    pub async fn add_child(
        &self,
        conn: &mut sqlx::SqliteConnection,
        child_id: i64,
    ) -> Result<(), crate::Error> {
        debug!(
            "Adding child `{child_id}` to tag `{}` ({})",
            self.name, self.id
        );

        sqlx::query!(
            "INSERT OR IGNORE INTO `tag_parents`(parent_id, child_id) VALUES (?, ?)",
            self.id,
            child_id,
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    pub async fn add_children(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tags: &Vec<Tag>,
    ) -> Result<(), crate::Error> {
        let mut trans = conn.begin().await?;

        for tag in tags {
            self.add_child(&mut trans, tag.id).await?;
        }

        trans.commit().await?;

        Ok(())
    }

    pub async fn delete(self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let mut trans = conn.begin().await?;

        sqlx::query!("DELETE FROM `tag_aliases` WHERE `tag_id` = $1", self.id)
            .execute(&mut *trans)
            .await?;
        sqlx::query!("DELETE FROM `tag_entries` WHERE `tag_id` = $1", self.id)
            .execute(&mut *trans)
            .await?;
        sqlx::query!(
            "DELETE FROM `tag_parents` WHERE `parent_id` = $1 OR `child_id` = $1",
            self.id
        )
        .execute(&mut *trans)
        .await?;
        sqlx::query!("DELETE FROM `tags` WHERE `id` = $1", self.id)
            .execute(&mut *trans)
            .await?;

        trans.commit().await?;
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
