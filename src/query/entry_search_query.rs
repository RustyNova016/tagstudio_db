use std::backtrace::Backtrace;

use nom::Finish as _;
use nom_language::error::convert_error;
use snafu::Snafu;

use crate::query::and2::QueryAnd2;
use crate::query::entries_with_tags::EntriesWithTags;
use crate::query::not2::QueryNot2;
use crate::query::or2::QueryOr2;
use crate::query::parse_expression;
use crate::query::tag_search_query::TagSearchQuery;
use crate::query::trait_entry_filter::EntryFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntrySearchQuery {
    EntriesWithTags(EntriesWithTags<Box<TagSearchQuery>>),

    Not(QueryNot2<Box<EntrySearchQuery>>),

    And(QueryAnd2<Box<EntrySearchQuery>, Box<EntrySearchQuery>>),
    Or(QueryOr2<Box<EntrySearchQuery>, Box<EntrySearchQuery>>),
}

impl EntryFilter for EntrySearchQuery {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        match self {
            Self::EntriesWithTags(val) => val.get_where_condition(bind_id),
            Self::Not(val) => val.get_where_condition(bind_id),
            Self::And(val) => val.get_where_condition(bind_id),
            Self::Or(val) => val.get_where_condition(bind_id),
        }
    }

    fn bind<'q, O>(&'q self, query: super::SQLQuery<'q, O>) -> super::SQLQuery<'q, O> {
        match self {
            Self::EntriesWithTags(val) => val.bind(query),
            Self::Not(val) => val.bind(query),
            Self::And(val) => val.bind(query),
            Self::Or(val) => val.bind(query),
        }
    }
}

impl EntrySearchQuery {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn and(self, other: Self) -> Self {
        Self::And(QueryAnd2(self.boxed(), other.boxed()))
    }

    pub fn or(self, other: Self) -> Self {
        Self::Or(QueryOr2(self.boxed(), other.boxed()))
    }

    pub fn not(self) -> Self {
        Self::Not(QueryNot2(self.boxed()))
    }

    pub fn parse(input: &str) -> Result<Self, InvalidSearchString> {
        parse_expression(input)
            .finish()
            .map(|(_, res)| res)
            .map_err(|err| InvalidSearchString {
                nom_trace: convert_error(input, err),
                backtrace: Backtrace::capture(),
            })
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("Couldn't parse the search query. Search trace: \n{nom_trace}"))]
pub struct InvalidSearchString {
    pub nom_trace: String,
    backtrace: Backtrace,
}
