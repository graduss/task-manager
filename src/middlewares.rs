//! Axum middleware for JWT authentication. Extracts the `Authorization: Bearer <token>`
//! header, decodes the JWT, loads the matching user from the database, and injects a
//! [`UserResponse`] extension into the request for downstream handlers.

use axum::{
  extract::{Request, State},
  middleware::Next,
  response::Response,
};

use crate::{
  app::AppState,
  errors::AppError,
  auth::decode_jwt,
  user::{ find_user_by_id, UserResponse },
};

/// Middleware that authenticates the request by validating the `Authorization: Bearer` token,
/// then injects the resolved [`UserResponse`] as a request extension.
/// Returns `401 Unauthorized` if the token is missing, invalid, or the user no longer exists.
pub async fn get_current_user(
  mut req: Request,
  next: Next,
) -> Result<Response, AppError> {

  let token = req
    .headers()
    .get("Authorization")
    .and_then(|v| v.to_str().ok())
    .and_then(|v| v.strip_prefix("Bearer "))
    .ok_or(AppError::Unauthorized)?;

  let claims = decode_jwt(token).map_err(|_| AppError::Unauthorized)?;

  req.extensions_mut().insert(claims);

  Ok(next.run(req).await)
}