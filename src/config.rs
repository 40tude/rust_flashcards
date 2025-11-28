use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    #[allow(dead_code)]
    pub secret_key: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let secret_key = env::var("FLASHCARDS_SECRET_KEY")
            .expect("FLASHCARDS_SECRET_KEY must be set in .env file");

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a valid u16");

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "./flashcards.db".to_string());

        Ok(Config {
            secret_key,
            port,
            database_url,
        })
    }
}