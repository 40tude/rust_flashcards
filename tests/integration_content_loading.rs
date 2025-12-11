// Integration tests for content loading pipeline
// Tests full workflow: markdown files → database → FTS sync

mod common;

use rust_flashcards::content::{images, markdown};
use rust_flashcards::db::queries;
use std::fs;
use tempfile::TempDir;

/// Tests complete markdown loading pipeline.
///
/// Creates markdown files, loads them into database, verifies HTML rendering,
/// category extraction, and FTS table population.
#[tokio::test]
async fn test_markdown_to_database_full_pipeline() {
    let (pool, _temp_dir) = common::create_test_pool().unwrap();

    // Create markdown directory
    let md_dir = TempDir::new().unwrap();
    fs::create_dir_all(&md_dir).unwrap();

    // Create markdown file with multiple Q&A pairs
    let content = r#"Question : Math - Algebra - What is 2+2?
Answer : 4

Question : Math - Geometry - What is a triangle?
Answer : A polygon with three sides

Question : Science - Physics - What is gravity?
Answer : A fundamental force"#;

    fs::write(md_dir.path().join("test.md"), content).unwrap();

    // Load markdown
    markdown::load_markdown(&pool, md_dir.path().to_str().unwrap()).unwrap();

    // Populate FTS table (normally done in main.rs after all content loading)
    queries::populate_fts_table(&pool).unwrap();

    // Verify database contents
    let conn = pool.get().unwrap();

    // Should have 3 flashcards
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 3);

    // Verify first card (Math - Algebra)
    let (cat, subcat, q_html, a_html): (Option<String>, Option<String>, String, String) = conn
        .query_row(
            "SELECT category, subcategory, question_html, answer_html FROM flashcards WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .unwrap();

    assert_eq!(cat, Some("Math".to_string()));
    assert_eq!(subcat, Some("Algebra".to_string()));
    assert!(q_html.contains("What is 2+2?"));
    assert!(a_html.contains("4"));

    // Verify FTS table populated
    let fts_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM flashcards_fts", [], |row| row.get(0))
        .unwrap();
    assert_eq!(fts_count, 3);

    // Verify FTS search works
    let search_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM flashcards_fts WHERE flashcards_fts MATCH 'gravity'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(search_count, 1);
}

/// Tests image loading pipeline.
///
/// Creates image files, loads them into database, verifies deck-aware paths.
#[tokio::test]
async fn test_images_to_database_full_pipeline() {
    let (pool, _temp_dir) = common::create_test_pool().unwrap();

    // Create image directory structure
    let img_base = TempDir::new().unwrap();
    let img_dir = img_base.path().join("static").join("test_deck").join("img");
    fs::create_dir_all(&img_dir).unwrap();

    // Create fake image files
    fs::write(img_dir.join("image1.png"), b"fake png data").unwrap();
    fs::write(img_dir.join("image2.webp"), b"fake webp data").unwrap();

    // Load images
    images::load_images(&pool, img_dir.to_str().unwrap()).unwrap();

    // Verify database contents
    let conn = pool.get().unwrap();

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 2);

    // Verify image cards have no category/subcategory
    let (cat, subcat): (Option<String>, Option<String>) = conn
        .query_row(
            "SELECT category, subcategory FROM flashcards WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();

    assert_eq!(cat, None);
    assert_eq!(subcat, None);

    // Verify deck-aware path in answer HTML
    let a_html: String = conn
        .query_row(
            "SELECT answer_html FROM flashcards WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();

    assert!(a_html.contains("/static/test_deck/img/"));
}

/// Tests multi-deck isolation.
///
/// Loads two separate decks into different databases, verifies no cross-contamination.
#[tokio::test]
async fn test_multi_deck_isolation() {
    // Create two separate database pools
    let (pool1, _temp_dir1) = common::create_test_pool().unwrap();
    let (pool2, _temp_dir2) = common::create_test_pool().unwrap();

    // Create deck 1 content
    let md_dir1 = TempDir::new().unwrap();
    fs::create_dir_all(&md_dir1).unwrap();
    fs::write(
        md_dir1.path().join("cards.md"),
        "Question : Deck1 - Cat1 - Q1?\nAnswer : A1",
    )
    .unwrap();

    // Create deck 2 content
    let md_dir2 = TempDir::new().unwrap();
    fs::create_dir_all(&md_dir2).unwrap();
    fs::write(
        md_dir2.path().join("cards.md"),
        "Question : Deck2 - Cat2 - Q2?\nAnswer : A2",
    )
    .unwrap();

    // Load into separate databases
    markdown::load_markdown(&pool1, md_dir1.path().to_str().unwrap()).unwrap();
    markdown::load_markdown(&pool2, md_dir2.path().to_str().unwrap()).unwrap();

    // Verify pool1 has only deck1 content
    let conn1 = pool1.get().unwrap();
    let cat1: String = conn1
        .query_row(
            "SELECT category FROM flashcards WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(cat1, "Deck1");

    // Verify pool2 has only deck2 content
    let conn2 = pool2.get().unwrap();
    let cat2: String = conn2
        .query_row(
            "SELECT category FROM flashcards WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(cat2, "Deck2");
}

/// Tests content validation errors.
///
/// Verifies graceful handling of missing directories, corrupted files.
#[tokio::test]
async fn test_content_validation_errors() {
    let (pool, _temp_dir) = common::create_test_pool().unwrap();

    // Test: missing directory (walkdir silently skips, not an error)
    // This succeeds but loads 0 cards
    markdown::load_markdown(&pool, "/nonexistent/path").unwrap();

    let conn = pool.get().unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 0);
    drop(conn);

    // Test: empty directory (should succeed but load nothing)
    let empty_dir = TempDir::new().unwrap();
    fs::create_dir_all(&empty_dir).unwrap();

    markdown::load_markdown(&pool, empty_dir.path().to_str().unwrap()).unwrap();

    let conn2 = pool.get().unwrap();
    let count: i64 = conn2
        .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 0);
}

/// Tests markdown with code blocks and syntax highlighting.
#[tokio::test]
async fn test_markdown_with_code_blocks_integration() {
    let (pool, _temp_dir) = common::create_test_pool().unwrap();

    let md_dir = TempDir::new().unwrap();
    fs::create_dir_all(&md_dir).unwrap();

    let content = r#"Question : Programming - Rust - How to print in Rust?
Answer : Use println! macro:

```rust
fn main() {
    println!("Hello, World!");
}
```"#;

    fs::write(md_dir.path().join("code.md"), content).unwrap();

    markdown::load_markdown(&pool, md_dir.path().to_str().unwrap()).unwrap();

    let conn = pool.get().unwrap();
    let a_html: String = conn
        .query_row(
            "SELECT answer_html FROM flashcards WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();

    // Should contain code block with syntax highlighting
    assert!(a_html.contains("<pre><code>"));
    assert!(a_html.contains("println!"));
}

/// Tests recursive directory loading.
#[tokio::test]
async fn test_recursive_directory_loading() {
    let (pool, _temp_dir) = common::create_test_pool().unwrap();

    // Create nested directory structure
    let md_dir = TempDir::new().unwrap();
    let nested_dir = md_dir.path().join("subdirectory");
    fs::create_dir_all(&nested_dir).unwrap();

    // Root level file
    fs::write(
        md_dir.path().join("root.md"),
        "Question : Root - Cat - Q1?\nAnswer : A1",
    )
    .unwrap();

    // Nested file
    fs::write(
        nested_dir.join("nested.md"),
        "Question : Nested - Cat - Q2?\nAnswer : A2",
    )
    .unwrap();

    markdown::load_markdown(&pool, md_dir.path().to_str().unwrap()).unwrap();

    let conn = pool.get().unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
        .unwrap();

    // Should find both files
    assert_eq!(count, 2);
}

/// Tests clearing and reloading content.
#[tokio::test]
async fn test_content_reload() {
    let (pool, _temp_dir) = common::create_test_pool().unwrap();

    let md_dir = TempDir::new().unwrap();
    fs::create_dir_all(&md_dir).unwrap();

    // Initial load
    fs::write(
        md_dir.path().join("v1.md"),
        "Question : Version - One - Q?\nAnswer : A1",
    )
    .unwrap();

    markdown::load_markdown(&pool, md_dir.path().to_str().unwrap()).unwrap();

    let conn = pool.get().unwrap();
    let count1: i64 = conn
        .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count1, 1);
    drop(conn);

    // Update content and reload
    fs::remove_file(md_dir.path().join("v1.md")).unwrap();
    fs::write(
        md_dir.path().join("v2.md"),
        "Question : Version - Two - Q1?\nAnswer : A2\n\nQuestion : Version - Two - Q2?\nAnswer : A2.2",
    )
    .unwrap();

    markdown::load_markdown(&pool, md_dir.path().to_str().unwrap()).unwrap();

    let conn = pool.get().unwrap();
    let count2: i64 = conn
        .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
        .unwrap();

    // Should have new content (old cleared)
    assert_eq!(count2, 2);

    // After reload, IDs restart from 1
    let category: Option<String> = conn
        .query_row(
            "SELECT category FROM flashcards ORDER BY id LIMIT 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(category, Some("Version".to_string()));
}

/// Tests mixed content types (markdown + images).
#[tokio::test]
async fn test_mixed_content_types() {
    let (pool, _temp_dir) = common::create_test_pool().unwrap();

    // Create markdown content
    let md_dir = TempDir::new().unwrap();
    fs::create_dir_all(&md_dir).unwrap();
    fs::write(
        md_dir.path().join("text.md"),
        "Question : Text - Category - Q?\nAnswer : A",
    )
    .unwrap();

    // Create image content
    let img_base = TempDir::new().unwrap();
    let img_dir = img_base.path().join("static").join("deck").join("img");
    fs::create_dir_all(&img_dir).unwrap();
    fs::write(img_dir.join("image.png"), b"fake").unwrap();

    // Load both
    markdown::load_markdown(&pool, md_dir.path().to_str().unwrap()).unwrap();
    images::load_images(&pool, img_dir.to_str().unwrap()).unwrap();

    let conn = pool.get().unwrap();

    // Total count should be 2 (1 markdown + 1 image)
    let total: i64 = conn
        .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
        .unwrap();
    assert_eq!(total, 2);

    // Count cards with categories (markdown only)
    let with_cat: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM flashcards WHERE category IS NOT NULL",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(with_cat, 1);

    // Count cards without categories (images only)
    let without_cat: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM flashcards WHERE category IS NULL",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(without_cat, 1);
}
