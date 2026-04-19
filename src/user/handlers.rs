//! HTTP handler for `GET /users/me`. Returns the authenticated user injected by middleware.

use axum::{
  extract::{ Extension },
  Json
};

use crate::{
  errors::AppResult,
  user::models::UserResponse
};

/// `GET /api/users/me` — returns the authenticated user injected by the auth middleware.
pub async fn get_current_user(
  Extension(user): Extension<UserResponse>
) -> AppResult<Json<UserResponse>> {

  Ok(Json(user))
}