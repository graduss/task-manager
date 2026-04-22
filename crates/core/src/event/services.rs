use sqlx::{ Postgres, Transaction };

use super::models::{ CreateEvent, EventType, Event, EventStatus };

pub async fn create_event(
  tx: &mut Transaction<'_, Postgres>,
  event: CreateEvent
) -> Result<(), sqlx::Error> {
  let _res = sqlx::query!(
    r#"
    INSERT INTO events (task_id, type)
    VALUES ($1, $2::event_type)
    "#,
    event.task_id,
    event.event_type as EventType,
  ).execute(&mut **tx)
  .await?;

  Ok(())
}

pub async fn get_pending_events(
  tx: &mut Transaction<'_, Postgres>,
) -> Result<Vec<Event>, sqlx::Error> {
  let rows = sqlx::query_as!(
    Event,
    r#"
    SELECT id, task_id, type as "event_type: EventType", status as "status: EventStatus", created_at, updated_at
    FROM events
    WHERE status = 'pending'
    ORDER BY created_at ASC
    LIMIT 10
    FOR UPDATE SKIP LOCKED
    "#
  )
  .fetch_all(&mut **tx)
  .await?;

  Ok(rows)
}