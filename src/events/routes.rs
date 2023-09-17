use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    Form,
};
use chrono::Utc;
use minijinja::{context, Environment};
use tracing::{error, info};

use crate::state::AppState;

use super::domain::Event;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EventInput {
    pub name: String,
    pub active: bool,
    pub platform_id: i64,
}

pub async fn show_event(
    Path(event_id): Path<i32>,
    State(app_state): State<AppState>,
) -> (StatusCode, Html<String>) {
    let result = app_state.event_store.find_by_id(event_id).await;

    match result {
        Ok(event) => {
            info!(
                "id: {}, name: {}, active: {}, date: {}",
                event.id, event.name, event.active, event.created_date
            );
            // initialize our templates
            let mut templates_env = Environment::new();
            templates_env
                .add_template("show_event", include_str!("../templates/show_event.html"))
                .unwrap();
            let template = templates_env.get_template("show_event").unwrap();
            let context = context! {event => event};
            let rendered = template.render(&context).unwrap();
            (StatusCode::OK, Html(rendered))
        }
        Err(err) => {
            error!("event not found: {}", err);
            (
                StatusCode::NOT_FOUND,
                Html(String::from("platform not found")),
            )
        }
    }
}

pub async fn add_event(
    app_state: State<AppState>,
    Form(event_input): Form<EventInput>,
) -> (StatusCode) {
    let e = event_input;
    let event = Event {
        id: 0,
        name: e.name,
        active: e.active,
        platform_id: e.platform_id,
        created_date: Utc::now().naive_utc(),
    };
    let result = app_state.event_store.insert_event(event).await;

    match result {
        Ok(_) => {
            info!("event added");
            StatusCode::OK
        }
        Err(err) => {
            error!("event not added: {}", err);
            StatusCode::BAD_REQUEST
        }
    }
}
