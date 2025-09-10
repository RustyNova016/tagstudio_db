use core::fmt::Display;
use core::ops::AddAssign as _;

use crate::query::Queryfragments;
use crate::query::SQLQuery;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqTagString {
    tag_name: String,
}

impl EqTagString {
    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);
        Some(format!(
            "ChildTags_{id} AS (
            -- Select the actual tag we have asked for
            SELECT
                `tags`.`id` AS tag_id
            FROM
                `tags`
                LEFT JOIN `tag_aliases` ON `tags`.`id` = `tag_aliases`.`tag_id`
            WHERE
                LOWER(`tags`.`name`) = LOWER(${id}) OR -- Try finding by name
                LOWER(`tags`.`name`) = replace(LOWER(${id}), '_', ' ') OR -- Try finding by name escaped
                LOWER(`tags`.`shorthand`) = LOWER(${id}) OR -- Try finding by shorthand
                LOWER(`tags`.`shorthand`) = replace(LOWER(${id}), '_', ' ') OR -- Try finding by shorthand escaped
                LOWER(`tag_aliases`.`name`) = LOWER(${id}) OR -- Try finding by aliased name
                LOWER(`tag_aliases`.`name`) = replace(LOWER(${id}), '_', ' ') -- Try finding by aliased name escaped

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
                    SELECT `entry_id` 
                    FROM `ChildTags_{id}`
                        INNER JOIN `tag_entries` ON `tag_entries`.`tag_id` = `ChildTags_{id}`.`tag_id`
                )"
                ))
    }

    pub fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(self.tag_name.to_string())
    }
}

impl<T: Display> From<T> for EqTagString {
    fn from(value: T) -> Self {
        Self {
            tag_name: value.to_string(),
        }
    }
}

impl From<EqTagString> for Queryfragments {
    fn from(value: EqTagString) -> Self {
        Queryfragments::EqTag(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::eq_tag_string::EqTagString;
    use crate::tests::fixtures::test_data::get_test_library;

    #[tokio::test]
    pub async fn tag_eq_test() {
        let lib = get_test_library().await;

        let result = Queryfragments::from(EqTagString::from("cat"))
            .fetch_all(&mut lib.db.get().await.unwrap())
            .await
            .unwrap();
        assert_eq!(result.len(), 2);

        let result = Queryfragments::from(EqTagString::from("meme"))
            .fetch_all(&mut lib.db.get().await.unwrap())
            .await
            .unwrap();
        assert_eq!(result.len(), 3);
    }
}
