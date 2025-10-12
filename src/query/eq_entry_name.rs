use core::ops::AddAssign as _;

use crate::query::SQLQuery;
use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::trait_entry_filter::EntryFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqEntryName(pub String);

impl EntryFilter for EqEntryName {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);

        Some(format!("`entries`.`filename` = ${id}"))
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(&self.0)
    }
}

impl From<EqEntryName> for EntrySearchQuery {
    fn from(value: EqEntryName) -> Self {
        EntrySearchQuery::EqEntryName(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::eq_entry_name::EqEntryName;
    use crate::tests::fixtures::assertions::assert_eq_entries;

    #[tokio::test]
    pub async fn eq_entry_name_test() {
        assert_eq_entries(EqEntryName("maxwell.png".to_string()), vec![1]).await;
    }
}
