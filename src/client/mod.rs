use core::str::FromStr as _;
use std::path::Path;

use snafu::ResultExt as _;
use sqlx::sqlite::SqliteConnectOptions;

use crate::client::conn_pool::PoolManager;
use crate::client::conn_pool::TSConnectionPool;
use crate::client::conn_pool::TSPoolResult;
use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

pub mod conn_pool;

pub struct TagStudioClient {
    pub pool: TSConnectionPool,
}

impl TagStudioClient {
    pub async fn open_library<T>(library_path: T) -> Result<Self, SqlxError>
    where
        T: AsRef<Path>,
    {
        let mut path = library_path.as_ref().to_path_buf();
        path.push(".TagStudio");
        path.push("ts_library.sqlite");

        Self::from_connection_string(path.to_str().expect("the path is not unicode!")).await
    }

    pub async fn from_connection_string(db: &str) -> Result<Self, SqlxError> {
        let optconn = SqliteConnectOptions::from_str(db).context(SqlxSnafu)?;
        let pool = PoolManager::create_pool(optconn);
        Ok(Self { pool })
    }

    pub async fn get_connection(&self) -> TSPoolResult {
        self.pool.get().await
    }
}
