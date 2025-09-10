use crate::query::SQLQuery;
use crate::query::trait_entry_filter::EntryFilter;
use crate::query::trait_tag_filter::TagFilter;

/// Negate the condition
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QueryNot2<T>(pub T);

impl<T> TagFilter for QueryNot2<T>
where
    T: TagFilter,
{
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        self.0
            .get_where_condition(bind_id)
            .map(|cond| format!("(NOT ({cond}))"))
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        self.0.bind(query)
    }
}

impl<T> EntryFilter for QueryNot2<T>
where
    T: TagFilter,
{
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        self.0
            .get_where_condition(bind_id)
            .map(|cond| format!("(NOT ({cond}))"))
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        self.0.bind(query)
    }
}
