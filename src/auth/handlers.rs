use axum::{extract::State, Json};
use validator::Validate;

use crate::{
  app::AppState,
  errors::AppResult
};

use super::models::{
  RegisterUserRequest, AuthResponse
};

pub async fn register_user(
  State(app_state): State<AppState>,
  Json(payload): Json<RegisterUserRequest>
) -> AppResult<Json<AuthResponse>> {
  payload.validate()?;

  let (token, user) = super::services::register_user(&app_state, &payload).await?;

  Ok(Json(AuthResponse { token, user }))
}