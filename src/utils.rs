// use sqlx::Connection;
// use sqlx::Sqlite;
// use sqlx::SqliteConnection;
// use sqlx::SqliteExecutor;
// use sqlx::Transaction;

// use crate::Library;

// /// A convenience trait to allow passing [`Library`], [`Transaction`] or even [`&mut SqliteConnection`](SqliteConnection)
// pub trait SqlConnection {
//     fn executor(&'_ mut self) -> impl SqliteExecutor<'_>;
//     fn begin(
//         &mut self,
//     ) -> impl Future<Output = Result<Transaction<'_, Sqlite>, sqlx::Error>> + Send;
// }

// impl<'c> SqlConnection for Transaction<'c, Sqlite> {
//     fn executor(&'_ mut self) -> impl SqliteExecutor<'_> {
//         &mut **self
//     }

//     async fn begin(&mut self) -> Result<Transaction<'_, Sqlite>, sqlx::Error> {
//         Connection::begin(&mut **self).await
//     }
// }

// impl SqlConnection for &mut SqliteConnection {
//     fn executor(&'_ mut self) -> impl SqliteExecutor<'_> {
//         &mut **self
//     }

//     async fn begin(&mut self) -> Result<Transaction<'_, Sqlite>, sqlx::Error> {
//         Connection::begin(&mut **self).await
//     }
// }

// impl SqlConnection for &Library {
//     fn executor(&'_ mut self) -> impl SqliteExecutor<'_> {
//         self.
//     }

//     async fn begin(&mut self) -> Result<Transaction<'_, Sqlite>, sqlx::Error> {
//         Connection::begin(&mut **self).await
//     }
// }
