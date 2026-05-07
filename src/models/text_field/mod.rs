use sqlx::prelude::FromRow;

use crate::models::entry::Entry;
use crate::models::errors::sqlx_error::SqlxError;

pub mod insert;
pub mod select;
pub mod update;

#[derive(Debug, FromRow, Clone, PartialEq, Eq)]
pub struct TextField {
    pub value: Option<String>,
    pub id: i64,
    pub type_key: String,
    pub entry_id: i64,
    pub position: i64,
}

impl TextField {
    pub async fn get_entry(&self, conn: &mut sqlx::SqliteConnection) -> Result<Entry, SqlxError> {
        Entry::find_by_id(conn, self.entry_id)
            .await
            .transpose()
            .expect("The text field has no associated entry")
    }
}
