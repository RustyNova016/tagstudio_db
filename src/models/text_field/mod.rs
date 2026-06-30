use sqlx::prelude::FromRow;

use crate::models::entry::Entry;
use crate::models::errors::sqlx_error::SqlxError;

pub mod select;
pub mod update;

#[derive(Debug, FromRow, Clone, PartialEq, Eq, sequelles::Table)]
#[sequelles(db_name = "text_field", snafu)]
#[sequelles(sqlite)]
#[sequelles(update, insert_struct, select, delete)]
#[sequelles(primary_key(key_name = "pk", columns(id)))]
pub struct TextField {
    #[sequelles(auto_increment)]
    pub id: i64,
    pub name: String,
    pub entry_id: i64,
    pub value: Option<String>,
    pub is_multiline: bool,
}

impl TextField {
    pub async fn get_entry(&self, conn: &mut sqlx::SqliteConnection) -> Result<Entry, SqlxError> {
        Entry::find_by_id(conn, self.entry_id)
            .await
            .transpose()
            .expect("The text field has no associated entry")
    }
}
