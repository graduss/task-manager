use uuid::Uuid;

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
  TaskUpdate,
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

pub async fn update_task(
  app_state: AppState,
  user_id: Uuid,
  task_id: Uuid,
  task_update: TaskUpdate,
) -> Result<Task, AppError> {
  let task = sqlx::query_as!(
    Task,
    r#"
    UPDATE tasks
    SET title = COALESCE($1, title),
        description = COALESCE($2, description),
        status = COALESCE($3, status),
        updated_at = NOW()
    WHERE id = $4 AND user_id = $5
    RETURNING id, user_id, title, description, status as "status: TaskStatus", created_at, updated_at
    "#,
    task_update.title,
    task_update.description,
    task_update.status as Option<TaskStatus>,
    task_id,
    user_id
  )
  .fetch_optional(&app_state.db_pool)
  .await?;

  match task {
    Some(task) => Ok(task),
    None => Err(AppError::NotFound),
  }
}

pub async fn get_task(
  app_state: AppState,
  user_id: Uuid,
  task_id: Uuid,
) -> Result<Task, AppError> {
  let task = sqlx::query_as!(
    Task,
    r#"
    SELECT id, user_id, title, description, status as "status: TaskStatus", created_at, updated_at
    FROM tasks
    WHERE id = $1 AND user_id = $2
    "#,
    task_id,
    user_id
  )
  .fetch_optional(&app_state.db_pool)
  .await?;

  match task {
    Some(task) => Ok(task),
    None => Err(AppError::NotFound),
  }
}

pub async fn delete_task(
  app_state: AppState,
  user_id: Uuid,
  task_id: Uuid,
) -> Result<(), AppError> {
  let result = sqlx::query!(
    r#"
    DELETE FROM tasks
    WHERE id = $1 AND user_id = $2
    "#,
    task_id,
    user_id
  )
  .execute(&app_state.db_pool)
  .await?;

  if result.rows_affected() == 0 {
    Err(AppError::NotFound)
  } else {
    Ok(())
  }
}
