//! HTTP handlers for auth routes: deserialise, validate, delegate to services.

use axum::{extract::State, Json};
use validator::Validate;

use crate::{
  app::AppState,
  errors::AppResult,
  common::SafeJson,
};

use super::models::{
  RegisterUserRequest,
  LoginUserRequest,
  AuthResponse
};

/// `POST /api/auth/register` — validates the request body and registers a new user.
pub async fn register_user(
  State(app_state): State<AppState>,
  SafeJson(payload): SafeJson<RegisterUserRequest>
) -> AppResult<Json<AuthResponse>> {
  payload.validate()?;

  let (token, user) = super::services::register_user(&app_state, &payload).await?;

  Ok(Json(AuthResponse { token, user }))
}

/// `POST /api/auth/login` — validates the request body and authenticates an existing user.
pub async fn login_user(
  State(app_state): State<AppState>,
  SafeJson(payload): SafeJson<LoginUserRequest>
) -> AppResult<Json<AuthResponse>> {
  payload.validate()?;

  let (token, user) = super::services::login_user(&app_state, &payload).await?;

  Ok(Json(AuthResponse { token, user }))
}