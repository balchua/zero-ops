use sqlx::migrate::MigrateDatabase;
use tracing::debug;
use tracing_subscriber::EnvFilter;

pub type Sql = sqlx::Sqlite;

pub async fn init(db_url: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();

    if !Sql::database_exists(db_url).await.unwrap_or(false) {
        debug!("Creating database {}", db_url);
        match Sql::create_database(db_url).await {
            Ok(_) => debug!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    }
}
