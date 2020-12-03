use crate::state::pool;
use std::sync::Arc;

pub struct StaticData {
  pub db: pool::DbPool,
}

#[derive(Clone)]
pub struct AppState {
  pub static_data: Arc<StaticData>,
}

pub type DbConnection =
  r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::PgConnection>>;

impl AppState {
  pub fn get_connection(&self) -> DbConnection {
    self
      .static_data
      .db
      .get()
      .expect("Failed to retrieve DB connection from pool")
  }
}

pub fn initialize() -> AppState {
  let db_pool = pool::get_connection_pool();

  AppState {
    static_data: Arc::new(StaticData { db: db_pool }),
  }
}
