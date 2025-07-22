use crate::models::entry::Entry;

pub struct TextField {
    pub value: Option<String>,
    pub id: i64,
    pub type_key: String,
    pub entry_id: i64,
    pub position: i64,
}

impl TextField {
    pub async fn insert_text_field(
        conn: &mut sqlx::SqliteConnection,
        entry_id: i64,
        type_key: String,
        value: String,
    ) -> Result<(), crate::Error> {
        sqlx::query!(
            "INSERT INTO `text_fields` VALUES (?, NULL, ?, ?, 0)",
            value,
            type_key,
            entry_id
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    pub async fn get_entry(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Entry, crate::Error> {
        Entry::find_by_id(conn, self.entry_id)
            .await
            .transpose()
            .expect("The text field has no associated entry")
    }
}
