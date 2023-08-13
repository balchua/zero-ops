use std::{env, path::PathBuf};

use thiserror::Error;

use crate::state::SqlPool;

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("migration failure")]
    InvalidMigration(String),
    #[error("db path not found")]
    DbPathNotFound(String),
}

pub struct Migrator {
    db_url: String,
    logger: slog::Logger,
}

impl Migrator {
    pub fn new(db_url: &str, logger: slog::Logger) -> Self {
        Migrator {
            db_url: db_url.to_string(),
            logger: logger,
        }
    }

    pub async fn migrate(&self) -> Result<(), MigrationError> {
        info!(self.logger, "migration begin");
        let db: SqlPool = SqlPool::connect(&self.db_url).await.unwrap();
        let working_dir = crate::migration::get_current_working_dir();

        match working_dir {
            Err(e) => Err(MigrationError::DbPathNotFound(e.to_string())),
            Ok(p) => {
                let migrations = p.join("./migrations");
                let migration_results = sqlx::migrate::Migrator::new(migrations)
                    .await
                    .unwrap()
                    .run(&db)
                    .await;
                match migration_results {
                    Ok(_) => {
                        info!(self.logger, "migration complete");
                        Ok(())
                    }
                    Err(error) => Err(MigrationError::InvalidMigration(error.to_string())),
                }
            }
        }
    }
}
