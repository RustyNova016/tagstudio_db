use sqlx::Sqlite;
use sqlx::query::QueryAs;
use sqlx::sqlite::SqliteArguments;

use crate::models::entry::Entry;
use crate::query::and::QueryAnd;
use crate::query::eq_field::EqField;
use crate::query::eq_tag::EqTag;
use crate::query::not::QueryNot;
use std::fmt::Write;

pub mod and;
pub mod eq_field;
pub mod eq_tag;
pub mod not;

pub type SQLQuery<'q> = QueryAs<'q, Sqlite, Entry, SqliteArguments<'q>>;

pub enum Queryfragments {
    And(Box<QueryAnd>),
    EqField(EqField),
    EqTag(EqTag),
    Not(Box<QueryNot>),
}

impl Queryfragments {
    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        match self {
            Self::EqField(val) => val.get_subquery(bind_id),
            Self::EqTag(val) => val.get_subquery(bind_id),
            Self::And(val) => val.get_subquery(bind_id),
            Self::Not(val) => val.get_subquery(bind_id),
        }
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        match self {
            Self::EqField(val) => val.get_where_condition(bind_id),
            Self::EqTag(val) => val.get_where_condition(bind_id),
            Self::And(val) => val.get_where_condition(bind_id),
            Self::Not(val) => val.get_where_condition(bind_id),
        }
    }

    pub fn bind<'q>(&'q self, query: SQLQuery<'q>) -> SQLQuery<'q> {
        match self {
            Self::EqField(val) => val.bind(query),
            Self::EqTag(val) => val.bind(query),
            Self::And(val) => val.bind(query),
            Self::Not(val) => val.bind(query),
        }
    }

    pub fn as_sql(&self) -> String {
        let mut query = String::new();

        if let Some(subqueries) = self.get_subquery(&mut 1) {
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

        if let Some(where_condition) = self.get_where_condition(&mut 1) {
            writeln!(query, "WHERE {where_condition}").unwrap();
        }

        query
    }

    pub async fn fetch_all(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<Entry>, sqlx::Error> {
        let sql = self.as_sql();
        let query = sqlx::query_as(&sql);
        self.bind(query).fetch_all(conn).await
    }
}
