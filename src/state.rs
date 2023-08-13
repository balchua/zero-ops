pub type SqlPool = sqlx::SqlitePool;

#[derive(Clone)]
pub struct State {
    pub sql: SqlPool,
}

pub type AppStateRaw = std::sync::Arc<State>;
pub type AppState = actix_web::web::Data<AppStateRaw>;
