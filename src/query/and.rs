use crate::query::Queryfragments;
use crate::query::SQLQuery;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QueryAnd(pub Queryfragments, pub Queryfragments);

impl QueryAnd {
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
            (Some(a), Some(b)) => Some(format!("({a} AND {b})")),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    pub fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        let query = self.0.bind(query);
        self.1.bind(query)
    }
}

impl From<QueryAnd> for Queryfragments {
    fn from(value: QueryAnd) -> Self {
        Queryfragments::And(Box::new(value))
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::and::QueryAnd;
    use crate::query::eq_tag_string::EqTagString;
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
