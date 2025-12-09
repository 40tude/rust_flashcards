// Rust guideline compliant 2025-01
use std::env;

/// Application configuration loaded from environment variables.
///
/// Provides deck path resolution and configuration priorities:
/// CLI args > Environment variables > Default values
#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub deck_id: String,
    pub deck_display_name: String,
    pub md_path: String,
    pub img_path: String,
}

impl Config {
    /// Loads configuration from CLI arguments and environment variables.
    ///
    /// # Configuration Priority
    /// 1. CLI arguments (highest priority)
    /// 2. Environment variables (DECK_ID, DECK_DISPLAY_NAME, DATABASE_URL, PORT)
    /// 3. Default values (deck, "Data Science Flashcards", "./deck.db", 8080)
    ///
    /// # Examples
    /// ```no_run
    /// use config::Config;
    /// let config = Config::from_env(Some("rust".to_string()), None).unwrap();
    /// println!("Markdown path: {}", config.md_path);
    /// ```
    ///
    /// # Errors
    /// Returns error if PORT environment variable is invalid u16.
    pub fn from_env(cli_deck: Option<String>, cli_deck_name: Option<String>) -> anyhow::Result<Self> {
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("PORT must be a valid u16");

        // Priority: CLI > Env Var > Default
        let deck_id = cli_deck
            .or_else(|| env::var("DECK_ID").ok())
            .unwrap_or_else(|| "deck".to_string());

        let deck_display_name = cli_deck_name
            .or_else(|| env::var("DECK_DISPLAY_NAME").ok())
            .or_else(|| env::var("DECK_NAME").ok()) // Backward compatibility
            .unwrap_or_else(|| deck_id.clone()); // Default to deck_id if nothing specified

        // Use DATABASE_URL only if it's NOT a local .db file (e.g., Heroku Postgres URL)
        // For local development with multiple decks, always use ./{deck_id}.db
        let database_url = env::var("DATABASE_URL")
            .ok()
            .filter(|url| !url.ends_with(".db"))
            .unwrap_or_else(|| format!("./{}.db", deck_id));

        // Compute content paths based on deck_id
        let md_path = format!("./static/{}/md", deck_id);
        let img_path = format!("./static/{}/img", deck_id);

        Ok(Config {
            port,
            database_url,
            deck_id,
            deck_display_name,
            md_path,
            img_path,
        })
    }
}
