use crate::{
  app::AppState,
  errors::AppError,
};

use super::models::{
  CreateTaskRequest,
  Task,
  TaskStatus,
};

pub async fn create_task(
  app_state: &AppState,
  user_id: &uuid::Uuid,
  task: &CreateTaskRequest,
) -> Result<Task, AppError> {
  let task = sqlx::query_as!(
    Task,
    r#"
    INSERT INTO tasks (user_id, title, description)
    VALUES ($1, $2, $3)
    RETURNING id, user_id, title, description, status as "status: TaskStatus", created_at, updated_at
    "#,
    user_id,
    task.title,
    task.description
  )
  .fetch_one(&app_state.db_pool)
  .await?;

  Ok(task)
}