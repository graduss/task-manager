//! Request and response types for the auth module.

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::user::UserResponse;

#[derive( Deserialize, Validate)]
pub struct RegisterUserRequest {
  #[validate(length(min = 3, max = 20))]
  pub username: String,
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 6))]
  pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginUserRequest {
  #[validate(email)]
  pub email: String,
  pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
  pub token: String,
  pub user: UserResponse,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}