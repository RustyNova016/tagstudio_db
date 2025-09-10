use core::ops::AddAssign as _;

use crate::query::Queryfragments;
use crate::query::SQLQuery;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AnyTagId {
    tag_name: Vec<i64>,
}

impl AnyTagId {
    pub fn new1(value: i64) -> Self {
        Self {
            tag_name: vec![value],
        }
    }

    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);
        Some(format!(
            "
            AnyTags_{id} AS (
                SELECT LOWER(value) AS tag
                FROM JSON_EACH(${id})
            ),
            AnyTags_{id}_replaced AS (
                SELECT replace(LOWER(value), '_', ' ') AS tag
                FROM JSON_EACH(${id})
            ),
            ChildTags_{id} AS (
            -- Select the actual tag we have asked for
                SELECT value AS tag_id
                FROM JSON_EACH(${id})

                UNION

            -- Recursive Select the parents
            SELECT tp.child_id AS tag_id
            FROM tag_parents tp
                INNER JOIN ChildTags_{id} c ON tp.parent_id = c.tag_id
        )"
        ))
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);
        Some(format!(
                    "`entries`.`id` IN (
                    SELECT `tag_entries`.`entry_id` 
                    FROM `ChildTags_{id}`
                        INNER JOIN `tag_entries` ON `tag_entries`.`tag_id` = `ChildTags_{id}`.`tag_id`
                )"
                ))
    }

    pub fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(serde_json::to_string(&self.tag_name).unwrap())
    }
}

impl From<Vec<i64>> for AnyTagId {
    fn from(value: Vec<i64>) -> Self {
        Self { tag_name: value }
    }
}

impl From<AnyTagId> for Queryfragments {
    fn from(value: AnyTagId) -> Self {
        Queryfragments::AnyTagId(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::any_tag_id::AnyTagId;
    use crate::tests::fixtures::test_data::get_test_library;

    #[tokio::test]
    pub async fn tag_eq_test() {
        let lib = get_test_library().await;

        let result = Queryfragments::from(AnyTagId::from(vec![1000, 1004]))
            .fetch_all(&mut lib.db.get().await.unwrap())
            .await
            .unwrap();
        assert_eq!(result.len(), 3);
    }
}
