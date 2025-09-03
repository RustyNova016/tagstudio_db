use core::ops::AddAssign;

use crate::query::Queryfragments;
use crate::query::SQLQuery;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqAnyEntryId {
    ids: Vec<i64>,
}

impl EqAnyEntryId {
    pub fn new(ids: Vec<i64>) -> Self {
        Self { ids }
    }

    pub fn new1(id: i64) -> Self {
        Self { ids: vec![id] }
    }

    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        bind_id.add_assign(1);
        None
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id_bind = *bind_id;
        bind_id.add_assign(1);

        Some(format!(
            "`entries`.`id` IN (SELECT value FROM JSON_EACH(${id_bind}))"
        ))
    }

    pub fn bind<'q>(&'q self, query: SQLQuery<'q>) -> SQLQuery<'q> {
        query.bind(serde_json::to_string(&self.ids).unwrap())
    }
}

impl From<EqAnyEntryId> for Queryfragments {
    fn from(value: EqAnyEntryId) -> Self {
        Queryfragments::EqEntryId(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::eq_any_entry_id::EqAnyEntryId;
    use crate::tests::fixtures::test_data::get_test_library;

    #[tokio::test]
    pub async fn tag_eq_test() {
        let lib = get_test_library().await;

        let result = Queryfragments::from(EqAnyEntryId::new1(0))
            .fetch_all(&mut lib.db.get().await.unwrap())
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
    }
}
