use core::ops::Deref;
use core::ops::DerefMut;
use std::sync::Arc;

use async_lock::RwLock;
use async_lock::RwLockWriteGuardArc;
use deadpool::managed::Object;
use sqlx::Acquire;
use sqlx::SqliteConnection;

use crate::client::conn_pool::PoolManager;
use crate::client::db::read_conn::ReadDBConn;
use crate::client::db::traits::write_conn::WriteConnection;
use crate::client::db::write_trans::WriteDBTrans;

#[derive(Debug)]
pub struct WriteDBConn {
    pub conn: Object<PoolManager>,
    pub write_guard: RwLockWriteGuardArc<()>,
    pub write_lock: Arc<RwLock<()>>,
}

impl WriteDBConn {
    #[cfg_attr(feature = "hotpath", hotpath::future_fn(log = true))]
    pub async fn begin(&mut self) -> WriteDBTrans<'_> {
        let trans = self.conn.begin().await.unwrap();

        WriteDBTrans {
            conn: trans,
            write_lock: Arc::clone(&self.write_lock),
        }
    }

    #[cfg_attr(feature = "hotpath", hotpath::future_fn(log = true))]
    pub async fn demote_read(self) -> ReadDBConn {
        drop(self.write_guard);

        let read_guard = self.write_lock.read_arc().await;

        ReadDBConn {
            conn: self.conn,
            read_guard: Arc::new(read_guard),
            write_lock: self.write_lock,
        }
    }
}

impl Deref for WriteDBConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        self.conn.deref()
    }
}

impl DerefMut for WriteDBConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.deref_mut()
    }
}

impl WriteConnection for WriteDBConn {
    fn conn(&mut self) -> &mut SqliteConnection {
        &mut self.conn
    }
}
