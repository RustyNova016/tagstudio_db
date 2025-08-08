use tracing::debug;

use crate::models::tag::Tag;

impl Tag {
    /// Get the row by its id
    pub async fn find_by_id(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM `tags` WHERE `id` = $1", id)
                .fetch_optional(conn)
                .await?,
        )
    }

    /// Get the tag by its exact name
    pub async fn find_by_exact_name(
        conn: &mut sqlx::SqliteConnection,
        name: &str,
    ) -> Result<Vec<Self>, crate::Error> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM `tags` WHERE `name` = $1", name)
                .fetch_all(conn)
                .await?,
        )
    }

    /// Get all the tags that match a string. This means any tag that have the same name, shorthand, or alias
    pub async fn find_tag_by_name(
        conn: &mut sqlx::SqliteConnection,
        name: &str,
    ) -> Result<Vec<Tag>, crate::Error> {
        debug!("Searching tag `{name}` by name");

        Ok(sqlx::query_as!(Tag, "
            SELECT `tags`.* 
            FROM
                `tags`
                LEFT JOIN `tag_aliases` ON `tags`.`id` = `tag_aliases`.`tag_id`
            WHERE
                LOWER(`tags`.`name`) = LOWER($1) OR -- Try finding by name
                LOWER(`tags`.`name`) = replace(LOWER($1), '_', ' ') OR -- Try finding by name excaped
                LOWER(`tags`.`shorthand`) = LOWER($1) OR -- Try finding by shorthand
                LOWER(`tags`.`shorthand`) = replace(LOWER($1), '_', ' ') OR -- Try finding by shorthand excaped
                LOWER(`tag_aliases`.`name`) = LOWER($1) OR -- Try finding by aliased name
                LOWER(`tag_aliases`.`name`) = replace(LOWER($1), '_', ' ') -- Try finding by aliased name excaped
        ", name).fetch_all(conn).await?)
    }
}
