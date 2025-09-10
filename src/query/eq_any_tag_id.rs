use core::ops::AddAssign as _;

use crate::query::SQLQuery;
use crate::query::tag_search_query::TagSearchQuery;
use crate::query::trait_tag_filter::TagFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqAnyTagId(pub Vec<i64>);

impl EqAnyTagId {
    pub fn new1(id: i64) -> Self {
        Self(vec![id])
    }
}

impl TagFilter for EqAnyTagId {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);

        Some(format!(
            "`tags`.`id` IN (SELECT value FROM JSON_EACH(${id}))"
        ))
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(serde_json::to_string(&self.0).unwrap())
    }
}

impl From<EqAnyTagId> for TagSearchQuery {
    fn from(value: EqAnyTagId) -> Self {
        TagSearchQuery::EqAnyTagId(value)
    }
}
