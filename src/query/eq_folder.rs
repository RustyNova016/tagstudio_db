use core::ops::AddAssign;

use crate::query::Queryfragments;
use crate::query::SQLQuery;

/// Check if the entry has the same folder as
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqEntryFolder {
    folder: String,
}

impl EqEntryFolder {
    pub fn new(folder: String) -> Self {
        Self { folder }
    }

    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        bind_id.add_assign(1);
        None
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id_bind = *bind_id;
        bind_id.add_assign(1);

        Some(format!(
            "replace(`entries`.`path`, `entries`.`filename`, '') =  ${id_bind}"
        ))
    }

    pub fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(&self.folder)
    }
}

impl From<EqEntryFolder> for Queryfragments {
    fn from(value: EqEntryFolder) -> Self {
        Queryfragments::EqEntryFolder(value)
    }
}

// #[cfg(test)]
// pub mod test {
//     use crate::query::Queryfragments;
//     use crate::query::eq_any_entry_id::EqAnyEntryId;
//     use crate::tests::fixtures::test_data::get_test_library;

//     #[tokio::test]
//     pub async fn tag_eq_test() {
//         let lib = get_test_library().await;

//         let result = Queryfragments::from(EqAnyEntryId::new1(0))
//             .fetch_all(&mut lib.db.get().await.unwrap())
//             .await
//             .unwrap();
//         assert_eq!(result.len(), 1);
//     }
// }
