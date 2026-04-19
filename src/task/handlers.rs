use axum::{
  extract::{State, Extension},
  Json,
  http::StatusCode,
};

use validator::Validate;

use crate::{
  app::AppState,
  errors::AppResult,
  user::UserResponse,
};

use super::{
  models,
  services,
};

pub async fn create_task(
  State(app_state): State<AppState>,
  Extension(user): Extension<UserResponse>,
  Json(payload): Json<models::CreateTaskRequest>,
) -> AppResult<(StatusCode, Json<models::Task>)> {
  payload.validate()?;

  let task = services::create_task(&app_state, &user.id, &payload).await?;

  Ok((StatusCode::CREATED, Json(task)))
}