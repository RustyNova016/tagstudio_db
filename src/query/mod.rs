pub mod eq_entry_field;
pub mod eq_folder2;
pub mod eq_entry_id2;
pub mod entry_search_query;
pub mod eq_tag_id;
pub mod tag_search_query;
use std::backtrace::Backtrace;
use std::fmt::Write;

use nom::Finish as _;
use nom_language::error::convert_error;
use snafu::Snafu;
use sqlx::Sqlite;
use sqlx::query::QueryAs;
use sqlx::sqlite::SqliteArguments;

use crate::models::entry::Entry;
use crate::query::and::QueryAnd;
use crate::query::any_tag_id::AnyTagId;
use crate::query::any_tag_string::AnyTagString;
use crate::query::eq_any_entry_id::EqAnyEntryId;
use crate::query::eq_field::EqField;
use crate::query::eq_folder::EqEntryFolder;
use crate::query::eq_tag_string::EqTagString;
use crate::query::not::QueryNot;
use crate::query::or::QueryOr;
use crate::query::parsing::expression::parse_expression;

pub mod and;
pub mod and2;
pub mod any_tag_id;
pub mod any_tag_string;
pub mod entries_with_tags;
pub mod eq_any_entry_id;
pub mod eq_field;
pub mod eq_folder;
pub mod eq_tag_or_children;
pub mod eq_tag_string;
pub mod eq_tag_string2;
pub mod not;
pub mod not2;
pub mod or;
pub mod or2;
pub mod parsing;
pub mod trait_entry_filter;
pub mod trait_tag_filter;

pub type SQLQuery<'q, O> = QueryAs<'q, Sqlite, O, SqliteArguments<'q>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Queryfragments {
    And(Box<QueryAnd>),
    Or(Box<QueryOr>),
    EqEntryId(EqAnyEntryId),
    EqField(EqField),
    Not(Box<QueryNot>),

    // --- Tag Eq ---
    AnyTagString(AnyTagString),
    AnyTagId(AnyTagId),
    EqTag(EqTagString),

    // --- Entry Eq ---
    EqEntryFolder(EqEntryFolder),
}

impl Queryfragments {
    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        match self {
            Self::EqEntryId(val) => val.get_subquery(bind_id),
            Self::EqField(val) => val.get_subquery(bind_id),
            Self::EqTag(val) => val.get_subquery(bind_id),
            Self::And(val) => val.get_subquery(bind_id),
            Self::Or(val) => val.get_subquery(bind_id),
            Self::Not(val) => val.get_subquery(bind_id),
            Self::AnyTagString(val) => val.get_subquery(bind_id),
            Self::AnyTagId(val) => val.get_subquery(bind_id),
            Self::EqEntryFolder(val) => val.get_subquery(bind_id),
        }
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        match self {
            Self::EqEntryId(val) => val.get_where_condition(bind_id),
            Self::EqField(val) => val.get_where_condition(bind_id),
            Self::EqTag(val) => val.get_where_condition(bind_id),
            Self::And(val) => val.get_where_condition(bind_id),
            Self::Or(val) => val.get_where_condition(bind_id),
            Self::Not(val) => val.get_where_condition(bind_id),
            Self::AnyTagString(val) => val.get_where_condition(bind_id),
            Self::AnyTagId(val) => val.get_where_condition(bind_id),
            Self::EqEntryFolder(val) => val.get_where_condition(bind_id),
        }
    }

    pub fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        match self {
            Self::EqEntryId(val) => val.bind(query),
            Self::EqField(val) => val.bind(query),
            Self::EqTag(val) => val.bind(query),
            Self::And(val) => val.bind(query),
            Self::Or(val) => val.bind(query),
            Self::Not(val) => val.bind(query),
            Self::AnyTagString(val) => val.bind(query),
            Self::AnyTagId(val) => val.bind(query),
            Self::EqEntryFolder(val) => val.bind(query),
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

    pub fn and(self, other: Self) -> Self {
        QueryAnd(self, other).into()
    }
}
