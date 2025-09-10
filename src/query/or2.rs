use crate::query::SQLQuery;
use crate::query::trait_entry_filter::EntryFilter;
use crate::query::trait_tag_filter::TagFilter;

/// Merge two filters with an `or` condition
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QueryOr2<T, U>(pub T, pub U);

impl<T, U> TagFilter for QueryOr2<T, U>
where
    T: TagFilter,
    U: TagFilter,
{
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let q_a = self.0.get_where_condition(bind_id);
        let q_b = self.1.get_where_condition(bind_id);

        match (q_a, q_b) {
            (Some(a), Some(b)) => Some(format!("({a} OR {b})")),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        let query = self.0.bind(query);
        self.1.bind(query)
    }
}

impl<T, U> EntryFilter for QueryOr2<T, U>
where
    T: EntryFilter,
    U: EntryFilter,
{
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let q_a = self.0.get_where_condition(bind_id);
        let q_b = self.1.get_where_condition(bind_id);

        match (q_a, q_b) {
            (Some(a), Some(b)) => Some(format!("({a} OR {b})")),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        let query = self.0.bind(query);
        self.1.bind(query)
    }
}
