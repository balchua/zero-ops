use std::sync::Arc;

use tracing::info;

use crate::events::repository::EventRepositoryTrait;

use super::{domain::Platform, repository::PlatformRepositoryTrait};

#[derive(Clone)]
pub struct PlatformService {
    pub platform_repo: Arc<dyn PlatformRepositoryTrait + Send + Sync>,
    pub event_repo: Arc<dyn EventRepositoryTrait + Send + Sync>,
}

impl PlatformService {
    pub fn new(
        platform_repo: Arc<dyn PlatformRepositoryTrait + Send + Sync>,
        event_repo: Arc<dyn EventRepositoryTrait + Send + Sync>,
    ) -> Self {
        PlatformService {
            platform_repo,
            event_repo,
        }
    }

    pub async fn get_platform(&self, id: i32) -> anyhow::Result<Platform> {
        let mut platform = self.platform_repo.find_by_id(id).await?;

        info!("id: {}, name: {}", platform.id, platform.name);

        let events = self.event_repo.find_by_platform_id(id).await?;
        info!(
            "total events attached to the platform {}, {}",
            platform.name,
            events.len()
        );
        platform.add_events(events);
        Ok(platform)
    }
}
