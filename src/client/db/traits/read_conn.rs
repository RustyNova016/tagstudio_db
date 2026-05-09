use sqlx::SqliteConnection;

use crate::client::db::traits::write_conn::WriteConnection;

pub trait ReadConnection {
    fn conn(&mut self) -> &mut SqliteConnection;
}

impl<T> ReadConnection for T
where
    T: WriteConnection,
{
    fn conn(&mut self) -> &mut SqliteConnection {
        self.conn()
    }
}
