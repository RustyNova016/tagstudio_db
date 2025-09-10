use core::ops::AddAssign as _;

use crate::query::SQLQuery;
use crate::query::trait_tag_filter::TagFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqTagId(pub i64);

impl TagFilter for EqTagId {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);

        Some(format!("`tags`.`id` = ${id}"))
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(&self.0)
    }
}
