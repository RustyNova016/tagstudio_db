use sqlx::Sqlite;
use sqlx::query::QueryAs;
use sqlx::sqlite::SqliteArguments;

use crate::models::entry::Entry;
use crate::query::and::TagAnd;
use crate::query::tag_eq::TagEq;
use std::fmt::Write;

pub mod and;
pub mod tag_eq;

pub type SQLQuery<'q> = QueryAs<'q, Sqlite, Entry, SqliteArguments<'q>>;

pub enum Queryfragments {
    Eq(TagEq),
    And(Box<TagAnd>),
}

impl Queryfragments {
    pub fn get_subquery(&self, id: &str) -> String {
        match self {
            Self::Eq(val) => val.get_subquery(id),
            Self::And(val) => val.get_subquery(id),
        }
    }

    pub fn get_where_condition(&self, id: &str) -> String {
        match self {
            Self::Eq(val) => val.get_where_condition(id),
            Self::And(val) => val.get_where_condition(id),
        }
    }

    pub fn bind<'q>(&'q self, query: SQLQuery<'q>) -> SQLQuery<'q> {
        match self {
            Self::Eq(val) => val.bind(query),
            Self::And(val) => val.bind(query),
        }
    }

    pub fn as_sql(&self) -> String {
        let mut query = String::new();

        let subqueries = self.get_subquery("");

        if !subqueries.is_empty() {
            writeln!(query, "WITH RECURSIVE {subqueries}").unwrap();
        }

        let where_condition = self.get_where_condition("");

        format!(
            "WITH RECURSIVE
                {subqueries}
            SELECT DISTINCT
                `entries`.*
            FROM
                `entries`
            WHERE
                {where_condition}"
        )
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::and::TagAnd;
    use crate::query::tag_eq::TagEq;

    #[test]
    pub fn test() {
        let tag_a = TagEq::from("cat");
        let tag_b = TagEq::from("Mouse");
        let and = TagAnd(tag_a.into(), tag_b.into());

        let query: Queryfragments = and.into();

        println!("{}", query.as_sql())
    }
}
