mod config;
mod content;
mod db;
mod routes;
mod session;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_sessions::{MemoryStore, SessionManagerLayer};

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

    // Setup session store
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_name("flashcards_session");

    // Build Axum router
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/next", get(routes::next))
        .route("/reset_session", get(routes::reset_session))
        .route("/search", get(routes::search_form).post(routes::search_submit))
        .route("/search_results", get(routes::search_results))
        .nest_service("/static", ServeDir::new("static"))
        .layer(session_layer)
        .with_state(pool);

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on http://{}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
