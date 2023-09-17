use crate::state::SqlPool;

use super::domain::Event;

#[derive(Clone)]
pub struct EventRepository {
    connection: SqlPool,
}

impl EventRepository {
    pub fn new(conn: SqlPool) -> Self {
        EventRepository { connection: conn }
    }

    // Find a event by its id
    pub async fn find_by_id(&self, id: i32) -> anyhow::Result<Event> {
        // Prepare a SQL statement to find the event by its id
        let event: Event = sqlx::query_as!(
            Event,
            "select id, active, name, created_date from events where id = ?",
            id
        )
        .fetch_one(&self.connection)
        .await?;

        // Return the event if found
        Ok(event)
    }

    // Find a event by its id
    pub async fn find_by_platform_id(&self, id: i32) -> anyhow::Result<Vec<Event>> {
        // Prepare a SQL statement to find the event by its id
        let events: Vec<Event> = sqlx::query_as!(
            Event,
            "select id, active, name, created_date from events where platform_id = ?",
            id
        )
        .fetch_all(&self.connection)
        .await?;

        // Return the event if found
        Ok(events)
    }
}
