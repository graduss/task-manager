//! Application entry point. Loads environment variables, initialises tracing,
//! creates the database pool, attaches middleware, and starts the HTTP listener.

mod app;
mod db;
mod auth;
mod errors;
mod user;
mod meddelware;

use dotenvy::dotenv;
use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "task_manager=info,tower_http=info".into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;
    let db_pool = db::create_db_pool(&database_url).await?;

    let app = app::create_app(app::AppState { db_pool })
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr = std::env::var("APP_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Starting server at {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
