use crate::{
    events::repository::EventRepository,
    platform::{repository::PlatformRepository, service::PlatformService},
};

pub type SqlPool = sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub sql: SqlPool,
    pub event_store: EventRepository,
    pub platform_store: PlatformRepository,
    pub platform_service: PlatformService,
}

impl AppState {
    pub fn new(sql: SqlPool) -> Self {
        let event_store = EventRepository::new(sql.clone());
        let platform_store = PlatformRepository::new(sql.clone());
        let platform_service = PlatformService::new(platform_store.clone(), event_store.clone());
        AppState {
            sql,
            event_store,
            platform_store,
            platform_service,
        }
    }
}
