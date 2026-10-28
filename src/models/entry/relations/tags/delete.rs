use crate::Entry;
use crate::SqlxError;
use crate::models::tag_entry::TagEntry;

impl Entry {
    /// Get the tags of the entry
    pub async fn remove_tag(
        &self,
        conn: &mut sqlx::SqliteConnection,
        tag_id: i64,
    ) -> Result<(), SqlxError> {
        TagEntry::delete_by_pk(conn, self.id, tag_id).await
    }
}
