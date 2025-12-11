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
    /// use rust_flashcards::config::Config;
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
            .clone()
            .or_else(|| env::var("DECK_ID").ok())
            .unwrap_or_else(|| "deck".to_string());

        // For display name: if deck_id came from CLI, ignore env vars and default to deck_id
        // This ensures --deck-id test_42 shows "test_42", not env DECK_DISPLAY_NAME
        let deck_display_name = if cli_deck_name.is_some() {
            // Explicit CLI display name provided
            cli_deck_name.unwrap()
        } else if cli_deck.is_some() {
            // CLI deck_id provided but no display name → use deck_id
            deck_id.clone()
        } else {
            // No CLI args → use env vars or default to deck_id
            env::var("DECK_DISPLAY_NAME")
                .or_else(|_| env::var("DECK_NAME")) // Backward compatibility
                .unwrap_or_else(|_| deck_id.clone())
        };

        tracing::info!("Config resolution: deck_id={}, deck_display_name={}, DECK_DISPLAY_NAME={:?}, DECK_NAME={:?}",
            deck_id, deck_display_name, env::var("DECK_DISPLAY_NAME"), env::var("DECK_NAME"));

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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serial_test::serial;
    use std::env;

    /// Helper to set and clear environment variables for testing.
    ///
    /// Automatically clears env vars when dropped to prevent test pollution.
    struct EnvGuard {
        keys: Vec<String>,
    }

    impl EnvGuard {
        fn new() -> Self {
            // Clear all config-related env vars at start to ensure clean state
            // SAFETY: Tests run sequentially, no concurrent access to env vars
            unsafe {
                env::remove_var("PORT");
                env::remove_var("DECK_ID");
                env::remove_var("DECK_DISPLAY_NAME");
                env::remove_var("DECK_NAME");
                env::remove_var("DATABASE_URL");
            }
            Self { keys: Vec::new() }
        }

        fn set(&mut self, key: &str, value: &str) {
            // SAFETY: Tests run sequentially, no concurrent access to env vars
            unsafe {
                env::set_var(key, value);
            }
            self.keys.push(key.to_string());
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            // SAFETY: Tests run sequentially, no concurrent access to env vars
            unsafe {
                for key in &self.keys {
                    env::remove_var(key);
                }
                // Also clean up standard env vars used in tests
                env::remove_var("PORT");
                env::remove_var("DECK_ID");
                env::remove_var("DECK_DISPLAY_NAME");
                env::remove_var("DECK_NAME");
                env::remove_var("DATABASE_URL");
            }
        }
    }

    // ========== Parametrized Tests for Configuration Priority ==========

    #[rstest]
    // (cli_deck, cli_name, env_deck, env_name, expected_deck, expected_name)
    #[case(Some("rust"), Some("Rust Cards"), None, None, "rust", "Rust Cards")]
    #[case(Some("rust"), None, None, None, "rust", "rust")]
    #[case(None, None, Some("py"), Some("Python"), "py", "Python")]
    #[case(Some("test"), None, Some("py"), Some("Python"), "test", "test")]
    #[case(None, None, None, None, "deck", "deck")]
    #[case(Some("cli_deck"), Some("CLI Name"), Some("env_deck"), Some("Env Name"), "cli_deck", "CLI Name")]
    #[case(Some("cli_deck"), None, Some("env_deck"), Some("Env Name"), "cli_deck", "cli_deck")]
    #[case(None, Some("Name Only"), Some("env_deck"), Some("Env Name"), "env_deck", "Name Only")]
    #[serial]
    fn test_config_priority(
        #[case] cli_deck: Option<&str>,
        #[case] cli_name: Option<&str>,
        #[case] env_deck: Option<&str>,
        #[case] env_name: Option<&str>,
        #[case] expected_deck: &str,
        #[case] expected_name: &str,
    ) {
        let mut guard = EnvGuard::new();

        // Set environment variables if provided
        if let Some(deck) = env_deck {
            guard.set("DECK_ID", deck);
        }
        if let Some(name) = env_name {
            guard.set("DECK_DISPLAY_NAME", name);
        }

        // Call from_env with CLI args
        let config = Config::from_env(
            cli_deck.map(|s| s.to_string()),
            cli_name.map(|s| s.to_string()),
        )
        .unwrap();

        assert_eq!(config.deck_id, expected_deck);
        assert_eq!(config.deck_display_name, expected_name);
    }

    #[test]
    #[serial]
    fn test_cli_deck_overrides_env_vars() {
        let mut guard = EnvGuard::new();
        guard.set("DECK_ID", "env_deck");
        guard.set("DECK_DISPLAY_NAME", "Env Name");

        let config = Config::from_env(Some("cli_deck".to_string()), None).unwrap();

        assert_eq!(config.deck_id, "cli_deck");
        // When CLI deck provided but no name, use deck_id
        assert_eq!(config.deck_display_name, "cli_deck");
    }

    #[test]
    #[serial]
    fn test_cli_name_with_env_deck() {
        let mut guard = EnvGuard::new();
        guard.set("DECK_ID", "env_deck");

        let config = Config::from_env(None, Some("CLI Name".to_string())).unwrap();

        // CLI name alone doesn't override env deck
        assert_eq!(config.deck_id, "env_deck");
        assert_eq!(config.deck_display_name, "CLI Name");
    }

    #[test]
    #[serial]
    fn test_default_values_when_no_args_or_env() {
        let _guard = EnvGuard::new();
        // Ensure no env vars set

        let config = Config::from_env(None, None).unwrap();

        assert_eq!(config.deck_id, "deck");
        assert_eq!(config.deck_display_name, "deck");
        assert_eq!(config.port, 8080);
    }

    // ========== Tests for Database URL Handling ==========

    #[test]
    #[serial]
    fn test_database_url_local_db_default() {
        let _guard = EnvGuard::new();

        let config = Config::from_env(Some("test".to_string()), None).unwrap();

        // Should use local .db file format
        assert_eq!(config.database_url, "./test.db");
    }

    #[test]
    #[serial]
    fn test_database_url_external_postgres() {
        let mut guard = EnvGuard::new();
        guard.set("DATABASE_URL", "postgresql://user:pass@host/db");

        let config = Config::from_env(None, None).unwrap();

        // Should use external URL (not ending with .db)
        assert_eq!(config.database_url, "postgresql://user:pass@host/db");
    }

    #[test]
    #[serial]
    fn test_database_url_local_db_file_ignored() {
        let mut guard = EnvGuard::new();
        guard.set("DATABASE_URL", "./custom.db");

        let config = Config::from_env(Some("mydeck".to_string()), None).unwrap();

        // Should ignore DATABASE_URL ending with .db and use deck-based name
        assert_eq!(config.database_url, "./mydeck.db");
    }

    #[test]
    #[serial]
    fn test_database_url_respects_deck_id() {
        let _guard = EnvGuard::new();

        let config1 = Config::from_env(Some("deck1".to_string()), None).unwrap();
        let config2 = Config::from_env(Some("deck2".to_string()), None).unwrap();

        assert_eq!(config1.database_url, "./deck1.db");
        assert_eq!(config2.database_url, "./deck2.db");
    }

    // ========== Tests for Path Generation ==========

    #[test]
    #[serial]
    fn test_path_generation_matches_deck_id() {
        let _guard = EnvGuard::new();

        let config = Config::from_env(Some("rust".to_string()), None).unwrap();

        assert_eq!(config.md_path, "./static/rust/md");
        assert_eq!(config.img_path, "./static/rust/img");
    }

    #[test]
    #[serial]
    fn test_path_generation_default_deck() {
        let _guard = EnvGuard::new();

        let config = Config::from_env(None, None).unwrap();

        assert_eq!(config.md_path, "./static/deck/md");
        assert_eq!(config.img_path, "./static/deck/img");
    }

    #[test]
    #[serial]
    fn test_path_generation_with_env_deck() {
        let mut guard = EnvGuard::new();
        guard.set("DECK_ID", "python");

        let config = Config::from_env(None, None).unwrap();

        assert_eq!(config.md_path, "./static/python/md");
        assert_eq!(config.img_path, "./static/python/img");
    }

    // ========== Tests for Port Parsing ==========

    #[test]
    #[serial]
    fn test_port_default_8080() {
        let _guard = EnvGuard::new();

        let config = Config::from_env(None, None).unwrap();

        assert_eq!(config.port, 8080);
    }

    #[test]
    #[serial]
    fn test_port_from_env_var() {
        let mut guard = EnvGuard::new();
        guard.set("PORT", "3000");

        let config = Config::from_env(None, None).unwrap();

        assert_eq!(config.port, 3000);
    }

    #[test]
    #[serial]
    #[should_panic(expected = "PORT must be a valid u16")]
    fn test_port_invalid_panics() {
        let mut guard = EnvGuard::new();
        guard.set("PORT", "invalid");

        let _config = Config::from_env(None, None).unwrap();
    }

    #[test]
    #[serial]
    #[should_panic(expected = "PORT must be a valid u16")]
    fn test_port_out_of_range_panics() {
        let mut guard = EnvGuard::new();
        guard.set("PORT", "70000");

        let _config = Config::from_env(None, None).unwrap();
    }

    // ========== Tests for Backward Compatibility ==========

    #[test]
    #[serial]
    fn test_backward_compatibility_deck_name() {
        let mut guard = EnvGuard::new();
        guard.set("DECK_NAME", "Legacy Name");

        let config = Config::from_env(None, None).unwrap();

        // DECK_NAME should be used if DECK_DISPLAY_NAME not set
        assert_eq!(config.deck_display_name, "Legacy Name");
    }

    #[test]
    #[serial]
    fn test_deck_display_name_overrides_deck_name() {
        let mut guard = EnvGuard::new();
        guard.set("DECK_NAME", "Legacy Name");
        guard.set("DECK_DISPLAY_NAME", "New Name");

        let config = Config::from_env(None, None).unwrap();

        // DECK_DISPLAY_NAME takes priority over DECK_NAME
        assert_eq!(config.deck_display_name, "New Name");
    }

    // ========== Integration Tests ==========

    #[test]
    #[serial]
    fn test_full_config_structure() {
        let mut guard = EnvGuard::new();
        guard.set("PORT", "5000");
        guard.set("DECK_ID", "test");
        guard.set("DECK_DISPLAY_NAME", "Test Deck");

        let config = Config::from_env(None, None).unwrap();

        assert_eq!(config.port, 5000);
        assert_eq!(config.deck_id, "test");
        assert_eq!(config.deck_display_name, "Test Deck");
        assert_eq!(config.database_url, "./test.db");
        assert_eq!(config.md_path, "./static/test/md");
        assert_eq!(config.img_path, "./static/test/img");
    }

    #[test]
    #[serial]
    fn test_cli_overrides_all_env_vars() {
        let mut guard = EnvGuard::new();
        guard.set("DECK_ID", "env_deck");
        guard.set("DECK_DISPLAY_NAME", "Env Name");

        let config = Config::from_env(
            Some("cli_deck".to_string()),
            Some("CLI Name".to_string()),
        )
        .unwrap();

        assert_eq!(config.deck_id, "cli_deck");
        assert_eq!(config.deck_display_name, "CLI Name");
        assert_eq!(config.database_url, "./cli_deck.db");
        assert_eq!(config.md_path, "./static/cli_deck/md");
        assert_eq!(config.img_path, "./static/cli_deck/img");
    }

    // ========== Property-Based Tests ==========

    /// Property-based tests using proptest for config invariants.
    ///
    /// Verifies that config resolution maintains critical properties across
    /// randomly generated inputs.
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        /// Generates valid deck IDs (lowercase alphanumeric with underscores).
        fn arb_deck_id() -> impl Strategy<Value = String> {
            "[a-z0-9_]{3,20}"
        }

        /// Generates valid display names (alphanumeric with spaces).
        fn arb_display_name() -> impl Strategy<Value = String> {
            "[A-Za-z0-9 ]{5,30}"
        }

        /// Tests that CLI deck always overrides environment variable.
        ///
        /// Invariant: When CLI deck_id is provided, result.deck_id must equal CLI value
        /// regardless of DECK_ID environment variable.
        #[test]
        #[serial]
        fn prop_cli_deck_always_overrides_env() {
            proptest!(|(cli_deck in arb_deck_id(), env_deck in arb_deck_id())| {
                let mut guard = EnvGuard::new();
                guard.set("DECK_ID", &env_deck);

                let config = Config::from_env(Some(cli_deck.clone()), None).unwrap();

                // CLI must always win
                prop_assert_eq!(&config.deck_id, &cli_deck);
            });
        }

        /// Tests that CLI display name always overrides environment variable.
        ///
        /// Invariant: When CLI display name is provided, result.deck_display_name
        /// must equal CLI value regardless of DECK_DISPLAY_NAME environment variable.
        #[test]
        #[serial]
        fn prop_cli_name_always_overrides_env() {
            proptest!(|(cli_name in arb_display_name(), env_name in arb_display_name())| {
                let mut guard = EnvGuard::new();
                guard.set("DECK_DISPLAY_NAME", &env_name);

                let config = Config::from_env(None, Some(cli_name.clone())).unwrap();

                // CLI must always win
                prop_assert_eq!(&config.deck_display_name, &cli_name);
            });
        }

        /// Tests that markdown path always matches deck_id.
        ///
        /// Invariant: config.md_path must equal "./static/{deck_id}/md" for any deck_id.
        #[test]
        #[serial]
        fn prop_md_path_matches_deck_id() {
            proptest!(|(deck_id in arb_deck_id())| {
                let _guard = EnvGuard::new();

                let config = Config::from_env(Some(deck_id.clone()), None).unwrap();

                let expected_path = format!("./static/{}/md", deck_id);
                prop_assert_eq!(&config.md_path, &expected_path);
            });
        }

        /// Tests that image path always matches deck_id.
        ///
        /// Invariant: config.img_path must equal "./static/{deck_id}/img" for any deck_id.
        #[test]
        #[serial]
        fn prop_img_path_matches_deck_id() {
            proptest!(|(deck_id in arb_deck_id())| {
                let _guard = EnvGuard::new();

                let config = Config::from_env(Some(deck_id.clone()), None).unwrap();

                let expected_path = format!("./static/{}/img", deck_id);
                prop_assert_eq!(&config.img_path, &expected_path);
            });
        }

        /// Tests database URL format for local databases.
        ///
        /// Invariant: When DATABASE_URL is not set or ends with .db,
        /// config.database_url must equal "./{deck_id}.db".
        #[test]
        #[serial]
        fn prop_database_url_format_local() {
            proptest!(|(deck_id in arb_deck_id())| {
                let _guard = EnvGuard::new();
                // No DATABASE_URL set, should use local format

                let config = Config::from_env(Some(deck_id.clone()), None).unwrap();

                let expected_url = format!("./{}.db", deck_id);
                prop_assert_eq!(&config.database_url, &expected_url);
            });
        }

        /// Tests that display name defaults to deck_id when not provided.
        ///
        /// Invariant: When both CLI name and env DECK_DISPLAY_NAME are absent,
        /// config.deck_display_name must equal config.deck_id.
        #[test]
        #[serial]
        fn prop_display_name_defaults_to_deck_id() {
            proptest!(|(deck_id in arb_deck_id())| {
                let _guard = EnvGuard::new();

                let config = Config::from_env(Some(deck_id.clone()), None).unwrap();

                // When no name provided, should default to deck_id
                prop_assert_eq!(&config.deck_display_name, &deck_id);
            });
        }
    }
}

