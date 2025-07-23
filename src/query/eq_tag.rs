use core::fmt::Display;
use core::ops::AddAssign as _;

use crate::query::Queryfragments;
use crate::query::SQLQuery;

pub struct EqTag {
    tag_name: String,
}

impl EqTag {
    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);
        Some(format!(
            "ChildTags_{id} AS (
            -- Select the actual tag we have asked for
            SELECT
                `tags`.`id` AS child_id
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
            -- (child_id and parent_id are reversed)
            SELECT
                tp.parent_id AS child_id
            FROM
                tag_parents tp
                INNER JOIN ChildTags_{id} c ON tp.child_id = c.child_id
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
                        INNER JOIN `tag_entries` ON `tag_entries`.`tag_id` = `ChildTags_{id}`.`child_id`
                )"
                ))
    }

    pub fn bind<'q>(&'q self, query: SQLQuery<'q>) -> SQLQuery<'q> {
        query.bind(self.tag_name.to_string())
    }
}

impl<T: Display> From<T> for EqTag {
    fn from(value: T) -> Self {
        Self {
            tag_name: value.to_string(),
        }
    }
}

impl From<EqTag> for Queryfragments {
    fn from(value: EqTag) -> Self {
        Queryfragments::EqTag(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::eq_tag::EqTag;
    use crate::tests::fixtures::test_data::get_test_library;

    #[tokio::test]
    pub async fn tag_eq_test() {
        let lib = get_test_library().await;

        let result = Queryfragments::from(EqTag::from("cat"))
            .fetch_all(&mut lib.db.get().await.unwrap())
            .await
            .unwrap();
        assert_eq!(result.len(), 2);

        let result = Queryfragments::from(EqTag::from("meme"))
            .fetch_all(&mut lib.db.get().await.unwrap())
            .await
            .unwrap();
        assert_eq!(result.len(), 3);
    }
}
