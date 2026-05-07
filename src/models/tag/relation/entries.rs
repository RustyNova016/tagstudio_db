use crate::SqlxError;
use crate::Tag;
use crate::models::tag_entry::TagEntry;

impl Tag {
    pub async fn add_entry(
        &self,
        conn: &mut sqlx::SqliteConnection,
        entry_id: i64,
    ) -> Result<(), SqlxError> {
        TagEntry {
            entry_id,
            tag_id: self.id,
        }
        .insert(conn)
        .await
    }
}
