use actix::Addr;
use infrastructure::mysql::MySqlCli;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Addr<MySqlCli>>,
}

impl AppState {
    pub fn new(db: Addr<MySqlCli>) -> Self {
        AppState { db: Arc::new(db) }
    }
}
