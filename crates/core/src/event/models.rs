use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "event_status", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum EventStatus {
  Pending,
  Processing,
  Completed,
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "event_type", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum EventType {
  CreateTask,
  UpdateTask,
  DeleteTask,
}

pub struct CreateEvent {
  pub task_id: Uuid,
  pub event_type: EventType,
}


pub struct Event {
  pub id: Uuid,
  pub task_id: Uuid,
  pub event_type: EventType,
  pub status: EventStatus,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}