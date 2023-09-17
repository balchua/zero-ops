use std::net::SocketAddr;

use axum::{routing::get, Router};

use tracing::info;

use crate::state::AppState;

mod db;
mod events;
mod initializer;
mod migration;
mod platform;
mod state;

const DB_URL: &str = "sqlite://data/events.db";

#[tokio::main]
async fn main() {
    initializer::init(DB_URL).await;
    let migrator = migration::Migrator::new(DB_URL);
    migrator.migrate().await.unwrap();

    let conn = db::db_connect(DB_URL).await;
    let app_state = AppState::new(conn.clone());
    // build our application with a route
    let app = Router::new()
        .route("/event/:event_id", get(events::routes::show_event))
        .route(
            "/platform/:platform_id",
            get(platform::routes::show_platform),
        )
        .with_state(app_state);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
