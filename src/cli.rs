// Rust guideline compliant 2025-11-29

use clap::Parser;

/// Flashcard web application with full-text search
#[derive(Parser, Debug)]
#[command(name = "rust-flashcards")]
#[command(version)]
#[command(about = "Flashcard web application with full-text search", long_about = None)]
pub struct Cli {
    /// Rebuild database by deleting existing DB file before startup
    #[arg(short = 'r', long = "rebuild-db")]
    pub rebuild_db: bool,

    // Future args (commented out for now):
    // /// Database file path (overrides DATABASE_URL)
    // #[arg(long, value_name = "PATH")]
    // pub database: Option<String>,
    //
    // /// Server port (overrides PORT env var)
    // #[arg(short, long, value_name = "PORT")]
    // pub port: Option<u16>,
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
