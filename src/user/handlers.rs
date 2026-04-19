use axum::{
  extract::{ Extension },
  Json
};

use crate::{
  errors::AppResult,
  user::models::UserResponse
};

pub async fn get_current_user(
  Extension(user): Extension<UserResponse>
) -> AppResult<Json<UserResponse>> {

  Ok(Json(user))
}