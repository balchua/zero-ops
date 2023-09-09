use crate::state::SqlPool;

use super::domain::Event;

pub struct EventRepository {
    connection: SqlPool,
}

impl EventRepository {
    pub fn new(conn: SqlPool) -> Self {
        EventRepository { connection: conn }
    }

    // Find a event by its id
    pub async fn find_by_id(&self, id: i32) -> Option<Event> {
        // Prepare a SQL statement to find the event by its id
        let event: Event =
            sqlx::query_as("select id, active, name, created_date from events where id = ?")
                .bind(id)
                .fetch_one(&self.connection)
                .await
                .ok()?;

        // Return the event if found
        Some(event)
    }
}
