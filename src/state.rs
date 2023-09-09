pub type SqlPool = sqlx::SqlitePool;

#[derive(Clone)]
pub struct State {
    pub sql: SqlPool,
}
