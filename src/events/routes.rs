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
) -> (StatusCode, String) {
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
        Ok(r) => {
            info!("event added, rowid: {}", r);
            (StatusCode::OK, r)
        }
        Err(err) => {
            error!("event not added: {}", err);
            (StatusCode::BAD_REQUEST, "event not added".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        events::repository::EventRepositoryTrait,
        platform::{
            domain::Platform, repository::PlatformRepositoryTrait, service::PlatformService,
        },
        state::SqlPool,
    };

    use super::*;
    use async_trait::async_trait;

    use std::{collections::HashMap, sync::Arc};

    pub struct MockEventRepository {
        events: Vec<Event>,
    }

    impl MockEventRepository {
        pub fn new(events: Vec<Event>) -> Self {
            Self { events }
        }
    }

    #[async_trait]
    impl EventRepositoryTrait for MockEventRepository {
        async fn find_by_id(&self, id: i32) -> anyhow::Result<Event> {
            for event in &self.events {
                if event.id == id as i64 {
                    return Ok(event.clone());
                }
            }
            Err(anyhow::anyhow!("event not found"))
        }

        async fn find_by_platform_id(&self, id: i32) -> anyhow::Result<Vec<Event>> {
            let matching_events = self
                .events
                .iter()
                .filter(|event| event.platform_id == id as i64)
                .cloned()
                .collect();
            Ok(matching_events)
        }

        async fn insert_event(&self, _event: Event) -> anyhow::Result<String> {
            Ok("1".to_string())
        }
    }

    pub struct MockPlatformRepository {
        platforms: HashMap<i64, Platform>,
    }

    impl MockPlatformRepository {
        pub fn new(platforms: Vec<Platform>) -> Self {
            let mut map = HashMap::new();
            for platform in platforms {
                map.insert(platform.id, platform);
            }
            Self { platforms: map }
        }
    }

    #[async_trait]
    impl PlatformRepositoryTrait for MockPlatformRepository {
        async fn find_by_id(&self, id: i32) -> anyhow::Result<Platform> {
            let i64_value: i64 = id.into();
            match self.platforms.get(&i64_value) {
                Some(platform) => Ok(platform.clone()),
                None => Err(anyhow::anyhow!("Platform not found")),
            }
        }
    }
    #[tokio::test]
    async fn test_show_event_success() {
        // create a mock event
        let event = Event {
            id: 1,
            name: "Test Event".to_string(),
            active: true,
            platform_id: 1,
            created_date: Utc::now().naive_utc(),
        };

        let platform = Platform {
            id: 1,
            name: "My Platform".to_string(),
            events: vec![event.clone()],
        };

        let pool = SqlPool::connect("sqlite::memory:").await.unwrap();
        let event_store = Arc::new(MockEventRepository::new(vec![event.clone()]));
        let platform_store = Arc::new(MockPlatformRepository::new(vec![platform.clone()]));
        let platform_service = PlatformService::new(platform_store.clone(), event_store.clone());
        // create a mock app state with a mock event store
        let app_state = AppState {
            sql: pool.clone(),
            event_store: event_store.clone(),
            platform_store: platform_store.clone(),
            platform_service: platform_service.clone(),
        };

        // call the show_event function with the mock event ID
        let (status, body) = show_event(Path(event.id as i32), State(app_state)).await;

        // check that the response is successful
        assert_eq!(status, StatusCode::OK);

        // check that the response body contains the event name
        assert!(body.0.contains(&event.name));
    }
}
