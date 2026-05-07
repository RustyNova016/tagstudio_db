use sqlx::prelude::FromRow;

pub mod delete;
pub mod insert;
pub mod select;

/// An alias of a tag
#[derive(Debug, FromRow, Clone, PartialEq, Eq)]
pub struct TagAlias {
    pub id: i64,
    pub name: String,
    pub tag_id: i64,
}
