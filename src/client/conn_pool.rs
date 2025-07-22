use deadpool::managed;
use deadpool::managed::Object;
use deadpool::managed::PoolError;
use sqlx::Connection as _;
use sqlx::SqliteConnection;
use sqlx::sqlite::SqliteConnectOptions;

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
    type Error = sqlx::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        SqliteConnection::connect_with(&self.config).await
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Self::Error> {
        Ok(conn.ping().await?)
    }
}

/// A connection pool of raw `SqliteConnection`.
pub type TSConnectionPool = managed::Pool<PoolManager>;

pub type TSPoolError = PoolError<sqlx::Error>;

pub type TSPoolResult = Result<Object<PoolManager>, TSPoolError>;
