use crate::Entry;
use crate::query::SQLQuery;
use crate::query::trait_entry_filter::EntryFilter;
use crate::query::trait_tag_filter::TagFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntriesWithTags<T>(pub T)
where
    T: TagFilter;

impl<T> EntryFilter for EntriesWithTags<T>
where
    T: TagFilter,
{
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        self.0.get_where_condition(bind_id).map(|tags_select| {
            format!(
                "`entries`.`id` IN (
                    SELECT `tag_entries`.`entry_id` 
                    FROM `tags`
                        INNER JOIN `tag_entries` ON `tag_entries`.`tag_id` = `tags`.`id`
                    WHERE ({tags_select})
                )"
            )
        })
    }

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        self.0.bind(query)
    }
}
