use core::ops::AddAssign as _;

use crate::query::SQLQuery;
use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::trait_entry_filter::EntryFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqEntryFolder(pub String);

impl EntryFilter for EqEntryFolder {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);

        Some(format!(
            "replace(`entries`.`path`, `entries`.`filename`, '') =  ${id}"
        ))
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(&self.0)
    }
}

impl From<EqEntryFolder> for EntrySearchQuery {
    fn from(value: EqEntryFolder) -> Self {
        EntrySearchQuery::EqEntryFolder(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::eq_folder::EqEntryFolder;
    use crate::tests::fixtures::assertions::assert_eq_entries;

    #[tokio::test]
    pub async fn eq_entry_id_test() {
        assert_eq_entries(EqEntryFolder("somwhere/far/".to_string()), vec![4]).await;
    }
}
