use crate::Entry;
use crate::SqlxError;
use crate::TextField;

impl Entry {
    pub async fn get_text_fields(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<TextField>, SqlxError> {
        TextField::find_by_entry(conn, self.id).await
    }
}
