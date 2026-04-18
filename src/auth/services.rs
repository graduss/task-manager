use bcrypt::{hash, DEFAULT_COST};
use crate::{
  app::AppState,
  errors::AppError,
  user::{create_user, NewUser, UserResponse},
};


pub async fn register_user(
  app_state: &AppState,
  payload: &super::models::RegisterUserRequest
) -> Result<(String, UserResponse), AppError> {
  let password_hash = hash(&payload.password, DEFAULT_COST)
    .map_err(|e| AppError::InternalServerError(e.into()))?;

  let new_user = NewUser {
    username: payload.username.clone(),
    email: payload.email.clone(),
    password_hash,
  };
  let user_response = create_user(&app_state.db_pool, new_user).await?;
  let token = super::jwt::create_jwt(user_response.id)?;

  Ok((token, user_response))
}