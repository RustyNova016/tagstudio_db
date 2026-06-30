use std::sync::Arc;

use async_lock::RwLock;
use async_lock::RwLockReadGuardArc;
use deadpool::managed::Object;
use sqlx::SqliteConnection;

use crate::client::conn_pool::PoolManager;
use crate::client::db::traits::read_conn::ReadConnection;
use crate::client::db::write_conn::WriteDBConn;

#[derive(Debug)]
pub struct ReadDBConn {
    pub conn: Object<PoolManager>,
    pub read_guard: Arc<RwLockReadGuardArc<()>>,
    pub write_lock: Arc<RwLock<()>>,
}

impl ReadDBConn {
    #[cfg_attr(feature = "hotpath", hotpath::future_fn(log = true))]
    pub async fn promote_write(self) -> WriteDBConn {
        drop(self.read_guard);

        #[cfg(feature = "hotpath")]
        hotpath::gauge!("waiting_write").inc(1.0);
        let write_guard = self.write_lock.write_arc().await;
        #[cfg(feature = "hotpath")]
        hotpath::gauge!("waiting_write").dec(1.0);

        WriteDBConn {
            conn: self.conn,
            write_guard: write_guard,
            write_lock: self.write_lock,
        }
    }
}

impl ReadConnection for ReadDBConn {
    fn conn(&mut self) -> &mut SqliteConnection {
        &mut self.conn
    }
}
