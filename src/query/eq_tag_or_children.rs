use crate::query::SQLQuery;
use crate::query::tag_search_query::TagSearchQuery;
use crate::query::trait_tag_filter::TagFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqTagOrChildren<T>(pub T)
where
    T: TagFilter;

impl<T> TagFilter for EqTagOrChildren<T>
where
    T: TagFilter,
{
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        self.0.as_tag_select(bind_id).map(|tags_select| {
            format!(
                "
                `tags`.`id` IN (
                    WITH RECURSIVE ChildTags AS (
                        -- Select the actual tag we have asked for
                        SELECT id AS tag_id
                        FROM ({tags_select})

                        UNION

                        -- Recursive Select the parents
                        SELECT tp.child_id AS tag_id
                        FROM tag_parents tp
                            INNER JOIN ChildTags c ON tp.parent_id = c.tag_id
                    )
                    SELECT tag_id AS id FROM ChildTags
                )
                "
            )
        })
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        self.0.bind(query)
    }
}

impl<T> From<EqTagOrChildren<T>> for TagSearchQuery
where
    T: TagFilter,
    TagSearchQuery: From<T>,
{
    fn from(value: EqTagOrChildren<T>) -> Self {
        TagSearchQuery::EqTagOrChildren(EqTagOrChildren(TagSearchQuery::from(value.0).boxed()))
    }
}
