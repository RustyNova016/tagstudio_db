use core::fmt::Display;

use crate::query::Queryfragments;
use crate::query::SQLQuery;

pub struct TagEq {
    tag_name: String,
}

impl TagEq {
    pub fn get_subquery(&self, id: &str) -> String {
        format!(
            "ChildTags_{id} AS (
    -- Select the actual tag we have asked for
    SELECT
        `tags`.`id` AS child_id
    FROM
        `tags`
        LEFT JOIN `tag_aliases` ON `tags`.`id` = `tag_aliases`.`tag_id`
    WHERE
        LOWER(`tags`.`name`) = LOWER($tag_name_{id}) OR -- Try finding by name
        LOWER(`tags`.`name`) = replace(LOWER($tag_name_{id}), '_', ' ') OR -- Try finding by name escaped
        LOWER(`tags`.`shorthand`) = LOWER($tag_name_{id}) OR -- Try finding by shorthand
        LOWER(`tags`.`shorthand`) = replace(LOWER($tag_name_{id}), '_', ' ') OR -- Try finding by shorthand escaped
        LOWER(`tag_aliases`.`name`) = LOWER($tag_name_{id}) OR -- Try finding by aliased name
        LOWER(`tag_aliases`.`name`) = replace(LOWER($tag_name_{id}), '_', ' ') -- Try finding by aliased name escaped
    UNION
    -- Recursive Select the parents
    -- (child_id and parent_id are reversed)
    SELECT
        tp.parent_id AS child_id
    FROM
        tag_parents tp
        INNER JOIN ChildTags_{id} c ON tp.child_id = c.child_id
)"
        )
    }

    pub fn get_where_condition(&self, id: &str) -> String {
        format!(
            "`entries`.`id` IN (
            SELECT `entry_id` 
            FROM `ChildTags_{id}`
                INNER JOIN `tag_entries` ON `tag_entries`.`tag_id` = `ChildTags_{id}`.`child_id`
        )"
        )
    }

    pub fn bind<'q>(&'q self, query: SQLQuery<'q>) -> SQLQuery<'q> {
        query.bind(self.tag_name.to_string())
    }
}

impl<T: Display> From<T> for TagEq {
    fn from(value: T) -> Self {
        Self {
            tag_name: value.to_string(),
        }
    }
}

impl From<TagEq> for Queryfragments {
    fn from(value: TagEq) -> Self {
        Queryfragments::Eq(value)
    }
}
