use std::sync::Arc;

use crate::{
    events::repository::{EventRepository, EventRepositoryTrait},
    platform::{
        repository::{PlatformRepository, PlatformRepositoryTrait},
        service::PlatformService,
    },
};

pub type SqlPool = sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub sql: SqlPool,
    pub event_store: Arc<dyn EventRepositoryTrait + Send + Sync>,
    pub platform_store: Arc<dyn PlatformRepositoryTrait + Send + Sync>,
    pub platform_service: PlatformService,
}

impl AppState {
    pub fn new(sql: SqlPool) -> Self {
        let event_store = Arc::new(EventRepository::new(sql.clone()))
            as Arc<dyn EventRepositoryTrait + Send + Sync>;
        let platform_store = Arc::new(PlatformRepository::new(sql.clone()))
            as Arc<dyn PlatformRepositoryTrait + Send + Sync>;
        let platform_service = PlatformService::new(platform_store.clone(), event_store.clone());
        AppState {
            sql,
            event_store,
            platform_store,
            platform_service,
        }
    }
}
