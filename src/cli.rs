// Rust guideline compliant 2025-11-29

use clap::Parser;

/// Flashcard web application with multi-deck support
#[derive(Parser, Debug)]
#[command(name = "rust-flashcards")]
#[command(version)]
#[command(about = "Flashcard web application with full-text search and multi-deck support", long_about = None)]
pub struct Cli {
    /// Rebuild deck by deleting existing DB file before startup
    #[arg(short = 'r', long = "rebuild-deck-id", value_name = "DECK_ID")]
    pub rebuild_deck: Option<String>,

    /// Deck to load (directory name under ./static/)
    #[arg(short = 'd', long = "deck-id", value_name = "DECK_ID")]
    pub deck: Option<String>,

    /// Display name for deck in HTML (overrides deck directory name)
    #[arg(short = 'n', long = "deck-display-name", value_name = "DECK_DISPLAY_NAME")]
    pub deck_name: Option<String>,
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
