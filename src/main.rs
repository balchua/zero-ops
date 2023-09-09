use std::net::SocketAddr;

use anyhow::Result;
use axum::{response::Html, routing::get, Router};
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

    // build our application with a route
    let app = Router::new().route("/", get(hello_handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_handler() -> Html<&'static str> {
    info!("calling the hello world handler");
    Html("<h1>Hello, World!</h1>")
}
