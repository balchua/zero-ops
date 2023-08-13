use std::{env, path::PathBuf};

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://data/events.db";

mod events;

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let working_dir = get_current_working_dir();

    match working_dir {
        Err(e) => panic!("cannot find current working directory {}", e),
        Ok(p) => {
            let migrations = p.join("./migrations");
            let migration_results = sqlx::migrate::Migrator::new(migrations)
                .await
                .unwrap()
                .run(&db)
                .await;
            match migration_results {
                Ok(_) => println!("Migration success"),
                Err(error) => {
                    panic!("error: {}", error);
                }
            }
            println!("migration: {:?}", migration_results);
        }
    }
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
