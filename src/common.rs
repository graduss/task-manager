use axum::{
  extract::{ FromRequest, Request, FromRequestParts, Query },
  http::request::Parts,
  Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::errors::AppError;

pub struct SafeJson<T>(pub T);

pub struct SafeQuery<T>(pub T);

impl<S, T> FromRequest<S> for SafeJson<T>
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
    Ok(SafeJson(query))
  }
}

impl <S, T> FromRequestParts<S> for SafeQuery<T>
where
  T: DeserializeOwned + Validate + Send,
  S: Send + Sync,
{
  type Rejection = AppError;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let Query(query) = Query::<T>::from_request_parts(parts, state).await
      .map_err(|err| {
        AppError::BadRequest(err.to_string())
      })?;
    Ok(SafeQuery(query))
  }
}