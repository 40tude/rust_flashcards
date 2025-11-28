// Rust guideline compliant 2025-01
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub deck_name: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("PORT must be a valid u16");

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "./flashcards.db".to_string());

        let deck_name = env::var("DECK_NAME").unwrap_or_else(|_| "Data Science Flashcards".to_string());

        Ok(Config { port, database_url, deck_name })
    }
}
