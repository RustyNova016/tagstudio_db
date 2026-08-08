use std::sync::Arc;

use async_lock::RwLock;
use futures::FutureExt;

use crate::TSPoolError;
use crate::client::conn_pool::TSConnectionPool;
use crate::client::conn_pool::TSPoolResult;
use crate::client::db::read_conn::ReadDBConn;
use crate::client::db::write_conn::WriteDBConn;

pub mod read_conn;
pub mod traits;
pub mod write_conn;
pub mod write_trans;

#[derive(Debug)]
pub struct DatabasePooler {
    write_lock: Arc<RwLock<()>>,
    pub pool: Arc<TSConnectionPool>,
}

impl DatabasePooler {
    pub fn new(pool: TSConnectionPool) -> Self {
        Self {
            pool: Arc::new(pool),
            write_lock: Arc::new(RwLock::new(())),
        }
    }

    #[cfg_attr(feature = "hotpath", hotpath::future_fn(log = true))]
    pub async fn get(&self) -> TSPoolResult {
        #[cfg(feature = "hotpath")]
        hotpath::gauge!("waiting_conn").inc(1.0);

        let conn = self.pool.get().await;

        #[cfg(feature = "hotpath")]
        hotpath::gauge!("waiting_conn").dec(1.0);
        conn
    }

    #[cfg_attr(feature = "hotpath", hotpath::future_fn(log = true))]
    pub async fn write_conn(&self) -> Result<WriteDBConn, TSPoolError> {
        #[cfg(feature = "hotpath")]
        hotpath::gauge!("waiting_write").inc(1.0);
        let write_guard = self.write_lock.write_arc().await;
        #[cfg(feature = "hotpath")]
        hotpath::gauge!("waiting_write").dec(1.0);
        let conn = self.pool.get().await?;

        Ok(WriteDBConn {
            conn,
            write_guard,
            write_lock: Arc::clone(&self.write_lock),
        })
    }

    #[cfg_attr(feature = "hotpath", hotpath::future_fn(log = true))]
    pub async fn read_conn(&self) -> Result<ReadDBConn, TSPoolError> {
        let read_guard = Arc::new(self.write_lock.read_arc().await);
        let conn = self.pool.get().await.unwrap();

        Ok(ReadDBConn {
            conn,
            read_guard,
            write_lock: Arc::clone(&self.write_lock),
        })
    }
}
