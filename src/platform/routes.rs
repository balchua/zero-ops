use axum::{
    extract::{Path, State},
    response::Html,
};
use minijinja::{context, Environment};
use tracing::{error, info};

use crate::state::AppState;

pub async fn show_platform(
    Path(platform_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Html<String> {
    let platform = app_state.platform_service.get_platform(platform_id).await;

    match platform {
        Ok(platform) => {
            info!("id: {}, name: {}", platform.id, platform.name);
            // initialize our templates
            let mut templates_env = Environment::new();
            templates_env
                .add_template(
                    "show_platform",
                    include_str!("../templates/show_platform.html"),
                )
                .unwrap();
            let template = templates_env.get_template("show_platform").unwrap();
            let context = context! {platform => platform};
            let rendered = template.render(&context).unwrap();
            Html(rendered)
        }
        Err(err) => {
            error!("platform not found: {}", err);
            Html(String::from("platform not found"))
        }
    }
}
