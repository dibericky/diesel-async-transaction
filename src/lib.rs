use diesel::connection::{AnsiTransactionManager, TransactionManager};
use diesel::{PgConnection, result::Error, Connection, RunQueryDsl, deserialize::{FromSqlRow, QueryableByName}};
use schema::my_table;
mod schema;

// pub async fn transaction_async<'a, R, E, F, Fut>(conn: &'a mut PgConnection, callback: F) -> Result<R, TransactionError<E>>
//     where F : FnOnce(&'a mut PgConnection) -> Fut, Fut: std::future::Future<Output=Result<R, E>>, E: std::fmt::Debug {
//     AnsiTransactionManager::begin_transaction(conn)
//         .map_err(|e| {
//             TransactionError::DieselError(e)
//         })?;

//     match callback(&mut *conn).await {
//         Ok(value) => {
//             AnsiTransactionManager::commit_transaction(conn)
//                 .map_err(|e| {
//                     TransactionError::DieselError(e)
//                 })?;
//             Ok(value)
//         }
//         Err(user_error) => match AnsiTransactionManager::rollback_transaction(conn) {
//             Ok(()) | Err(Error::BrokenTransactionManager) => Err(TransactionError::CallbackError(user_error)),
//             Err(rollback_error) => Err(TransactionError::DieselError(rollback_error)),
//         },
//     }
// }

pub async fn transaction_async<F, R, E>(conn: &mut PgConnection, callback: F) -> Result<R, E>
    where
        F: FnOnce(&mut PgConnection) -> Result<R, E>,
        E: From<Error>,
    {
        AnsiTransactionManager::begin_transaction(conn)?;
        match callback(&mut *conn) {
            Ok(value) => {
                AnsiTransactionManager::commit_transaction(conn)?;
                Ok(value)
            }
            Err(user_error) => match AnsiTransactionManager::rollback_transaction(conn) {
                Ok(()) => Err(user_error),
                Err(Error::BrokenTransactionManager) => {
                    // In this case we are probably more interested by the
                    // original error, which likely caused this
                    Err(user_error)
                }
                Err(rollback_error) => Err(rollback_error.into()),
            },
        }
    }

#[derive(FromSqlRow, Debug, QueryableByName)]
#[diesel(table_name = my_table)]
pub struct MyTable {
    name: String
}

#[derive(Debug)]
enum TransactionError<E> where E: std::fmt::Debug {
    CallbackError(E),
    DieselError(diesel::result::Error)
}
