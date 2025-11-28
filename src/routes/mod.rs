pub mod debug;
pub mod landing;
pub mod practice;

pub use debug::reset_session;
pub use landing::{apply_filters, landing};
pub use practice::practice;

use crate::config::Config;
use crate::db::connection::DbPool;

/// Shared application state passed to all route handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub config: Config,
}
