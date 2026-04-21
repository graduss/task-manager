use axum::{
  extract::{ FromRequest, Request },
  Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::errors::AppError;

pub struct SafeQuery<T>(pub T);

impl<S, T> FromRequest<S> for SafeQuery<T>
where
  T: DeserializeOwned + Validate + Send,
  S: Send + Sync,
{
  type Rejection = AppError;

  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    let Json(query) = Json::<T>::from_request(req, state).await
      .map_err(|err| {
        AppError::BadRequest(err.to_string())
      })?;
    Ok(SafeQuery(query))
  }
}