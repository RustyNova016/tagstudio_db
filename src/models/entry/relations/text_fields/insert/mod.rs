use crate::Entry;
use crate::SqlxError;
use crate::TextField;

impl Entry {
    pub async fn add_text_field(
        &self,
        conn: &mut sqlx::SqliteConnection,
        type_key: &str,
        value: &str,
    ) -> Result<TextField, SqlxError> {
        TextField {
            entry_id: self.id,
            id: 0,
            position: 0,
            type_key: type_key.to_string(),
            value: Some(value.to_string()),
        }
        .insert(conn)
        .await
    }
}
