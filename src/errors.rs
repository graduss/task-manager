//! Centralised error handling. [`AppError`] covers all failure modes and implements
//! [`IntoResponse`] to emit structured JSON with the appropriate HTTP status code.
//! [`AppResult<T>`] is a convenience alias used across handlers and services.

use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  extract::rejection::QueryRejection,
  Json,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("NotFound")]
  NotFound,

  #[error("Unauthorized")]
  Unauthorized,

  #[error("Conflict: {0}")]
  Conflict(String),

  #[error("BadRequest: {0}")]
  BadRequest(String),

  #[error("ValidationError: {0}")]
  ValidationError(#[from] validator::ValidationErrors),

  #[error("InternalServerError: {0}")]
  InternalServerError(#[from] anyhow::Error),

  #[error("DatabaseError: {0}")]
  DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
  /// Maps each error variant to an HTTP status code and a JSON `{ "error": "..." }` body.
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found".into()),
      AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
      AppError::Conflict(message) => (StatusCode::CONFLICT, message.clone()),
      AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message.clone()),
      AppError::ValidationError(err) => {
        tracing::error!("Validation error: {:?}", err);
        (StatusCode::BAD_REQUEST, err.to_string())
      }
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

impl From<QueryRejection> for AppError {
  fn from(e: QueryRejection) -> Self {
    tracing::error!("Query extraction error: {:?}", e);
    AppError::BadRequest(e.to_string())
  }
}

pub type AppResult<T> = Result<T, AppError>;