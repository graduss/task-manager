use axum::{
  extract::{ State, Extension, Query },
  Json,
  http::StatusCode,
};

use validator::Validate;

use crate::{
  app::AppState,
  errors::{ AppResult, AppError },
  auth::Claims,
};

use super::{
  models,
  services,
};

pub async fn create_task(
  State(app_state): State<AppState>,
  Extension(claims): Extension<Claims>,
  Json(payload): Json<models::CreateTaskRequest>,
) -> AppResult<(StatusCode, Json<models::Task>)> {
  payload.validate()?;

  let task = services::create_task(app_state, claims.sub, payload).await?;

  Ok((StatusCode::CREATED, Json(task)))
}

pub async fn list_tasks(
  State(app_state): State<AppState>,
  Extension(claims): Extension<Claims>,
  Query(params): Query<models::TasksQuery>,
) -> AppResult<Json<models::TasksResponse>> {
  params.validate()
    .map_err(|e| {
      tracing::error!("Validation error: {:?}", e);
      AppError::ValidationError(e)
    })?;

  let tasks_response = services::list_tasks(app_state, claims.sub, params).await
  .map_err(|e| {
    tracing::error!("Error listing tasks: {:?}", e);
    e
  })?;

  Ok(Json(tasks_response))
}