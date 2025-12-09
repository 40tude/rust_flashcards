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
    /// Loads configuration from environment variables.
    ///
    /// # Configuration Priority
    /// 1. Environment variables (DECK_ID, DECK_DISPLAY_NAME, DATABASE_URL, PORT)
    /// 2. Default values (deck, "Data Science Flashcards", "./flashcards.db", 8080)
    ///
    /// # Examples
    /// ```no_run
    /// use config::Config;
    /// let config = Config::from_env().unwrap();
    /// println!("Markdown path: {}", config.md_path);
    /// ```
    ///
    /// # Errors
    /// Returns error if PORT environment variable is invalid u16.
    pub fn from_env() -> anyhow::Result<Self> {
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("PORT must be a valid u16");

        let deck_id = env::var("DECK_ID").unwrap_or_else(|_| "deck".to_string());

        let deck_display_name = env::var("DECK_DISPLAY_NAME")
            .or_else(|_| env::var("DECK_NAME")) // Backward compatibility
            .unwrap_or_else(|_| "Data Science Flashcards".to_string());

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "./flashcards.db".to_string());

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
