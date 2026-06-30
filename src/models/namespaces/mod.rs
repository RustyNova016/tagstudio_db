use sequelles::sqlx::FromRow;

#[derive(Debug, FromRow, Clone, PartialEq, Eq, sequelles::Table)]
#[sequelles(db_name = "namespaces", snafu)]
#[sequelles(sqlite)]
#[sequelles(update, insert_struct, select)]
#[sequelles(primary_key(key_name = "pk", columns(namespace)))]
pub struct Namespace {
    #[sequelles(auto_increment)]
    pub namespace: String,
    pub name: String,
}
