use core::fmt::Display;

use crate::query::and2::QueryAnd2;
use crate::query::entries_with_tags::EntriesWithTags;
use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::eq_tag_id::EqTagId;
use crate::query::eq_tag_or_children::EqTagOrChildren;
use crate::query::eq_tag_string2::EqTagString2;
use crate::query::not2::QueryNot2;
use crate::query::or2::QueryOr2;
use crate::query::trait_tag_filter::TagFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TagSearchQuery {
    EqTagId(EqTagId),
    EqTagString(EqTagString2),

    EqTagOrChildren(EqTagOrChildren<Box<TagSearchQuery>>),
    Not(QueryNot2<Box<TagSearchQuery>>),

    And(QueryAnd2<Box<TagSearchQuery>, Box<TagSearchQuery>>),
    Or(QueryOr2<Box<TagSearchQuery>, Box<TagSearchQuery>>),
}

impl TagFilter for TagSearchQuery {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        match self {
            Self::EqTagId(val) => val.get_where_condition(bind_id),
            Self::EqTagString(val) => val.get_where_condition(bind_id),
            Self::EqTagOrChildren(val) => val.get_where_condition(bind_id),
            Self::Not(val) => val.get_where_condition(bind_id),
            Self::And(val) => val.get_where_condition(bind_id),
            Self::Or(val) => val.get_where_condition(bind_id),
        }
    }

    fn bind<'q, O>(&'q self, query: super::SQLQuery<'q, O>) -> super::SQLQuery<'q, O> {
        match self {
            Self::EqTagId(val) => val.bind(query),
            Self::EqTagString(val) => val.bind(query),
            Self::EqTagOrChildren(val) => val.bind(query),
            Self::Not(val) => val.bind(query),
            Self::And(val) => val.bind(query),
            Self::Or(val) => val.bind(query),
        }
    }
}

impl TagSearchQuery {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn eq_tag_string<T: Display>(value: T) -> Self {
        Self::EqTagString(EqTagString2::from(value))
    }

    pub fn add_children_tags_opaque(self) -> Self {
        Self::EqTagOrChildren(EqTagOrChildren(self.boxed()))
    }

    pub fn into_entry_search_query(self) -> EntrySearchQuery {
        EntrySearchQuery::EntriesWithTags(EntriesWithTags(self.boxed()))
    }

    pub fn or(self, other: Self) -> Self {
        Self::Or(QueryOr2(self.boxed(), other.boxed()))
    }
}
