use crate::query::Queryfragments;
use crate::query::SQLQuery;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QueryOr(pub Queryfragments, pub Queryfragments);

impl QueryOr {
    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        let q_a = self.0.get_subquery(bind_id);
        let q_b = self.1.get_subquery(bind_id);

        match (q_a, q_b) {
            (Some(a), Some(b)) => Some(format!("{a}, {b}")),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let q_a = self.0.get_where_condition(bind_id);
        let q_b = self.1.get_where_condition(bind_id);

        match (q_a, q_b) {
            (Some(a), Some(b)) => Some(format!("({a} OR {b})")),
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

impl From<QueryOr> for Queryfragments {
    fn from(value: QueryOr) -> Self {
        Queryfragments::Or(Box::new(value))
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::and::QueryAnd;
    use crate::query::eq_tag::EqTagString;
    use crate::tests::fixtures::test_data::get_test_library;

    #[tokio::test]
    pub async fn tag_and_test() {
        let lib = get_test_library().await;

        let result = Queryfragments::from(QueryAnd(
            EqTagString::from("Maxwell").into(),
            EqTagString::from("Doge").into(),
        ))
        .fetch_all(&mut lib.db.get().await.unwrap())
        .await
        .unwrap();
        assert_eq!(result.len(), 1);
    }
}
