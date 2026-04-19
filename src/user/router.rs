//! Mounts `GET /users/me` behind the JWT auth middleware.

use axum::{ Router, middleware, routing::get };
use crate::{
  app::AppState,
  middlewares,
};

/// Builds the user sub-router, protecting all routes with the JWT auth middleware.
pub fn create_router(app_state: &AppState) -> Router<AppState> {
    Router::new()
      .route(
        "/users/me",
        get(super::handlers::get_current_user)
      )
      .layer(middleware::from_fn_with_state(app_state.clone(), middlewares::get_current_user))
}