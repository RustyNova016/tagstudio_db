use core::ops::AddAssign;

use crate::query::Queryfragments;
use crate::query::SQLQuery;

pub struct EqEntryId {
    id: i64,
}

impl EqEntryId {
    pub fn new(id: i64) -> Self {
        Self { id }
    }

    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        bind_id.add_assign(1);
        None
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id_bind = *bind_id;
        bind_id.add_assign(1);

        Some(format!("`entries`.`id`= ${id_bind}"))
    }

    pub fn bind<'q>(&'q self, query: SQLQuery<'q>) -> SQLQuery<'q> {
        query.bind(self.id)
    }
}

impl From<EqEntryId> for Queryfragments {
    fn from(value: EqEntryId) -> Self {
        Queryfragments::EqEntryId(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::eq_entry_id::EqEntryId;
    use crate::tests::fixtures::test_data::get_test_library;

    #[tokio::test]
    pub async fn tag_eq_test() {
        let lib = get_test_library().await;

        let result = Queryfragments::from(EqEntryId::new(0))
            .fetch_all(&mut lib.db.get().await.unwrap())
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
    }
}
