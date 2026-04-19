use uuid::Uuid;

use super::models::{NewUser, User, UserResponse};
use crate::{
  db::DbPool,
  errors::AppError,
};

pub async fn create_user(db_pool: &DbPool, new_user: NewUser) -> Result<UserResponse, AppError> {
  let exists = sqlx::query_scalar!(
    r#"
    SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 OR email = $2)
    "#,
    new_user.username,
    new_user.email
  )
  .fetch_one(db_pool)
  .await?;

  if let Some(e) = exists && e {
    return Err(AppError::Conflict("Username or email already exists".into()));
  }

  let user: User = sqlx::query_as!(
    User,
    r#"
    INSERT INTO users (username, email, password_hash)
    VALUES ($1, $2, $3)
    RETURNING *"#,
    new_user.username,
    new_user.email,
    new_user.password_hash
  )
  .fetch_one(db_pool)
  .await?;

  Ok(user.into())
}

pub async fn find_user_by_email(db_pool: &DbPool, email: &str) -> Result<Option<User>, AppError> {
  let user: Option<User> = sqlx::query_as!(
    User,
    r#"
    SELECT * FROM users WHERE email = $1
    "#,
    email
  )
  .fetch_optional(db_pool)
  .await?;

  Ok(user)
}

pub async fn find_user_by_id(db_pool: &DbPool, id: Uuid) -> Result<Option<User>, AppError> {
  let user: Option<User> = sqlx::query_as!(
    User,
    r#"
    SELECT * FROM users WHERE id = $1
    "#,
    id
  )
  .fetch_optional(db_pool)
  .await?;

  Ok(user)
}