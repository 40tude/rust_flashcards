pub mod connection;
pub mod models;
pub mod schema;

// Re-export commonly used items
pub use connection::{create_pool, DbPool};
pub use models::Flashcard;
pub use schema::{init_database, populate_fts_table};