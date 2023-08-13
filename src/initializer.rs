use slog::{Drain, Logger};
use sqlx::migrate::MigrateDatabase;

pub type Sql = sqlx::Sqlite;

pub async fn init(db_url: &str) -> Logger {
    let drain = slog_json::Json::new(std::io::stdout())
        .set_pretty(true)
        .add_default_keys()
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!("format" => "pretty"));

    if !Sql::database_exists(db_url).await.unwrap_or(false) {
        debug!(log, "Creating database {}", db_url);
        match Sql::create_database(db_url).await {
            Ok(_) => debug!(log, "Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    }
    log
}
