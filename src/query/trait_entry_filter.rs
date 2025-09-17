use core::ops::Deref as _;

use snafu::ResultExt;

use crate::Entry;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;
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

    fn fetch_all(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Vec<Entry>, SqlxError>> + Send
    where
        Self: Sync,
    {
        async {
            let sql = self
                .as_entry_select(&mut 1)
                .unwrap_or_else(|| "SELECT * FROM `entries`".to_string());
            let query = sqlx::query_as(&sql);
            self.bind(query).fetch_all(conn).await.context(SqlxSnafu)
        }
    }

    fn fetch_optional(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Option<Entry>, SqlxError>> + Send
    where
        Self: Sync,
    {
        async {
            let sql = self
                .as_entry_select(&mut 1)
                .unwrap_or_else(|| "SELECT * FROM `entries`".to_string());
            let query = sqlx::query_as(&sql);
            self.bind(query)
                .fetch_optional(conn)
                .await
                .context(SqlxSnafu)
        }
    }
}

impl<T> EntryFilter for Box<T>
where
    T: EntryFilter,
{
    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        self.deref().bind(query)
    }

    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        self.deref().get_where_condition(bind_id)
    }
}
