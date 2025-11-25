mod config;
mod db;

fn main() -> anyhow::Result<()> {
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

    tracing::info!("Phase 1 complete: Foundation & Setup successful!");

    Ok(())
}
