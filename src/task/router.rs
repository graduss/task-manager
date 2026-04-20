use axum::{
  Router, middleware, routing::{ post, get },
};

use crate::{
  app::AppState,
  middlewares,
};

pub fn create_router() -> Router<AppState> {
  Router::new()
    .route(
      "/tasks",
      post(super::handlers::create_task).get(super::handlers::list_tasks)
    )
    .layer(middleware::from_fn(middlewares::get_current_user))
}