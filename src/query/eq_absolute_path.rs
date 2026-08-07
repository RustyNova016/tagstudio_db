use core::ops::AddAssign as _;

use crate::query::SQLQuery;
use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::trait_entry_filter::QueryEntryFilter;

/// Search parameter that filter on the entry with its full absolute path
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqAbsolutePath {
    pub path: String,
    pub library_path: String,
}

impl QueryEntryFilter for EqAbsolutePath {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let path_id = *bind_id;
        bind_id.add_assign(1);

        let lib_id = *bind_id;
        bind_id.add_assign(1);

        Some(format!(
            "`entries`.`id` in (SELECT
                `entries`.`id`
            FROM
                `entries`
            WHERE
                CONCAT (${lib_id}, '/', `entries`.`path`) = ${path_id} -- UNIX
                OR CONCAT (${lib_id}, '\\', `entries`.`path`) = ${path_id} -- Windows
            )"
        ))
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(&self.path).bind(&self.library_path)
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
            EqAbsolutePath {
                library_path: "/tmp".to_string(),
                path: "/tmp/somwhere/far/away.png".to_string(),
            },
            vec!["somwhere/far/away.png"],
        )
        .await;
    }
}
