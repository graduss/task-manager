use std::io::Take;

use axum::{
  extract::{ State, Extension, Path },
  Json,
  http::StatusCode,
};

use validator::Validate;

use crate::{
  app::AppState,
  errors::{ AppResult, AppError },
  auth::Claims,
  common::{ SafeJson, SafeQuery },
};

use super::{
  models,
  services,
};

pub async fn create_task(
  State(app_state): State<AppState>,
  Extension(claims): Extension<Claims>,
  SafeJson(payload): SafeJson<models::CreateTaskRequest>,
) -> AppResult<(StatusCode, Json<models::Task>)> {
  payload.validate()?;

  let task = services::create_task(app_state, claims.sub, payload).await?;

  Ok((StatusCode::CREATED, Json(task)))
}

pub async fn list_tasks(
  State(app_state): State<AppState>,
  Extension(claims): Extension<Claims>,
  SafeQuery(params): SafeQuery<models::TasksQuery>,
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

pub async fn update_task(
  State(app_state): State<AppState>,
  Extension(claims): Extension<Claims>,
  Path(task_id): Path<uuid::Uuid>,
  SafeJson(payload): SafeJson<models::TaskUpdate>,
) -> AppResult<(StatusCode, Json<models::Task>)> {
  payload.validate()?;

  let task = services::update_task(app_state, claims.sub, task_id, payload).await?;

  Ok((StatusCode::OK, Json(task)))
}

pub async fn get_task(
  State(app_state): State<AppState>,
  Extension(claims): Extension<Claims>,
  Path(task_id): Path<uuid::Uuid>,
) -> AppResult<Json<models::Task>> {
  let task = services::get_task(app_state, claims.sub, task_id).await?;

  Ok(Json(task))
}

pub async fn delete_task(
  State(app_state): State<AppState>,
  Extension(claims): Extension<Claims>,
  Path(task_id): Path<uuid::Uuid>,
) -> AppResult<StatusCode> {
  services::delete_task(app_state, claims.sub, task_id).await?;

  Ok(StatusCode::NO_CONTENT)
}