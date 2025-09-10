use crate::Entry;
use crate::Tag;
use crate::query::SQLQuery;

/// Trait for all the query fragments that can generate `WHERE` filter for a `SELECT` on the `entries` table
pub trait EntryFilter {
    /// Transform the query into a condition that can be used in a `where`, with the table `entries` declared
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String>;

    /// Bind the inner values to the SQL query
    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O>;

    /// Transform the query into a select that matches the condition
    fn as_entry_select(&self, bind_id: &mut u64) -> Option<String> {
        self.get_where_condition(bind_id)
            .map(|wher| format!("SELECT * FROM `entries` WHERE ({wher})"))
    }

    async fn fetch_all(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<Entry>, sqlx::Error> {
        let sql = self
            .as_entry_select(&mut 1)
            .unwrap_or_else(|| "SELECT * FROM `entries`".to_string());
        let query = sqlx::query_as(&sql);
        self.bind(query).fetch_all(conn).await
    }
}
