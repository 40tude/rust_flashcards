mod config;
mod content;
mod db;
mod routes;
mod session;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_sessions::{MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env
    dotenvy::dotenv().ok();

    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")))
        .init();

    tracing::info!("Starting rust-flashcards application");

    // Load configuration
    let config = config::Config::from_env()?;
    tracing::info!("Configuration loaded: port={}, database={}", config.port, config.database_url);

    // Create database connection pool
    let pool = db::create_pool(&config.database_url)?;

    // Initialize database schema
    db::init_database(&pool)?;

    // Load content only if database is empty (fast startup optimization)
    if db::queries::is_database_empty(&pool)? {
        tracing::info!("No database in place, create it");

        // Validate content directories before attempting to load
        let md_status = content::validate_content_directory("./static/md");
        let img_status = content::validate_content_directory("./static/img");

        // Check if at least one directory is valid
        let has_valid_content = matches!(md_status, content::ContentDirStatus::Valid) || matches!(img_status, content::ContentDirStatus::Valid);

        if !has_valid_content {
            // Both directories are invalid - exit with error
            eprintln!("\nERROR: Cannot start application - no content directories found\n");
            eprintln!("The application requires at least one of the following directories:");
            eprintln!("  - ./static/md/  (for markdown flashcards)");
            eprintln!("  - ./static/img/ (for image-only flashcards)\n");
            eprintln!("Current status:");
            eprintln!("  ./static/md  -> {:?}", md_status);
            eprintln!("  ./static/img -> {:?}\n", img_status);
            eprintln!("Setup instructions:");
            eprintln!("  1. Create at least one directory:");
            eprintln!("     mkdir static/md");
            eprintln!("     OR");
            eprintln!("     mkdir static/img\n");
            eprintln!("  2. Add content files:");
            eprintln!("     - For markdown: place .md files in static/md/");
            eprintln!("     - For images: place .png or .webp files in static/img/\n");
            eprintln!("For more information, see the README.md file.");
            std::process::exit(1);
        }

        // Load content from valid directories
        if matches!(md_status, content::ContentDirStatus::Valid) {
            content::load_markdown(&pool, "./static/md")?;
        } else {
            tracing::warn!("Content directory unavailable: ./static/md (reason: {:?})", md_status);
            tracing::warn!("Continuing with image-only flashcards");
        }

        if matches!(img_status, content::ContentDirStatus::Valid) {
            content::load_images(&pool, "./static/img")?;
        } else {
            tracing::warn!("Content directory unavailable: ./static/img (reason: {:?})", img_status);
            tracing::warn!("Continuing with markdown-only flashcards");
        }

        db::queries::populate_fts_table(&pool)?;
    } else {
        let count = db::queries::get_total_count(&pool)?;
        tracing::info!("Database in place no need to create it - {} cards loaded", count);
    }

    tracing::info!("Content loaded successfully. Starting web server...");

    // Setup session store
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false).with_name("flashcards_session");

    // Save port before moving config
    let port = config.port;

    // Create app state with config and pool
    let app_state = routes::AppState {
        pool,
        config,
    };

    // Build Axum router
    let app = Router::new()
        .route("/", get(routes::landing))
        .route("/apply_filters", post(routes::apply_filters))
        .route("/practice", get(routes::practice))
        .route("/reset_session", get(routes::reset_session))
        .nest_service("/static", ServeDir::new("static"))
        .layer(session_layer)
        .with_state(app_state);

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server listening on http://{}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
