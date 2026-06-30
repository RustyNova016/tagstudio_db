use sequelles::InsertOrIgnore;

use crate::Entry;
use crate::TextField;
use crate::models::text_field::TextFieldInsert;
use crate::models::text_field::TextFieldSqlError;

impl Entry {
    #[deprecated]
    pub async fn add_text_field(
        &self,
        conn: &mut sqlx::SqliteConnection,
        type_key: &str,
        value: &str,
    ) -> Result<Option<TextField>, TextFieldSqlError> {
        TextFieldInsert::builder()
            .entry_id(self.id)
            .value(value.to_string())
            .name(type_key)
            .is_multiline(false)
            .build()
            .insert_or_ignore(conn)
            .await
    }
}
