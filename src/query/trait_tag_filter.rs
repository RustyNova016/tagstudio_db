use core::ops::Deref;

use crate::Tag;
use crate::query::SQLQuery;
use crate::query::entries_with_tags::EntriesWithTags;
use crate::query::eq_tag_or_children::EqTagOrChildren;

/// Trait for all the querry fragments that can generate `WHERE` filter for a `SELECT` on the `tags` table
pub trait TagFilter {
    /// Transform the query into a condition that can be used in a `where`, with the table `tags` declared
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String>;

    /// Bind the inner values to the SQL query
    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O>;

    /// Transform the query into a select that matches the condition
    fn as_tag_select(&self, bind_id: &mut u64) -> Option<String> {
        self.get_where_condition(bind_id)
            .map(|wher| format!("SELECT * FROM `tags` WHERE ({wher})"))
    }

    fn fetch_all(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Vec<Tag>, sqlx::Error>> + Send
    where
        Self: Sync,
    {
        async {
            let sql = self
                .as_tag_select(&mut 1)
                .unwrap_or_else(|| "SELECT * FROM `tags`".to_string());
            let query = sqlx::query_as(&sql);
            self.bind(query).fetch_all(conn).await
        }
    }

    fn into_entry_filter(self) -> EntriesWithTags<Self>
    where
        Self: Sized,
    {
        EntriesWithTags(self)
    }

    fn add_children_tags(self) -> EqTagOrChildren<Self>
    where
        Self: Sized,
    {
        EqTagOrChildren(self)
    }
}

impl<T> TagFilter for Box<T>
where
    T: TagFilter,
{
    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        self.deref().bind(query)
    }

    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        self.deref().get_where_condition(bind_id)
    }
}
