//! Database connection pool. Creates a [`PgPool`] with a maximum of 10 connections.

use sqlx::{PgPool, postgres::PgPoolOptions};

pub type DbPool = PgPool;

/// Creates a PostgreSQL connection pool capped at 10 connections.
pub async fn create_db_pool(url: &str) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await?;

    Ok(pool)
}