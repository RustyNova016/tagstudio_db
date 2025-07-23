use std::path::Path;
use std::path::PathBuf;

use chrono::NaiveDateTime;
use futures::Stream;
use sqlx::Acquire;
use sqlx::FromRow;
use tracing::debug;

use crate::models::folder::Folder;
use crate::models::tag::Tag;
use crate::models::text_field::TextField;
use crate::query::Queryfragments;

#[cfg(feature = "fs")]
pub mod fs;

#[derive(Debug, FromRow)]
pub struct Entry {
    pub id: i64,
    pub folder_id: i64,
    pub path: String,
    pub filename: String,
    pub suffix: String,
    pub date_created: Option<NaiveDateTime>,
    pub date_modified: Option<NaiveDateTime>,
    pub date_added: Option<NaiveDateTime>,
}

impl Entry {
    /// Get the row by its id
    pub async fn find_by_id(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM `entries` WHERE `id` = ?", id)
                .fetch_optional(conn)
                .await?,
        )
    }

    /// Get the row by its path
    pub async fn find_by_path(
        conn: &mut sqlx::SqliteConnection,
        path: &Path,
    ) -> Result<Vec<Self>, crate::Error> {
        let path = path.to_string_lossy();

        Ok(sqlx::query_as!(
            Self,
            "
            SELECT `entries`.* 
            FROM `entries`
            WHERE `entries`.`path` = ?",
            path
        )
        .fetch_all(conn)
        .await?)
    }

    /// Get the entry by its cannon path
    pub async fn find_by_cannon_path(
        conn: &mut sqlx::SqliteConnection,
        path: &Path,
    ) -> Result<Vec<Self>, crate::Error> {
        let path_string = path.to_string_lossy();

        Ok(sqlx::query_as!(
            Self,
            "SELECT `entries`.*
                    FROM `entries`
                        INNER JOIN `folders` ON `folders`.id = `entries`.`folder_id`
                    WHERE
                        `folders`.`path` + '/' + `entries`.`path` = :target_path OR -- UNIX
                        `folders`.`path` + '\\' + `entries`.`path` = :target_path   -- Windows
                    ",
            path_string
        )
        .fetch_all(&mut *conn)
        .await?)
    }

    pub async fn search<'a>(
        conn: &'a mut sqlx::SqliteConnection,
        search: &'a Queryfragments,
    ) -> Result<Vec<Self>, crate::Error> {
        let sql = search.as_sql();
        let query = sqlx::query_as(&sql);
        let query = search.bind(query);
        Ok(query.fetch_all(conn).await?)
    }

    pub fn stream_entries(
        conn: &mut sqlx::SqliteConnection,
    ) -> std::pin::Pin<Box<dyn Stream<Item = Result<Entry, sqlx::Error>> + Send + '_>> {
        sqlx::query_as!(Self, "SELECT * FROM `entries`").fetch(conn)
    }

    pub async fn get_folder(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Folder, crate::Error> {
        Folder::find_by_id(conn, self.id)
            .await
            .transpose()
            .expect("Couldn't find entry's folder")
    }

    /// Get the path of the file on the filesystem
    pub async fn get_global_path(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<PathBuf, crate::Error> {
        let root_path = self.get_folder(&mut *conn).await?.path;
        let mut path = PathBuf::from(root_path);
        path.push(&self.path);
        Ok(path)
    }

    pub async fn get_text_fields(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<TextField>, crate::Error> {
        Ok(sqlx::query_as!(
            TextField,
            "SELECT `text_fields`.* 
            FROM `entries` 
                INNER JOIN `text_fields` ON `text_fields`.`entry_id` = `entries`.`id`
            WHERE
                `entries`.`id` = ?",
            self.id
        )
        .fetch_all(conn)
        .await?)
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

    pub async fn add_tag(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tag_id: i64,
    ) -> Result<(), crate::Error> {
        debug!("Adding tag {tag_id} to entry {}", self.id);
        sqlx::query!(
            "INSERT OR IGNORE INTO `tag_entries`(entry_id, tag_id) VALUES (?, ?)",
            self.id,
            tag_id
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    pub async fn add_tags(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tags: &Vec<Tag>,
    ) -> Result<(), crate::Error> {
        let mut trans = conn.begin().await?;

        for tag in tags {
            self.add_tag(&mut trans, tag.id).await?;
        }

        trans.commit().await?;

        Ok(())
    }
}
