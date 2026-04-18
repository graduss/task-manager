use axum::{ Router, routing::post };

use crate::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new().route(
      "/auth/register",
      post(super::handlers::register_user)
    )
}