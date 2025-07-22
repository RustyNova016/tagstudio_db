pub mod client;
pub mod error;
pub mod models;
pub mod query;
#[cfg(feature = "test_utils")]
pub mod tests;
pub mod utils;

pub use crate::client::conn_pool::TSPoolError;
pub use crate::error::Error;

pub mod sqlx {
    pub use sqlx::*;
}
