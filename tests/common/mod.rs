// Rust guideline compliant 2025-01
//! Common test utilities and fixtures.
//!
//! Provides shared test infrastructure including database setup,
//! connection pool management, and test data builders for flashcard tests.

use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use tempfile::TempDir;

pub mod fixtures;

/// Database connection pool type alias for test convenience
pub type DbPool = Pool<SqliteConnectionManager>;

/// Creates file-based SQLite database pool for integration tests.
///
/// Uses temporary directory to ensure test isolation. Pool configured
/// with max_size=5 matching production configuration.
///
/// # Examples
/// ```no_run
/// let (pool, _temp_dir) = create_test_pool().unwrap();
/// // _temp_dir automatically cleaned up when dropped
/// ```
///
/// # Errors
/// Returns error if temp directory creation or database initialization fails.
pub fn create_test_pool() -> Result<(DbPool, TempDir)> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");

    let manager = SqliteConnectionManager::file(&db_path);
    let pool = Pool::builder().max_size(5).build(manager)?;

    // Initialize schema
    let conn = pool.get()?;
    init_test_schema(&conn)?;

    Ok((pool, temp_dir))
}

/// Initializes test database schema.
///
/// Creates flashcards table and flashcards_fts virtual table
/// matching production schema.
///
/// # Errors
/// Returns error if table creation fails.
pub fn init_test_schema(conn: &Connection) -> Result<()> {
    // Main flashcards table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS flashcards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category TEXT,
            subcategory TEXT,
            question_html TEXT NOT NULL,
            answer_html TEXT NOT NULL
        )",
        [],
    )?;

    // FTS5 virtual table for full-text search
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS flashcards_fts USING fts5(
            id UNINDEXED,
            category,
            subcategory,
            question_html,
            answer_html,
            content='flashcards',
            content_rowid='id'
        )",
        [],
    )?;

    Ok(())
}

/// Test flashcard data builder.
///
/// Simplifies creation of test flashcards with sensible defaults.
///
/// # Examples
/// ```no_run
/// let card = TestCard::new()
///     .category("Math")
///     .subcategory("Algebra")
///     .question("What is 2+2?")
///     .answer("4")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct TestCard {
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub question_html: String,
    pub answer_html: String,
}

impl TestCard {
    /// Creates new test card builder with default question/answer.
    pub fn new() -> Self {
        Self {
            category: None,
            subcategory: None,
            question_html: "<h3>Question:</h3><p>Test question</p>".to_string(),
            answer_html: "<h3>Answer:</h3><p>Test answer</p>".to_string(),
        }
    }

    /// Sets category for test card.
    pub fn category(mut self, cat: &str) -> Self {
        self.category = Some(cat.to_string());
        self
    }

    /// Sets subcategory for test card.
    pub fn subcategory(mut self, subcat: &str) -> Self {
        self.subcategory = Some(subcat.to_string());
        self
    }

    /// Sets question HTML for test card.
    pub fn question(mut self, q: &str) -> Self {
        self.question_html = format!("<h3>Question:</h3><p>{}</p>", q);
        self
    }

    /// Sets answer HTML for test card.
    pub fn answer(mut self, a: &str) -> Self {
        self.answer_html = format!("<h3>Answer:</h3><p>{}</p>", a);
        self
    }

    /// Creates image-only test card (no category/subcategory).
    pub fn image_only(mut self, img_path: &str) -> Self {
        self.category = None;
        self.subcategory = None;
        self.question_html = "<h3>Question:</h3>".to_string();
        self.answer_html = format!(
            r#"<div class="text-center"><img src="{}" class="img-fluid rounded shadow-sm" style="max-height: 70vh; width: auto;"></div>"#,
            img_path
        );
        self
    }

    /// Finalizes builder and returns test card.
    pub fn build(self) -> Self {
        self
    }
}

impl Default for TestCard {
    fn default() -> Self {
        Self::new()
    }
}
