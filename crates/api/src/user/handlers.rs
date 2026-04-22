//! HTTP handler for `GET /users/me`. Returns the authenticated user injected by middleware.

use axum::{
  extract::{ Extension, State },
  Json
};

use crate::{
  app::AppState,
  errors::{ AppResult, AppError },
  user::models::UserResponse,
  auth::Claims,
};

use super::{
  services::find_user_by_id,
};

/// `GET /api/users/me` — returns the authenticated user injected by the auth middleware.
pub async fn get_current_user(
  State(app_state): State<AppState>,
  Extension(claims): Extension<Claims>
) -> AppResult<Json<UserResponse>> {
  let user = find_user_by_id(&app_state.db_pool, claims.sub).await?
  .ok_or(AppError::Unauthorized)?;

  Ok(Json(user.into()))
}