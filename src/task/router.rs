use axum::{
  Router, middleware, routing::post
};

use crate::{
  app::AppState,
  middlewares,
};

pub fn create_router(app_state: &AppState) -> Router<AppState> {
  Router::new()
    .route(
      "/tasks",
      post(super::handlers::create_task)
    )
    .layer(middleware::from_fn_with_state(app_state.clone(), middlewares::get_current_user))
}