use crate::Entry;
use crate::Tag;
use crate::query::Queryfragments;
use crate::query::eq_entry_id::EqEntryId;
use crate::query::eq_tag::EqTag;

impl Entry {
    /// Return true if the entry has a tag, tag parent or alias that match the input string
    pub async fn match_tag(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tag: &str,
    ) -> Result<bool, crate::Error> {
        let search = Queryfragments::EqTag(EqTag::from(tag)).and(EqEntryId::new(self.id).into());
        let sql = search.as_sql();
        let query = sqlx::query_as(&sql);
        let query = search.bind(query);

        Ok(query.fetch_optional(conn).await?.is_some())
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
