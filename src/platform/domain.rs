use crate::events::domain::Event;

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Platform {
    pub id: i64,
    pub name: String,
    #[sqlx(skip)]
    pub events: Vec<Event>,
}

impl Platform {
    pub fn add_events(&mut self, e: Vec<Event>) {
        self.events.extend(e)
    }
}
