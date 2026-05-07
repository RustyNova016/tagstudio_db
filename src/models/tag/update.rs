use snafu::ResultExt;

use crate::SqlxError;
use crate::Tag;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl Tag {
    pub async fn update(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query!(
            sql = "
            UPDATE `tags` SET 
                `name` = {self.name},
                `shorthand` = {self.shorthand},
                `color_namespace` = {self.color_namespace},
                `color_slug` = {self.color_slug},
                `is_hidden` = {self.is_hidden},
                `is_category` = {self.is_category},
                `icon` = {self.icon},
                `disambiguation_id` = {self.disambiguation_id}
        "
        )
        .execute(&mut *conn)
        .await
        .context(SqlxSnafu)?;

        Ok(())
    }
}
