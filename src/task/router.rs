use axum::{
  Router, middleware, routing::{ post, put },
};

use crate::{
  app::AppState,
  middlewares,
};

pub fn create_router() -> Router<AppState> {
  Router::new()
    .route(
      "/tasks",
      post(super::handlers::create_task)
        .get(super::handlers::list_tasks)
    )
    .route(
      "/tasks/{task_id}",
      put(super::handlers::update_task)
        .get(super::handlers::get_task)
        .delete(super::handlers::delete_task)
    )
    .layer(middleware::from_fn(middlewares::get_current_user))
}