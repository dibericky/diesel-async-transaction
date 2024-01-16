use async_transaction::{transaction_async, MyTable};
use diesel::connection::{AnsiTransactionManager, TransactionManager};
use diesel::{PgConnection, result::Error, Connection, RunQueryDsl, deserialize::{FromSqlRow, QueryableByName}};

#[tokio::main]
async fn main() {
    let mut conn = PgConnection::establish("postgresql://postgres:postgres@localhost:5552/prova").unwrap();

    
    let result = transaction_async(&mut conn, |cn: &mut PgConnection| {
        Box::pin(async move {
            diesel::sql_query("UPDATE my_table SET name = 'ciaoooooo' where name = 'ciao2'")
            .execute(cn)
        })
    }).await;
    
    println!("RESULT: {result:?}");

    let result2 = diesel::sql_query("SELECT name from my_table ")
            .load::<MyTable>(&mut conn).unwrap();
    println!("RESULT2: {result2:?}");
}
