use chrono::{DateTime, Utc};
use serde::{ Serialize, Deserialize };
use validator::Validate;
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Task {
  pub id: Uuid,
  pub user_id: Uuid,
  pub title: String,
  pub description: Option<String>,
  pub status: TaskStatus,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct CreateTaskRequest {
  #[validate(length(min = 1, max = 255, message = "Title cannot be empty and must be less than 255 characters"))]
  pub title: String,
  pub description: Option<String>,
}

/// Query parameters for listing tasks, with optional filters and pagination.
#[derive(Deserialize, Validate)]
pub struct TasksQuery {
  pub status: Option<TaskStatus>,
  pub search: Option<String>,
  #[validate(range(min = 0, message = "Page must be a non-negative integer"))]
  pub page: i32,
  #[validate(range(min = 1, max = 100, message = "Page size must be between 1 and 100"))]
  pub page_size: i32,
}

#[derive(Serialize)]
pub struct TasksResponse {
  pub tasks: Vec<Task>,
  pub page: u32,
  pub page_size: u32,
  pub has_next: bool,
}
