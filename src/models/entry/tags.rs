use crate::Entry;
use crate::Tag;
use crate::query::and::QueryAnd;
use crate::query::eq_entry_id::EqEntryId;
use crate::query::eq_tag_string::EqTagString;
use crate::query::trait_entry_filter::EntryFilter as _;
use crate::query::trait_tag_filter::TagFilter as _;

impl Entry {
    /// Return true if the entry has the exact tag provided. This also checks the tag aliases and shorthand
    pub async fn match_exact_tag(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tag: &str,
    ) -> Result<bool, crate::Error> {
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
    ) -> Result<bool, crate::Error> {
        let search = QueryAnd(
            EqEntryId(self.id),
            EqTagString::from(tag)
                .add_children_tags()
                .into_entry_filter(),
        );

        Ok(!search.fetch_all(conn).await?.is_empty())
    }

    pub async fn get_tags(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<Tag>, crate::Error> {
        Ok(sqlx::query_as!(
            Tag,
            "SELECT `tags`.* 
            FROM `entries` 
                INNER JOIN `tag_entries` ON `tag_entries`.`entry_id` = `entries`.`id`
                INNER JOIN `tags` ON `tag_entries`.`tag_id` = `tags`.`id`
            WHERE
                `entries`.`id` = ?",
            self.id
        )
        .fetch_all(conn)
        .await?)
    }

    pub async fn get_tags_and_parents(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<Tag>, crate::Error> {
        Ok(sqlx::query_as!(
            Tag,
            "SELECT `tags`.* 
            FROM `entries` 
                INNER JOIN `tag_entries` ON `tag_entries`.`entry_id` = `entries`.`id`
                INNER JOIN `tags` ON `tag_entries`.`tag_id` = `tags`.`id`
            WHERE
                `entries`.`id` = ?",
            self.id
        )
        .fetch_all(conn)
        .await?)
    }
}
