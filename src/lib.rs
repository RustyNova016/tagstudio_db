pub mod client;
pub mod datastructures;
pub mod error;
pub mod models;
pub mod query;
#[cfg(feature = "test_utils")]
pub mod tests;

pub use crate::client::conn_pool::TSPoolError;
pub use crate::error::Error;
pub use crate::models::errors::sqlx_error::SqlxError;

// === Database ===
pub use crate::models::alias::TagAlias;
pub use crate::models::entry::Entry;
pub use crate::models::folder::Folder;
pub use crate::models::library::Library;
pub use crate::models::tag::Tag;
pub use crate::models::text_field::TextField;

pub mod sqlx {
    pub use sqlx::*;
}
