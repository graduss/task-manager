use std::task;

use sqlx::Execute;

use crate::{
  app::AppState,
  errors::AppError,
};

use super::models::{
  CreateTaskRequest,
  Task,
  TaskStatus,
  TasksQuery,
  TasksResponse,
};

pub async fn create_task(
  app_state: AppState,
  user_id: uuid::Uuid,
  task: CreateTaskRequest,
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

pub async fn list_tasks(
  app_state: AppState,
  user_id: uuid::Uuid,
  params: TasksQuery,
) -> Result<TasksResponse, AppError> {
  let mut query = String::from(
    r#"SELECT id, user_id, title, description, status, created_at, updated_at FROM tasks WHERE user_id = $1"#,
  );

  let mut n = 1u32;

  if params.status.is_some() {
    n += 1;
    query.push_str(&format!(" AND status = ${n}"));
  }

  if params.search.is_some() {
    n += 1;
    query.push_str(&format!(" AND (title ILIKE ${n} OR description ILIKE ${n})"));
  }

  let offset = params.page * params.page_size;

  n += 1;
  query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${n}"));
  n += 1;
  query.push_str(&format!(" OFFSET ${n}"));

  let mut q = sqlx::query_as(&query).bind(user_id);

  if let Some(status) = params.status {
    q = q.bind(status);
  }

  if let Some(search) = params.search {
    q = q.bind(format!("%{}%", search));
  }

  let tasks = q
    .bind(params.page_size + 1)
    .bind(offset)
    .fetch_all(&app_state.db_pool).await?;

  let has_next = tasks.len() as i32 > params.page_size;

  Ok(TasksResponse {
    tasks: tasks.into_iter().take(params.page_size as usize).collect(),
    page: params.page as u32,
    page_size: params.page_size as u32,
    has_next,
  })
}