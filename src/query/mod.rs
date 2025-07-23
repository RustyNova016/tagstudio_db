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
    pub fn get_subquery(&self, bind_id: &mut u64) -> String {
        match self {
            Self::Eq(val) => val.get_subquery(*bind_id),
            Self::And(val) => val.get_subquery(bind_id),
        }
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> String {
        match self {
            Self::Eq(val) => val.get_where_condition(*bind_id),
            Self::And(val) => val.get_where_condition(bind_id),
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

        let subqueries = self.get_subquery(&mut 1);

        if !subqueries.is_empty() {
            writeln!(query, "WITH RECURSIVE {subqueries}").unwrap();
        }

        writeln!(
            query,
            "SELECT DISTINCT
                `entries`.*
            FROM
                `entries`"
        )
        .unwrap();

        let where_condition = self.get_where_condition(&mut 1);

        if !where_condition.is_empty() {
            writeln!(query, "WHERE {where_condition}").unwrap();
        }

        query
    }

    pub async fn fetch_all(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<Entry>, sqlx::Error> {
        let sql = self.as_sql();
        eprintln!("{sql}");
        let query = sqlx::query_as(&sql);
        self.bind(query).fetch_all(conn).await
    }
}
