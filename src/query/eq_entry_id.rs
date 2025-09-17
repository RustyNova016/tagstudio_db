use core::ops::AddAssign as _;

use crate::query::SQLQuery;
use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::trait_entry_filter::EntryFilter;

/// Search parameter that filter on a specific entry id
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqEntryId(pub i64);

impl EntryFilter for EqEntryId {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);

        Some(format!("`entries`.`id` = ${id}"))
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(self.0)
    }
}

impl From<EqEntryId> for EntrySearchQuery {
    fn from(value: EqEntryId) -> Self {
        EntrySearchQuery::EqEntryId(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::eq_entry_id::EqEntryId;
    use crate::tests::fixtures::assertions::assert_eq_entries;

    #[tokio::test]
    pub async fn eq_entry_id_test() {
        assert_eq_entries(EqEntryId(2), vec![2]).await;
    }
}
