use async_trait::async_trait;

use crate::state::SqlPool;

use super::domain::Platform;

#[derive(Clone)]
pub struct PlatformRepository {
    connection: SqlPool,
}

#[async_trait]
pub trait PlatformRepositoryTrait {
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Platform>;
}

#[async_trait]
impl PlatformRepositoryTrait for PlatformRepository {
    // Find a platform by its id
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Platform> {
        // Prepare a SQL statement to find the platform by its id
        // not using macro here.
        let p = sqlx::query_as(
            r#"select   id , 
                        name 
                    from platform 
                    where id = ?"#,
        )
        .bind(id)
        .fetch_one(&self.connection)
        .await?;

        // Return the platform if found
        Ok(p)
    }
}

impl PlatformRepository {
    pub fn new(conn: SqlPool) -> Self {
        PlatformRepository { connection: conn }
    }
}
