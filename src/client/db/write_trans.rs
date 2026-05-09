use core::ops::Deref;
use core::ops::DerefMut;
use std::sync::Arc;

use async_lock::RwLock;
use async_lock::RwLockWriteGuardArc;
use sqlx::Sqlite;
use sqlx::SqliteConnection;
use sqlx::Transaction;

use crate::client::db::traits::write_conn::WriteConnection;

#[derive(Debug)]
pub struct WriteDBTrans<'pool> {
    pub conn: Transaction<'pool, Sqlite>,
    pub write_lock: Arc<RwLock<()>>,
}

impl<'conn> WriteDBTrans<'conn> {
    #[cfg_attr(feature = "hotpath", hotpath::future_fn(log = true))]
    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.conn.commit().await
    }
}

impl<'conn> Deref for WriteDBTrans<'conn> {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        self.conn.deref()
    }
}

impl<'conn> DerefMut for WriteDBTrans<'conn> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.deref_mut()
    }
}

impl<'pool> WriteConnection for WriteDBTrans<'pool> {
    fn conn(&mut self) -> &mut SqliteConnection {
        &mut self.conn
    }
}

