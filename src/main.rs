use async_transaction::{transaction_async, MyTable};
use diesel::connection::{AnsiTransactionManager, TransactionManager};
use diesel::{PgConnection, result::Error, Connection, RunQueryDsl, deserialize::{FromSqlRow, QueryableByName}};

#[tokio::main]
async fn main() {
    let mut conn = PgConnection::establish("postgresql://postgres:postgres@localhost:5552/prova").unwrap();

    let result = diesel::sql_query("SELECT name from my_table ")
            .load::<MyTable>(&mut conn)
            .map_err(|_| ()).unwrap();
    // let result = transaction_async(&mut conn, |cn| async { 
    //     diesel::sql_query("SELECT name from my_table ")
    //         .execute(cn)
    //         .map_err(|_| ())
    //         // .load::<EntitiesCode>(cn)
    // }).await;
    println!("RESULT: {result:?}")
}
