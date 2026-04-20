use async_trait::async_trait;
use axum::extract::{ FromRequestParts, Query, Parts };
use crate::errors::AppError;

struct SafeQuery;

#[async_trait]
impl<S> FromRequestParts<S> for SafeQuery
where
  S: Send + Sync,
{
  type Rejection = AppError;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    let query = Query::<T>::from_request_parts(parts, _state).await?;
    Ok(SafeQuery)
  }
}