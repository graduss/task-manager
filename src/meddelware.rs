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

pub async fn get_current_user(
  State(app_state): State<AppState>,
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
  let user: UserResponse = find_user_by_id(&app_state.db_pool, claims.sub).await?
    .ok_or(AppError::Unauthorized)?
    .into();

  req.extensions_mut().insert(user);

  Ok(next.run(req).await)
}