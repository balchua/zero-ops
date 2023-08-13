#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_json;

use actix_web::{web, App, HttpRequest, HttpServer};
use anyhow::Result;

mod db;
mod events;
mod initializer;
mod migration;
mod state;
const DB_URL: &str = "sqlite://data/events.db";

#[actix_web::main]
async fn main() -> Result<()> {
    let _guard = slog_envlogger::init().unwrap();
    let log = initializer::init(DB_URL).await;
    let migrator = migration::Migrator::new(DB_URL, log.clone());
    migrator.migrate().await?;
    let db_pool = db::db_connect(DB_URL).await;

    let s = state::State { sql: db_pool };
    let raw: state::AppStateRaw = state::AppStateRaw::new(s);
    let state: state::AppState = state::AppState::new(raw);
    // start HTTP server on port 8080
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/{name}", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(anyhow::Error::from)
}

async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}
