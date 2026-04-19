//! Mounts `POST /auth/register` and `POST /auth/login`.

use axum::{ Router, routing::post };
use crate::app::AppState;

/// Builds the auth sub-router with registration and login routes.
pub fn create_router() -> Router<AppState> {
    Router::new()
      .route(
        "/auth/register",
        post(super::handlers::register_user)
      )
      .route(
        "/auth/login",
        post(super::handlers::login_user)
      )
}