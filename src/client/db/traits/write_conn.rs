use core::ops::DerefMut;

use deadpool::managed::Object;
use sqlx::SqliteConnection;
use sqlx::SqliteTransaction;

use crate::client::conn_pool::PoolManager;

pub trait WriteConnection {
    fn conn(&mut self) -> &mut SqliteConnection;
}

impl WriteConnection for SqliteConnection {
    fn conn(&mut self) -> &mut SqliteConnection {
        self
    }
}

impl WriteConnection for Object<PoolManager> {
    fn conn(&mut self) -> &mut SqliteConnection {
        self.deref_mut()
    }
}

impl<'c> WriteConnection for SqliteTransaction<'c> {
    fn conn(&mut self) -> &mut SqliteConnection {
        self.deref_mut()
    }
}
