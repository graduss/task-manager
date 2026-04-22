use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use common_core::db;
use common_core::event;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "task_manager=info,tower_http=info".into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;
    let db_pool = db::create_db_pool(&database_url).await?;

    tracing::info!("Worker started, connected to database");

    loop {
        let mut tx = db_pool.begin().await?;

        match event::services::get_pending_events(&mut tx).await {
            Ok(events) => {
                for event in events {
                    tracing::info!("Processing event: {:?}", event.event_type);
                    // Simulate processing time
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                    // Here you would add logic to handle the event based on its type
                    // For example, if it's a CreateTask event, you would create a new task in the database

                    // After processing, you would update the event status to completed
                    // This is just a placeholder and should be replaced with actual update logic
                    tracing::info!("Completed event: {:?}", event.event_type);
                }
            },
            Err(e) => {
                tracing::error!("Failed to fetch pending events: {:?}", e);
                tx.rollback().await?;
                continue;
            }
        }

        tx.commit().await?;

        // Sleep for a short duration before checking for new events
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}