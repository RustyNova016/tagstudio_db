use crate::query::SQLQuery;
use crate::query::trait_entry_filter::EntryFilter;
use crate::query::trait_tag_filter::TagFilter;

/// Turn a [`TagFilter`] into a [`EntryFilter`] by filtering all the entries that have the specific tags
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

#[cfg(test)]
pub mod test {
    use crate::query::eq_tag_id::EqTagId;
    use crate::query::eq_tag_or_children::EqTagOrChildren;
    use crate::query::trait_tag_filter::TagFilter;
    use crate::tests::fixtures::assertions::assert_eq_entries;

    #[tokio::test]
    pub async fn tag_and_test() {
        assert_eq_entries(
            EqTagOrChildren(EqTagId(1001)).into_entry_filter(),
            vec![0, 2],
        )
        .await;
    }
}
