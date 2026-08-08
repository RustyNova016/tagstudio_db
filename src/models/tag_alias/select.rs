use snafu::ResultExt;

use crate::SqlxError;
use crate::TagAlias;
use crate::models::errors::sqlx_error::SqlxSnafu;

impl TagAlias {
    /// Fetch the alias by its name and tag.
    ///
    /// ⚠️ Returns a vec as an alias's uniqueness isn't enforced
    pub async fn find_by_name(
        conn: &mut sqlx::SqliteConnection,
        name: &str,
        tag_id: i64,
    ) -> Result<Vec<Self>, SqlxError> {
        let sql;
        sea_query::sqlx::sqlite::query_as!(
            sql = "SELECT * FROM `tag_aliases` WHERE `name` = {name} AND `tag_id` = {tag_id}"
        )
        .fetch_all(conn)
        .await
        .context(SqlxSnafu)
    }
}
