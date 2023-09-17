use axum::{
    extract::{Path, State},
    response::Html,
};
use minijinja::{context, Environment};
use tracing::{error, info};

use crate::state::AppState;

pub struct EventRoutes {}

impl EventRoutes {}

pub async fn show_event(
    Path(event_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Html<String> {
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
            Html(rendered)
        }
        Err(err) => {
            error!("event not found: {}", err);
            Html(String::from("event not found"))
        }
    }
}
