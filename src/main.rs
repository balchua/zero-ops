use std::net::SocketAddr;

use axum::{routing::get, Router};

use minijinja::Environment;
use tracing::info;

mod db;
mod events;
mod initializer;
mod migration;
mod state;

const DB_URL: &str = "sqlite://data/events.db";

#[tokio::main]
async fn main() {
    initializer::init(DB_URL).await;
    let migrator = migration::Migrator::new(DB_URL);
    migrator.migrate().await.unwrap();

    let conn = db::db_connect(DB_URL).await;

    // build our application with a route
    let app = Router::new()
        .route(
            "/event/:event_id",
            get(events::routes::EventRoutes::show_event),
        )
        .with_state(conn);
    //.with_state(templates_env);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
