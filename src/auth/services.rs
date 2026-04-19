use bcrypt::{hash, DEFAULT_COST, verify};
use crate::{
  app::AppState,
  errors::AppError,
  user::{create_user, find_user_by_email, NewUser, UserResponse},
};

use super::{
  models::{ RegisterUserRequest, LoginUserRequest },
  jwt::create_jwt,
};


pub async fn register_user(
  app_state: &AppState,
  payload: &RegisterUserRequest
) -> Result<(String, UserResponse), AppError> {
  let password_hash = hash(&payload.password, DEFAULT_COST)
    .map_err(|e| AppError::InternalServerError(e.into()))?;

  let new_user = NewUser {
    username: payload.username.clone(),
    email: payload.email.clone(),
    password_hash,
  };
  let user_response = create_user(&app_state.db_pool, new_user).await?;
  let token = create_jwt(user_response.id)?;

  Ok((token, user_response))
}

pub async fn login_user(
  app_state: &AppState,
  payload: &LoginUserRequest
) -> Result<(String, UserResponse), AppError> {
  let user = find_user_by_email(&app_state.db_pool, &payload.email).await?
  .ok_or(AppError::Unauthorized)?;

  if !verify(&payload.password, &user.password_hash)
    .map_err(|e| AppError::InternalServerError(e.into()))? {
    return Err(AppError::Unauthorized);
  }

  let token = create_jwt(user.id)?;

  Ok((token, user.into()))
}