use diesel::connection::{AnsiTransactionManager, TransactionManager};
use diesel::{PgConnection, result::Error, Connection, RunQueryDsl, deserialize::{FromSqlRow, QueryableByName}};
use futures::future::BoxFuture;
use schema::my_table;
mod schema;

pub async fn transaction_async<R, E, F>(
    conn: &mut PgConnection,
    callback: F,
) -> Result<R, TransactionError<E>>
where
    F: for<'a> Fn(&'a mut PgConnection) -> BoxFuture<'a, Result<R, E>>,
    E: std::fmt::Debug,
{
    AnsiTransactionManager::begin_transaction(conn)
        .map_err(|e| TransactionError::DieselError(e))?;

    match callback(conn).await {
        Ok(value) => {
            AnsiTransactionManager::commit_transaction(conn)
                .map_err(|e| TransactionError::DieselError(e))?;
            Ok(value)
        }
        Err(user_error) => match AnsiTransactionManager::rollback_transaction(conn) {
            Ok(()) | Err(diesel::result::Error::BrokenTransactionManager) => {
                Err(TransactionError::CallbackError(user_error))
            }
            Err(rollback_error) => Err(TransactionError::DieselError(rollback_error)),
        },
    }
}
#[derive(FromSqlRow, Debug, QueryableByName)]
#[diesel(table_name = my_table)]
pub struct MyTable {
    name: String
}

#[derive(Debug)]
pub enum TransactionError<E> where E: std::fmt::Debug {
    CallbackError(E),
    DieselError(diesel::result::Error)
}
