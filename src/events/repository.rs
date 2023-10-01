use async_trait::async_trait;
use tracing::debug;

use crate::state::SqlPool;

use super::domain::Event;

#[derive(Clone)]
pub struct EventRepository {
    connection: SqlPool,
}

#[async_trait]
pub trait EventRepositoryTrait {
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Event>;
    async fn find_by_platform_id(&self, id: i32) -> anyhow::Result<Vec<Event>>;
    async fn insert_event(&self, event: Event) -> anyhow::Result<String>;
}

impl EventRepository {
    pub fn new(conn: SqlPool) -> Self {
        EventRepository { connection: conn }
    }
}

#[async_trait]
impl EventRepositoryTrait for EventRepository {
    // Find a event by its id
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Event> {
        // Prepare a SQL statement to find the event by its id
        let event: Event = sqlx::query_as!(
            Event,
            "select id, active, name, created_date, platform_id from events where id = ?",
            id
        )
        .fetch_one(&self.connection)
        .await?;

        // Return the event if found
        Ok(event)
    }

    // Find a event by its id
    async fn find_by_platform_id(&self, id: i32) -> anyhow::Result<Vec<Event>> {
        // Prepare a SQL statement to find the event by its id
        let events: Vec<Event> = sqlx::query_as!(
            Event,
            "select id, active, name, created_date, platform_id from events where platform_id = ?",
            id
        )
        .fetch_all(&self.connection)
        .await?;

        // Return the event if found
        Ok(events)
    }

    // Find a event by its id
    async fn insert_event(&self, event: Event) -> anyhow::Result<String> {
        // not using the macro here.
        let tx = self.connection.begin().await?;
        let row = sqlx::query("insert into events (active, name, platform_id) values (?, ?, ?)")
            .bind(event.active)
            .bind(event.name)
            .bind(event.platform_id)
            .execute(&self.connection)
            .await?;
        debug!("inserted event: {}", row.last_insert_rowid());
        tx.commit().await?;

        Ok(row.last_insert_rowid().to_string())
    }
}
