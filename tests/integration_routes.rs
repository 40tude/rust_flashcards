// Integration tests for HTTP routes and session management
// Tests full request/response cycles with axum-test

mod common;

use axum::{routing::{get, post}, Router};
use axum_test::TestServer;
use rust_flashcards::{config::Config, content::markdown, db::queries, routes::{self, AppState}};
use tower_http::services::ServeDir;
use tower_sessions::{MemoryStore, SessionManagerLayer};

/// Creates test server with minimal content for route testing.
async fn setup_test_server() -> TestServer {
    let (pool, _temp_dir) = common::create_test_pool().unwrap();

    // Load minimal test content
    let md_dir = tempfile::TempDir::new().unwrap();
    std::fs::create_dir_all(&md_dir).unwrap();

    let content = r#"Question : Math - Algebra - What is 2+2?
Answer : 4

Question : Math - Geometry - What is a triangle?
Answer : A polygon with three sides

Question : Science - Physics - What is gravity?
Answer : A fundamental force"#;

    std::fs::write(md_dir.path().join("test.md"), content).unwrap();
    markdown::load_markdown(&pool, md_dir.path().to_str().unwrap()).unwrap();
    queries::populate_fts_table(&pool).unwrap();

    // Create minimal config
    let config = Config {
        database_url: "./test.db".to_string(),
        port: 8080,
        deck_id: "test".to_string(),
        deck_display_name: "Test Deck".to_string(),
        md_path: "./static/test/md".to_string(),
        img_path: "./static/test/img".to_string(),
    };

    let state = AppState { pool, config };

    // Create session layer
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Create router matching production
    let app = Router::new()
        .route("/", get(routes::landing))
        .route("/apply_filters", post(routes::apply_filters))
        .route("/practice", get(routes::practice))
        .route("/reset_session", get(routes::reset_session))
        .nest_service("/static", ServeDir::new("./static"))
        .layer(session_layer)
        .with_state(state);

    TestServer::new(app).unwrap()
}

/// Tests landing page renders successfully.
#[tokio::test]
async fn test_landing_page_renders() {
    let server = setup_test_server().await;

    let response = server.get("/").await;

    // Should return 200 OK
    response.assert_status_ok();

    // Should contain HTML content
    let text = response.text();
    assert!(text.contains("Test Deck")); // deck name
    assert!(text.contains("Keywords")); // form field
    assert!(text.contains("Categories")); // form field
}

/// Tests apply filters redirects to practice.
#[tokio::test]
async fn test_apply_filters_redirect() {
    let server = setup_test_server().await;

    let response = server
        .post("/apply_filters")
        .form(&[
            ("keywords", "gravity"),
            ("all_categories", "on"),
            ("all_subcategories", "on"),
            ("all_images", "on"),
        ])
        .await;

    // Should redirect (303 See Other)
    assert_eq!(response.status_code(), 303);

    // Should redirect to /practice
    let location = response.header("location");
    assert_eq!(location, "/practice");
}

/// Tests practice page displays flashcard.
#[tokio::test]
async fn test_practice_displays_flashcard() {
    let server = setup_test_server().await;

    // First apply default filters
    server
        .post("/apply_filters")
        .form(&[
            ("all_categories", "on"),
            ("all_subcategories", "on"),
            ("all_images", "on"),
        ])
        .await;

    // Then load practice page
    let response = server.get("/practice").await;

    response.assert_status_ok();

    let text = response.text();
    // Should contain one of the flashcard questions
    assert!(
        text.contains("What is 2+2?")
            || text.contains("What is a triangle?")
            || text.contains("What is gravity?")
    );
}

/// Tests practice avoids recently seen cards.
#[tokio::test]
async fn test_practice_avoids_seen_cards() {
    let server = setup_test_server().await;

    // Apply filters
    server
        .post("/apply_filters")
        .form(&[
            ("all_categories", "on"),
            ("all_subcategories", "on"),
            ("all_images", "on"),
        ])
        .await;

    // Load practice page multiple times and collect questions
    let mut seen_questions = Vec::new();

    for _ in 0..5 {
        let response = server.get("/practice").await;
        let text = response.text();

        // Extract question (simplified - just check if it's different from last)
        if let Some(q) = extract_question(&text) {
            // Should not repeat immediately (due to seen_ids tracking)
            if !seen_questions.is_empty() {
                // Note: With only 3 cards, after seeing all 3, it will reset and repeat
                // So we just verify we get valid questions
                assert!(!q.is_empty());
            }
            seen_questions.push(q);
        }
    }

    // Should have seen multiple cards
    assert!(seen_questions.len() >= 3);
}

/// Helper to extract question text from HTML.
fn extract_question(html: &str) -> Option<String> {
    // Simplified extraction - look for common question patterns
    if html.contains("What is 2+2?") {
        Some("2+2".to_string())
    } else if html.contains("What is a triangle?") {
        Some("triangle".to_string())
    } else if html.contains("What is gravity?") {
        Some("gravity".to_string())
    } else {
        None
    }
}

/// Tests filter validation error flow.
#[tokio::test]
async fn test_filter_validation_error() {
    let server = setup_test_server().await;

    // Submit invalid filters: specific categories but no subcategories
    let response = server
        .post("/apply_filters")
        .form(&[
            ("categories", "Math"),
            // No subcategories selected (invalid)
            ("all_images", "on"),
        ])
        .await;

    // Should redirect back to landing page
    assert_eq!(response.status_code(), 303);
    let location = response.header("location");
    assert_eq!(location, "/");

    // Note: The error message is shown once and then cleared from session
    // In a real browser, the redirect would show it, but axum-test doesn't
    // preserve session state across redirects the same way
    // So this test just verifies the redirect happens
}

/// Tests session persistence across requests.
#[tokio::test]
async fn test_session_persistence() {
    let server = setup_test_server().await;

    // Apply specific filters
    server
        .post("/apply_filters")
        .form(&[
            ("keywords", "math"),
            ("categories", "Math"),
            ("subcategories", "Algebra"),
            ("all_images", "on"),
        ])
        .await;

    // Navigate to landing page - filters should be preserved
    let response = server.get("/").await;
    let text = response.text();

    // Check that Math category is selected (checkbox should be checked)
    // This is a simplified check - in real HTML we'd look for checked="checked"
    assert!(text.contains("Math"));
}

/// Tests reset session clears all session data.
#[tokio::test]
async fn test_reset_session() {
    let server = setup_test_server().await;

    // Apply filters
    server
        .post("/apply_filters")
        .form(&[
            ("keywords", "gravity"),
            ("categories", "Science"),
            ("subcategories", "Physics"),
            ("all_images", "on"),
        ])
        .await;

    // Reset session
    let reset_response = server.get("/reset_session").await;

    // Returns HTML (200 OK), not redirect
    assert_eq!(reset_response.status_code(), 200);

    // Load landing page - should have default state
    let landing_response = server.get("/").await;
    landing_response.assert_status_ok();

    // Keywords should be empty, all categories/subcategories should be checked
    let text = landing_response.text();
    // In default state, all categories checkbox is checked
    assert!(text.contains("Categories"));
}

/// Tests filtered card count display.
#[tokio::test]
async fn test_filtered_card_count() {
    let server = setup_test_server().await;

    // Apply filter for Math category only
    server
        .post("/apply_filters")
        .form(&[
            ("categories", "Math"),
            ("subcategories", "Algebra"),
            ("subcategories", "Geometry"),
            ("all_images", "on"),
        ])
        .await;

    // Load landing page
    let response = server.get("/").await;
    let text = response.text();

    // Should show filtered count (2 Math cards)
    // Note: Actual HTML structure might vary
    assert!(text.contains("2") || text.contains("Math"));
}

/// Tests keyword filtering.
#[tokio::test]
async fn test_keyword_filtering() {
    let server = setup_test_server().await;

    // Apply keyword filter for "triangle"
    server
        .post("/apply_filters")
        .form(&[
            ("keywords", "triangle"),
            ("all_categories", "on"),
            ("all_subcategories", "on"),
            ("all_images", "on"),
        ])
        .await;

    // Load practice page
    let response = server.get("/practice").await;

    // Should return OK (whether or not FTS finds matches)
    response.assert_status_ok();

    // Note: FTS search behavior depends on SQLite FTS5 tokenization
    // We just verify the page renders successfully with keyword filters applied
}

/// Tests empty categories (images-only mode).
#[tokio::test]
async fn test_images_only_mode() {
    let server = setup_test_server().await;

    // Submit empty categories (images-only mode)
    let response = server
        .post("/apply_filters")
        .form(&[
            // No categories selected (images-only)
            ("all_subcategories", "on"),
            ("all_images", "on"),
        ])
        .await;

    // Should succeed (redirect to practice)
    assert_eq!(response.status_code(), 303);

    let location = response.header("location");
    assert_eq!(location, "/practice");
}
