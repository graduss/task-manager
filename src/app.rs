//! Assembles the Axum [`Router`] and defines [`AppState`] (shared database pool).

use axum::{Router};
use crate::{
  auth,
  user,
};

#[derive(Clone)]
pub struct AppState {
  pub db_pool: crate::db::DbPool,
}

/// Builds the root [`Router`], nesting all feature sub-routers under `/api`.
pub fn create_app(app_state: AppState) -> Router {
    Router::new()
    .nest("/api", auth::router::create_router())
    .nest("/api", user::router::create_router(app_state.clone()))
    .with_state(app_state)
}