use axum::{ Router, routing::post };

use crate::app::AppState;

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