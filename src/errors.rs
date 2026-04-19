//! Centralised error handling. [`AppError`] covers all failure modes and implements
//! [`IntoResponse`] to emit structured JSON with the appropriate HTTP status code.
//! [`AppResult<T>`] is a convenience alias used across handlers and services.

use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("NotFound: {0}")]
  NotFound(String),

  #[error("Unauthorized")]
  Unauthorized,

  #[error("Conflict: {0}")]
  Conflict(String),

  #[error("BadRequest: {0}")]
  BadRequest(String),

  #[error("InternalServerError: {0}")]
  InternalServerError(#[from] anyhow::Error),

  #[error("DatabaseError: {0}")]
  DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
  /// Maps each error variant to an HTTP status code and a JSON `{ "error": "..." }` body.
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      AppError::NotFound(message) => (StatusCode::NOT_FOUND, message.clone()),
      AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
      AppError::Conflict(message) => (StatusCode::CONFLICT, message.clone()),
      AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message.clone()),
      AppError::InternalServerError(err) => {
        tracing::error!("Internal server error: {err}");
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into())
      }
      AppError::DatabaseError(err) => {
        tracing::error!("Database error: {err}");
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into())
      }
    };

    let body = Json(serde_json::json!({ "error": error_message }));
    (status, body).into_response()
  }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(e: validator::ValidationErrors) -> Self {
        AppError::BadRequest(e.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;