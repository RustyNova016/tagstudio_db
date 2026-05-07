use sqlx::prelude::FromRow;

pub mod delete;
pub mod insert;

#[derive(Debug, FromRow, Clone)]
pub struct TagParent {
    pub parent_id: i64,
    pub child_id: i64,
}
