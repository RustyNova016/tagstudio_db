use core::ops::AddAssign;

use crate::query::Queryfragments;
use crate::query::SQLQuery;

pub struct TagAnd(pub Queryfragments, pub Queryfragments);

impl TagAnd {
    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        let q_a = self.0.get_subquery(bind_id);
        bind_id.add_assign(1);
        let q_b = self.1.get_subquery(bind_id);
        bind_id.add_assign(1);

        match (q_a, q_b) {
            (Some(a), Some(b)) => Some(format!("{a}, {b}")),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let q_a = self.0.get_where_condition(bind_id);
        bind_id.add_assign(1);
        let q_b = self.1.get_where_condition(bind_id);
        bind_id.add_assign(1);

        match (q_a, q_b) {
            (Some(a), Some(b)) => Some(format!("({a} AND {b})")),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    pub fn bind<'q>(&'q self, query: SQLQuery<'q>) -> SQLQuery<'q> {
        let query = self.0.bind(query);
        self.1.bind(query)
    }
}

impl From<TagAnd> for Queryfragments {
    fn from(value: TagAnd) -> Self {
        Queryfragments::And(Box::new(value))
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::and::TagAnd;
    use crate::query::tag_eq::TagEq;
    use crate::tests::fixtures::test_data::get_test_library;

    #[tokio::test]
    pub async fn tag_and_test() {
        let lib = get_test_library().await;

        let result = Queryfragments::from(TagAnd(
            TagEq::from("Maxwell").into(),
            TagEq::from("Doge").into(),
        ))
        .fetch_all(&mut lib.db.get().await.unwrap())
        .await
        .unwrap();
        assert_eq!(result.len(), 1);
    }
}
