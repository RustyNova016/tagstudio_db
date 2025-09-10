use crate::query::Queryfragments;
use crate::query::SQLQuery;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QueryNot(pub Queryfragments);

impl QueryNot {
    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        self.0.get_subquery(bind_id)
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        self.0
            .get_where_condition(bind_id)
            .map(|cond| format!("(NOT ({cond}))"))
    }

    pub fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        self.0.bind(query)
    }
}

impl From<Queryfragments> for QueryNot {
    fn from(value: Queryfragments) -> Self {
        Self(value)
    }
}

impl From<QueryNot> for Queryfragments {
    fn from(value: QueryNot) -> Self {
        Queryfragments::Not(Box::new(value))
    }
}

impl Queryfragments {
    pub fn into_not(self) -> Self {
        QueryNot::from(self).into()
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::eq_tag_string::EqTagString;
    use crate::query::not::QueryNot;
    use crate::tests::fixtures::test_data::get_test_library;

    #[tokio::test]
    pub async fn query_not_test() {
        let lib = get_test_library().await;

        let result = Queryfragments::from(QueryNot(EqTagString::from("Maxwell").into()))
            .fetch_all(&mut lib.db.get().await.unwrap())
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
    }
}
