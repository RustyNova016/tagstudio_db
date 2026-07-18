use snafu::Location;

use crate::SqlxError;
use crate::models::tag_parent::TagParentSqlError;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum TagError {
    TransactionError {
        source: sqlx::Error,
        #[snafu(implicit)]
        location: Location,
    },

    TagParentError {
        source: TagParentSqlError,
        #[snafu(implicit)]
        location: Location,
    },

    TagSQLxError {
        source: SqlxError,
        #[snafu(implicit)]
        location: Location,
    },
}
