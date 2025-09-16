use core::ops::AddAssign as _;

use crate::query::SQLQuery;
use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::trait_entry_filter::EntryFilter;

/// Filter on the entry with its full absolute path
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqAbsolutePath(pub String);

impl EntryFilter for EqAbsolutePath {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);

        Some(format!(
            "`entries`.`id` in (SELECT
                `entries`.`id`
            FROM
                `entries`
                INNER JOIN `folders` ON `folders`.id = `entries`.`folder_id`
            WHERE
                CONCAT (`folders`.`path`, '/', `entries`.`path`) = ${id} -- UNIX
                OR CONCAT (`folders`.`path`, '\\', `entries`.`path`) = ${id} -- Windows
            )"
        ))
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(&self.0)
    }
}

impl From<EqAbsolutePath> for EntrySearchQuery {
    fn from(value: EqAbsolutePath) -> Self {
        EntrySearchQuery::EqAbsolutePath(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::eq_absolute_path::EqAbsolutePath;
    use crate::tests::fixtures::assertions::assert_eq_entries;

    #[tokio::test]
    pub async fn eq_absolute_path_test() {
        assert_eq_entries(
            EqAbsolutePath("/tmp/somwhere/far/away.png".to_string()),
            vec![4],
        )
        .await;
    }
}
