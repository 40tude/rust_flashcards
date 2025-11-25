mod config;
mod content;
mod db;
mod routes;

use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env
    dotenvy::dotenv().ok();

    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Starting rust-flashcards application");

    // Load configuration
    let config = config::Config::from_env()?;
    tracing::info!("Configuration loaded: port={}, database={}", config.port, config.database_url);

    // Create database connection pool
    let pool = db::create_pool(&config.database_url)?;

    // Initialize database schema
    db::init_database(&pool)?;

    // Load content from markdown and PNG files
    tracing::info!("Loading content...");
    content::load_markdown(&pool, "./static/md")?;
    content::load_images(&pool, "./static/png")?;

    // Populate FTS table
    db::queries::populate_fts_table(&pool)?;

    tracing::info!("Content loaded successfully. Starting web server...");

    // Build Axum router
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"));

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on http://{}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
