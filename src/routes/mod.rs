pub mod debug;
pub mod landing;
pub mod practice;

pub use debug::reset_session;
pub use landing::{apply_filters, landing};
pub use practice::practice;

use crate::config::Config;
use crate::db::connection::DbPool;

/// Shared application state for route handlers.
///
/// Contains database connection pool and configuration.
/// Cloned for each request handler via Axum's `State` extractor.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool for querying flashcards.
    pub pool: DbPool,
    /// Application configuration loaded from environment.
    pub config: Config,
}
