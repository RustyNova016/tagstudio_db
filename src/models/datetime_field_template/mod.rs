use sequelles::sqlx::FromRow;

#[derive(Debug, FromRow, Clone, PartialEq, Eq, sequelles::Table)]
#[sequelles(db_name = "datetime_field_templates", snafu)]
#[sequelles(sqlite)]
#[sequelles(update, insert_struct, select)]
#[sequelles(primary_key(key_name = "pk", columns(id)))]
pub struct DatetimeFieldTemplate {
    #[sequelles(auto_increment)]
    pub id: i64,
    pub name: String,
}
