use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    id: i32,
    name: String,
    created_date: String,
    active: bool,
}

// create the getters for the Event struct
impl Event {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_date(&self) -> &str {
        &self.created_date
    }

    pub fn active(&self) -> bool {
        self.active
    }
}
