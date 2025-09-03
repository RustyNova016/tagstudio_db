use core::fmt::Display;
use core::ops::AddAssign as _;

use crate::query::Queryfragments;
use crate::query::SQLQuery;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AnyTagString {
    tag_name: Vec<String>,
}

impl AnyTagString {
    pub fn new1<T: Display>(value: T) -> Self {
        Self {
            tag_name: vec![value.to_string()],
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
                SELECT
                    `tags`.`id` AS tag_id
                FROM
                    `tags`
                    LEFT JOIN `tag_aliases` ON `tags`.`id` = `tag_aliases`.`tag_id`
                WHERE
                    LOWER(`tags`.`name`) IN AnyTags_{id} OR -- Try finding by name
                    LOWER(`tags`.`name`) IN AnyTags_{id}_replaced OR -- Try finding by name escaped
                    LOWER(`tags`.`shorthand`) IN AnyTags_{id} OR -- Try finding by shorthand
                    LOWER(`tags`.`shorthand`) IN AnyTags_{id}_replaced OR -- Try finding by shorthand escaped
                    LOWER(`tag_aliases`.`name`) IN AnyTags_{id} OR -- Try finding by aliased name
                    LOWER(`tag_aliases`.`name`) IN AnyTags_{id}_replaced -- Try finding by aliased name escaped

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

    pub fn bind<'q>(&'q self, query: SQLQuery<'q>) -> SQLQuery<'q> {
        query.bind(serde_json::to_string(&self.tag_name).unwrap())
    }
}

impl From<Vec<String>> for AnyTagString {
    fn from(value: Vec<String>) -> Self {
        Self { tag_name: value }
    }
}

impl From<AnyTagString> for Queryfragments {
    fn from(value: AnyTagString) -> Self {
        Queryfragments::AnyTag(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::any_tag_string::AnyTagString;
    use crate::tests::fixtures::test_data::get_test_library;

    #[tokio::test]
    pub async fn tag_eq_test() {
        let lib = get_test_library().await;

        let result = Queryfragments::from(AnyTagString::from(vec![
            "cat".to_string(),
            "dog".to_string(),
        ]))
        .fetch_all(&mut lib.db.get().await.unwrap())
        .await
        .unwrap();
        assert_eq!(result.len(), 3);
    }
}
