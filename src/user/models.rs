use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub password_hash: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewUser {
  pub username: String,
  pub email: String,
  pub password_hash: String,
}

#[derive(Serialize)]
pub struct UserResponse {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
  fn from(user: User) -> Self {
    UserResponse {
      id: user.id,
      username: user.username,
      email: user.email,
      created_at: user.created_at,
      updated_at: user.updated_at,
    }
  }
}