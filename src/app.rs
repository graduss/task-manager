use axum::{Router};
use crate::auth;

#[derive(Clone)]
pub struct AppState {
  pub db_pool: crate::db::DbPool,
}

pub fn create_app(app_state: AppState) -> Router {
    Router::new()
    .nest("/api", auth::router::create_router())
    .with_state(app_state)
}