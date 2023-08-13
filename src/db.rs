use crate::state::{self, SqlPool};

pub async fn db_connect(db_url: &str) -> SqlPool {
    state::SqlPool::connect(db_url).await.unwrap()
}
