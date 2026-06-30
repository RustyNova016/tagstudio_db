use sequelles::sqlx::FromRow;

#[derive(Debug, FromRow, Clone, PartialEq, Eq, sequelles::Table)]
#[sequelles(db_name = "text_field_templates", snafu)]
#[sequelles(sqlite)]
#[sequelles(update, insert_struct, select)]
#[sequelles(primary_key(key_name = "pk", columns(id)))]
pub struct TextFieldTemplate {
    #[sequelles(auto_increment)]
    pub id: i64,
    pub name: String,
    pub is_multiline: bool,
}
