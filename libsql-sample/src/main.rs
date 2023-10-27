use anyhow::Result;
use libsql_client::{Client, Statement};
use random_string;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the database
    let db = Client::from_config(libsql_client::Config {
        url: url::Url::parse("http://127.0.0.1:8080").unwrap(),
        auth_token: None,
    })
    .await?;

    println!("Migration start");
    // Create a table
    let res = db
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )",
        )
        .await?;

    let res = db
        .execute(
            "CREATE TABLE IF NOT EXISTS address (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id),
            name TEXT NOT NULL,
            Constraint address_uc UNIQUE(user_id, name)
        )",
        )
        .await?;
    println!("Migration start {:?}", res.rows_affected);
    println!("Migration complete");
    // Insert some data

    let res = db
        .execute(Statement::with_args(
            "INSERT INTO users (name, email) VALUES (?, ?) ON CONFLICT do nothing",
            &["bob", "angels@bob.com"],
        ))
        .await?;

    println!("how many rows inserted {:?}", res.rows_affected);

    let charset = "abcdefghijklmnopqrstuvwxyz1234567890@._";
    //add a lifetime in statement

    let mut statements = Vec::with_capacity(100);
    let start = Instant::now();
    for i in 1..=5000000 {
        let stmt = Statement::with_args(
            "INSERT INTO address (user_id, name) VALUES (?, ?) ON CONFLICT do nothing",
            &["1", random_string::generate(10, charset).as_str()],
        );
        statements.push(stmt);
        if i % 100 == 0 {
            println!("batch inserting");
            let r = db.batch(statements).await?;
            println!("Rows batch inserted: {:?}", r.len());

            // reinitialize the statements
            statements = Vec::with_capacity(100);
            sleep(Duration::from_millis(100)).await;
        }
    }

    let duration = start.elapsed();

    println!(
        "Time elapsed in expensive_function() is: {:?}",
        duration.as_millis()
    );

    // Query the by joining users and address table
    let rows = db
        .execute(
            "SELECT users.name, users.email, address.name FROM users INNER JOIN address ON users.id = address.user_id limit 100"
        )
        .await?;

    // let rows = db
    //     .execute(Statement::with_args(
    //         "SELECT name, email FROM users where name = ?",
    //         &["bob"],
    //     ))
    //     .await?;

    // for row in rows.rows {
    //     let name: &str = row.try_get(0).unwrap();
    //     let email: &str = row.try_get(1).unwrap();
    //     let address: &str = row.try_get(2).unwrap();
    //     println!("{} ({}) - ({})", name, email, address);
    // }

    Ok(())
}
