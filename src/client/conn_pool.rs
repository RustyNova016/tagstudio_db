use deadpool::managed;
use deadpool::managed::Object;
use deadpool::managed::PoolError;
use snafu::ResultExt as _;
use sqlx::Connection as _;
use sqlx::SqliteConnection;
use sqlx::sqlite::SqliteConnectOptions;

use crate::models::errors::sqlx_error::SqlxError;
use crate::models::errors::sqlx_error::SqlxSnafu;

#[derive(Debug)]
pub struct PoolManager {
    config: SqliteConnectOptions,
}

impl PoolManager {
    pub fn create_pool(config: SqliteConnectOptions) -> TSConnectionPool {
        TSConnectionPool::builder(PoolManager { config })
            .build()
            .unwrap()
    }
}

impl managed::Manager for PoolManager {
    type Type = sqlx::SqliteConnection;
    type Error = SqlxError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        SqliteConnection::connect_with(&self.config)
            .await
            .context(SqlxSnafu)
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Self::Error> {
        Ok(conn.ping().await.context(SqlxSnafu)?)
    }
}

/// A connection pool of raw `SqliteConnection`.
pub type TSConnectionPool = managed::Pool<PoolManager>;

pub type TSPoolError = PoolError<SqlxError>;

pub type TSPoolResult = Result<Object<PoolManager>, TSPoolError>;
