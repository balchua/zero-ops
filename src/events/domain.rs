use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub struct Event {
    pub id: i64,
    pub name: String,
    pub created_date: chrono::NaiveDateTime,
    pub active: bool,
}
