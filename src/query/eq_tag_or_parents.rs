use crate::query::SQLQuery;
use crate::query::trait_tag_filter::TagFilter;

pub struct EqTagOrParents<T>(pub T)
where
    T: TagFilter;

impl<T> TagFilter for EqTagOrParents<T>
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
