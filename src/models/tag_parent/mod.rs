use sqlx::prelude::FromRow;

pub mod delete;

#[derive(Debug, FromRow, Clone, sequelles::Table)]
#[sequelles(sqlite, db_name = "tag_parents", snafu)]
#[sequelles(insert)]
pub struct TagParent {
    pub parent_id: i64,
    pub child_id: i64,
}
