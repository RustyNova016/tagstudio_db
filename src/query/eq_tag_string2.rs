use core::fmt::Display;
use core::ops::AddAssign as _;

use crate::query::SQLQuery;
use crate::query::tag_search_query::TagSearchQuery;
use crate::query::trait_tag_filter::TagFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqTagString2(pub String);

impl TagFilter for EqTagString2 {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let id = *bind_id;
        bind_id.add_assign(1);

        // The condition changes whether the tag is capitalised or not.
        if self.0 == self.0.to_lowercase() {
            // Tag isn't case sensitive

            Some(format!("
                LOWER(`tags`.`name`) = LOWER(${id}) OR -- Try finding by name
                LOWER(`tags`.`name`) = replace(LOWER(${id}), '_', ' ') OR -- Try finding by name escaped
                LOWER(`tags`.`shorthand`) = LOWER(${id}) OR -- Try finding by shorthand
                LOWER(`tags`.`shorthand`) = replace(LOWER(${id}), '_', ' ') OR -- Try finding by shorthand escaped
                `tags`.`id` IN (SELECT `tag_aliases`.`tag_id` FROM `tag_aliases` WHERE LOWER(`tag_aliases`.`name`) = LOWER(${id}) OR LOWER(`tag_aliases`.`name`) = replace(LOWER(${id}), '_', ' ')) -- Try finding by aliased name
            "))
        } else {
            // Tag is case sensitive
            Some(format!("
                `tags`.`name` = ${id} OR -- Try finding by name
                `tags`.`name` = replace(${id}, '_', ' ') OR -- Try finding by name escaped
                `tags`.`shorthand` = ${id} OR -- Try finding by shorthand
                `tags`.`shorthand` = replace(${id}, '_', ' ') OR -- Try finding by shorthand escaped
                `tags`.`id` IN (SELECT `tag_aliases`.`tag_id` FROM `tag_aliases` WHERE `tag_aliases`.`name` = ${id} OR `tag_aliases`.`name` = replace(${id}, '_', ' ')) -- Try finding by aliased name
            "))
        }
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        query.bind(&self.0)
    }
}

impl<T: Display> From<T> for EqTagString2 {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

impl From<EqTagString2> for TagSearchQuery {
    fn from(value: EqTagString2) -> Self {
        TagSearchQuery::EqTagString(value)
    }
}
