use core::str::FromStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use chrono::NaiveDateTime;
use sqlx::Acquire;
use sqlx::FromRow;
use tracing::debug;

use crate::models::folder::Folder;
use crate::models::tag::Tag;
use crate::models::text_field::TextField;

pub mod reads;
pub mod tags;

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
    pub async fn insert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        debug!("Adding entry `{}`", self.path);

        Ok(sqlx::query_as!(
            Self,
            "INSERT INTO `entries` VALUES (NULL, ?, ?, ?, ?, ?, ?, ?) RETURNING *;",
            self.folder_id,
            self.path,
            self.filename,
            self.suffix,
            self.date_created,
            self.date_modified,
            self.date_added
        )
        .fetch_one(conn)
        .await?)
    }

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
        path: &str,
    ) -> Result<Vec<Self>, crate::Error> {
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

    /// Get the entry by its cannon path (Aka, the library's root path + the file's path in the library)
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
                        `folders`.`path` + '/' + `entries`.`path` = $1 OR -- UNIX
                        `folders`.`path` + '\\' + `entries`.`path` = $1   -- Windows
                    ",
            path_string
        )
        .fetch_all(&mut *conn)
        .await?)
    }

    pub async fn get_folder(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Folder, crate::Error> {
        Folder::find_by_id(conn, self.folder_id)
            .await
            .transpose()
            .unwrap_or_else(|| panic!("Couldn't find entry's folder! Something went horribly wrong, as every entries should have their own folder. Tried to get folder id: {}", self.id))
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

    /// Get the relative path of the file
    pub fn get_relative_path(&self) -> PathBuf {
        PathBuf::from_str(&self.path).unwrap()
    }

    /// Get the filename of the file. This is more secured than the inner `filename` field as it takes it from the path
    pub fn get_filename(&self) -> Option<OsString> {
        self.get_relative_path()
            .file_name()
            .map(|f| f.to_os_string())
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

    pub async fn add_text_field(
        &self,
        conn: &mut sqlx::SqliteConnection,
        type_key: &str,
        value: &str,
    ) -> Result<(), crate::Error> {
        TextField::insert_text_field(conn, self.id, type_key, value).await
    }
}
