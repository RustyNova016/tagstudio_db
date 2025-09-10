use core::ops::AddAssign as _;

use crate::query::SQLQuery;
use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::trait_entry_filter::EntryFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqAnyEntryId(pub Vec<i64>);

impl EqAnyEntryId {
    pub fn new1(id: i64) -> Self {
        Self(vec![id])
    }
}

impl EntryFilter for EqAnyEntryId {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);

        Some(format!(
            "`entries`.`id` IN (SELECT value FROM JSON_EACH(${id}))"
        ))
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(serde_json::to_string(&self.0).unwrap())
    }
}

impl From<EqAnyEntryId> for EntrySearchQuery {
    fn from(value: EqAnyEntryId) -> Self {
        EntrySearchQuery::EqAnyEntryId(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::eq_any_entry_id::EqAnyEntryId;
    use crate::tests::fixtures::assertions::assert_eq_entries;

    #[tokio::test]
    pub async fn eq_entry_id_test() {
        assert_eq_entries(EqAnyEntryId(vec![2, 4]), vec![2, 4]).await;
    }
}
