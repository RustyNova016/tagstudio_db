use crate::query::SQLQuery;
use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::tag_search_query::TagSearchQuery;
use crate::query::trait_entry_filter::EntryFilter;
use crate::query::trait_tag_filter::TagFilter;

/// Merge two filters with an `and` condition
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QueryAnd<T, U>(pub T, pub U);

impl<T, U> TagFilter for QueryAnd<T, U>
where
    T: TagFilter,
    U: TagFilter,
{
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let q_a = self.0.get_where_condition(bind_id);
        let q_b = self.1.get_where_condition(bind_id);

        match (q_a, q_b) {
            (Some(a), Some(b)) => Some(format!("({a} AND {b})")),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    /// Bind the inner values to the SQL query
    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        let query = self.0.bind(query);
        self.1.bind(query)
    }
}

impl<T, U> EntryFilter for QueryAnd<T, U>
where
    T: EntryFilter,
    U: EntryFilter,
{
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let q_a = self.0.get_where_condition(bind_id);
        let q_b = self.1.get_where_condition(bind_id);

        match (q_a, q_b) {
            (Some(a), Some(b)) => Some(format!("({a} AND {b})")),
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

impl<T, U> From<QueryAnd<T, U>> for TagSearchQuery
where
    T: TagFilter,
    TagSearchQuery: From<T> + From<U>,
{
    fn from(value: QueryAnd<T, U>) -> Self {
        TagSearchQuery::And(QueryAnd(
            TagSearchQuery::from(value.0).boxed(),
            TagSearchQuery::from(value.1).boxed(),
        ))
    }
}

impl<T, U> From<QueryAnd<T, U>> for EntrySearchQuery
where
    EntrySearchQuery: From<T> + From<U>,
{
    fn from(value: QueryAnd<T, U>) -> Self {
        EntrySearchQuery::And(QueryAnd(
            EntrySearchQuery::from(value.0).boxed(),
            EntrySearchQuery::from(value.1).boxed(),
        ))
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::and::QueryAnd;
    use crate::query::eq_tag_id::EqTagId;
    use crate::query::eq_tag_or_children::EqTagOrChildren;
    use crate::query::eq_tag_string::EqTagString;
    use crate::query::trait_tag_filter::TagFilter;
    use crate::tests::fixtures::assertions::assert_eq_entries;

    #[tokio::test]
    pub async fn tag_and_test() {
        assert_eq_entries(
            QueryAnd(
                EqTagOrChildren(EqTagId(1001)).into_entry_filter(),
                EqTagOrChildren(EqTagId(1003)).into_entry_filter(),
            ),
            vec![2],
        )
        .await;

        assert_eq_entries(
            QueryAnd(
                EqTagOrChildren(EqTagString::from("maxwell")).into_entry_filter(),
                EqTagOrChildren(EqTagString::from("doge")).into_entry_filter(),
            ),
            vec![2],
        )
        .await;
    }
}
