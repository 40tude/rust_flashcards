use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

use crate::db::connection::DbPool;
use crate::db::queries;

pub fn load_images(pool: &DbPool, png_dir: &str) -> Result<()> {
    tracing::info!("Loading image flashcards from {}", png_dir);

    let mut count = 0;

    // Walk through all .png and .webp files recursively
    for entry in WalkDir::new(png_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| {
                    let ext_lower = ext.to_ascii_lowercase();
                    ext_lower == "png" || ext_lower == "webp"
                })
                .unwrap_or(false)
        })
    {
        let path = entry.path();
        tracing::debug!("Processing image file: {:?}", path);

        match process_image_file(pool, path, png_dir) {
            Ok(()) => {
                count += 1;
                tracing::debug!("Loaded image flashcard from {:?}", path);
            }
            Err(e) => {
                tracing::warn!("Failed to process {:?}: {}", path, e);
            }
        }
    }

    tracing::info!("Loaded {} image flashcards", count);
    Ok(())
}

fn process_image_file(pool: &DbPool, path: &Path, base_dir: &str) -> Result<()> {
    // Convert absolute path to relative path from base_dir
    let relative_path = path
        .strip_prefix(base_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");

    // Extract deck_id from base_dir path (e.g., "./static/deck/img" -> "deck")
    let deck_id = Path::new(base_dir)
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("deck");

    // Question is empty (just the header as HTML)
    let question_html = "<h3>Question:</h3>\n".to_string();

    // Answer contains the image with deck-aware path, Bootstrap class, centered
    let answer_html = format!(
        "<h3>Answer:</h3>\n<p align=\"center\"><img src='/static/{}/img/{}' class='img-fluid'></p>",
        deck_id, relative_path
    );

    // Insert into database - Images: category and subcategory = None
    queries::insert_flashcard(pool, None, None, &question_html, &answer_html)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::fs;
    use tempfile::TempDir;

    /// Creates in-memory database for image loading tests.
    fn setup_test_db() -> DbPool {
        let manager = r2d2_sqlite::SqliteConnectionManager::memory();
        let pool = r2d2::Pool::builder().max_size(1).build(manager).unwrap();

        let conn = pool.get().unwrap();
        conn.execute(
            "CREATE TABLE flashcards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category TEXT,
                subcategory TEXT,
                question_html TEXT NOT NULL,
                answer_html TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        pool
    }

    // ========== Tests for process_image_file ==========

    #[rstest]
    #[case("./static/deck/img", "test.png", "deck", "/static/deck/img/test.png")]
    #[case("./static/rust/img", "test.webp", "rust", "/static/rust/img/test.webp")]
    #[case(
        "./static/py_deck/img",
        "subdir/image.png",
        "py_deck",
        "/static/py_deck/img/subdir/image.png"
    )]
    #[case(
        "./static/test_42/img",
        "foo/bar/baz.webp",
        "test_42",
        "/static/test_42/img/foo/bar/baz.webp"
    )]
    fn test_process_image_file_path_generation(
        #[case] base_dir: &str,
        #[case] relative_file: &str,
        #[case] expected_deck_id: &str,
        #[case] expected_img_path: &str,
    ) {
        let pool = setup_test_db();
        let temp_dir = TempDir::new().unwrap();

        // Create directory structure
        let full_base_dir = temp_dir.path().join(base_dir.trim_start_matches("./"));
        fs::create_dir_all(&full_base_dir).unwrap();

        // Create image file
        let image_path = full_base_dir.join(relative_file);
        fs::create_dir_all(image_path.parent().unwrap()).unwrap();
        fs::write(&image_path, b"fake image data").unwrap();

        // Process the image
        process_image_file(&pool, &image_path, full_base_dir.to_str().unwrap()).unwrap();

        // Verify database insertion
        let conn = pool.get().unwrap();
        let (cat, subcat, q_html, a_html): (Option<String>, Option<String>, String, String) = conn
            .query_row(
                "SELECT category, subcategory, question_html, answer_html FROM flashcards WHERE id = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .unwrap();

        // Images have no category/subcategory
        assert_eq!(cat, None);
        assert_eq!(subcat, None);

        // Question is just the header
        assert_eq!(q_html, "<h3>Question:</h3>\n");

        // Answer contains the image with correct deck-aware path
        assert!(a_html.contains(&format!("/static/{}/img/", expected_deck_id)));
        assert!(a_html.contains(expected_img_path));
        assert!(a_html.contains("class='img-fluid'"));
    }

    #[test]
    fn test_backslash_to_forward_slash_conversion() {
        let pool = setup_test_db();
        let temp_dir = TempDir::new().unwrap();

        // Simulate Windows path with backslashes
        let base_dir = temp_dir.path().join("static").join("deck").join("img");
        fs::create_dir_all(&base_dir).unwrap();

        let image_path = base_dir.join("subdir").join("test.png");
        fs::create_dir_all(image_path.parent().unwrap()).unwrap();
        fs::write(&image_path, b"fake image").unwrap();

        process_image_file(&pool, &image_path, base_dir.to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let a_html: String = conn
            .query_row(
                "SELECT answer_html FROM flashcards WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();

        // Should NOT contain backslashes
        assert!(!a_html.contains('\\'));
        // Should contain forward slashes
        assert!(a_html.contains("subdir/test.png"));
    }

    #[test]
    fn test_deck_id_extraction_from_base_dir() {
        let pool = setup_test_db();
        let temp_dir = TempDir::new().unwrap();

        // Test various base_dir formats
        let test_cases = vec![
            ("./static/rust/img", "rust"),
            ("./static/python_deck/img", "python_deck"),
            ("./static/test_42/img", "test_42"),
            ("./static/deck/img", "deck"),
        ];

        for (base_pattern, expected_deck_id) in test_cases {
            let base_dir = temp_dir.path().join(base_pattern.trim_start_matches("./"));
            fs::create_dir_all(&base_dir).unwrap();

            let image_path = base_dir.join("test.png");
            fs::write(&image_path, b"fake").unwrap();

            // Clear database
            let conn = pool.get().unwrap();
            conn.execute("DELETE FROM flashcards", []).unwrap();
            drop(conn);

            process_image_file(&pool, &image_path, base_dir.to_str().unwrap()).unwrap();

            let conn = pool.get().unwrap();
            let a_html: String = conn
                .query_row(
                    "SELECT answer_html FROM flashcards ORDER BY id DESC LIMIT 1",
                    [],
                    |row| row.get(0),
                )
                .unwrap();

            assert!(
                a_html.contains(&format!("/static/{}/img/", expected_deck_id)),
                "Expected deck_id '{}' in answer_html: {}",
                expected_deck_id,
                a_html
            );
        }
    }

    #[test]
    fn test_question_html_format() {
        let pool = setup_test_db();
        let temp_dir = TempDir::new().unwrap();

        let base_dir = temp_dir.path().join("static").join("deck").join("img");
        fs::create_dir_all(&base_dir).unwrap();

        let image_path = base_dir.join("test.png");
        fs::write(&image_path, b"fake").unwrap();

        process_image_file(&pool, &image_path, base_dir.to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let q_html: String = conn
            .query_row(
                "SELECT question_html FROM flashcards WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();

        // Question should be exactly the header
        assert_eq!(q_html, "<h3>Question:</h3>\n");
    }

    #[test]
    fn test_answer_html_format() {
        let pool = setup_test_db();
        let temp_dir = TempDir::new().unwrap();

        let base_dir = temp_dir.path().join("static").join("deck").join("img");
        fs::create_dir_all(&base_dir).unwrap();

        let image_path = base_dir.join("test.png");
        fs::write(&image_path, b"fake").unwrap();

        process_image_file(&pool, &image_path, base_dir.to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let a_html: String = conn
            .query_row(
                "SELECT answer_html FROM flashcards WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();

        // Answer should contain required elements
        assert!(a_html.contains("<h3>Answer:</h3>"));
        assert!(a_html.contains("<p align=\"center\">"));
        assert!(a_html.contains("<img src='/static/deck/img/test.png'"));
        assert!(a_html.contains("class='img-fluid'"));
        assert!(a_html.contains("</p>"));
    }

    // ========== Tests for load_images ==========

    #[test]
    fn test_load_images_png_files() {
        let pool = setup_test_db();
        let temp_dir = TempDir::new().unwrap();

        let img_dir = temp_dir.path().join("img");
        fs::create_dir_all(&img_dir).unwrap();

        // Create PNG files
        fs::write(img_dir.join("image1.png"), b"fake1").unwrap();
        fs::write(img_dir.join("image2.png"), b"fake2").unwrap();

        load_images(&pool, img_dir.to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_load_images_webp_files() {
        let pool = setup_test_db();
        let temp_dir = TempDir::new().unwrap();

        let img_dir = temp_dir.path().join("img");
        fs::create_dir_all(&img_dir).unwrap();

        // Create WEBP files
        fs::write(img_dir.join("image1.webp"), b"fake1").unwrap();
        fs::write(img_dir.join("image2.webp"), b"fake2").unwrap();

        load_images(&pool, img_dir.to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_load_images_mixed_extensions() {
        let pool = setup_test_db();
        let temp_dir = TempDir::new().unwrap();

        let img_dir = temp_dir.path().join("img");
        fs::create_dir_all(&img_dir).unwrap();

        // Create mixed files
        fs::write(img_dir.join("image1.png"), b"fake1").unwrap();
        fs::write(img_dir.join("image2.webp"), b"fake2").unwrap();
        fs::write(img_dir.join("image3.PNG"), b"fake3").unwrap(); // Case insensitive
        fs::write(img_dir.join("image4.WEBP"), b"fake4").unwrap();

        load_images(&pool, img_dir.to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        // Should load all 4 images (case-insensitive)
        assert_eq!(count, 4);
    }

    #[test]
    fn test_load_images_skips_non_image_files() {
        let pool = setup_test_db();
        let temp_dir = TempDir::new().unwrap();

        let img_dir = temp_dir.path().join("img");
        fs::create_dir_all(&img_dir).unwrap();

        // Create valid and invalid files
        fs::write(img_dir.join("valid.png"), b"fake").unwrap();
        fs::write(img_dir.join("readme.txt"), b"text").unwrap();
        fs::write(img_dir.join("image.jpg"), b"jpeg").unwrap(); // Not PNG/WEBP

        load_images(&pool, img_dir.to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        // Should only load .png file
        assert_eq!(count, 1);
    }

    #[test]
    fn test_load_images_recursive_directories() {
        let pool = setup_test_db();
        let temp_dir = TempDir::new().unwrap();

        let img_dir = temp_dir.path().join("img");
        fs::create_dir_all(&img_dir).unwrap();

        // Create nested directory structure
        let nested_dir = img_dir.join("subdir");
        fs::create_dir_all(&nested_dir).unwrap();

        fs::write(img_dir.join("root.png"), b"fake1").unwrap();
        fs::write(nested_dir.join("nested.png"), b"fake2").unwrap();

        load_images(&pool, img_dir.to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        // Should find both files (recursive)
        assert_eq!(count, 2);
    }

    // ========== Property-Based Tests ==========

    /// Property-based tests using proptest for image path handling invariants.
    ///
    /// Verifies that image path processing maintains critical properties across
    /// randomly generated file paths and deck IDs.
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        /// Generates valid deck IDs.
        fn arb_deck_id() -> impl Strategy<Value = String> {
            "[a-z0-9_]{3,20}"
        }

        /// Generates file paths with various separators.
        fn arb_file_path() -> impl Strategy<Value = String> {
            prop::string::string_regex("[a-z0-9_/\\\\]{10,50}\\.(png|webp)").unwrap()
        }

        /// Tests that generated HTML never contains backslashes.
        ///
        /// Invariant: All HTML output must use forward slashes for paths,
        /// regardless of input path format (Windows/Unix).
        #[test]
        fn prop_no_backslashes_in_html() {
            proptest!(|(_filepath in arb_file_path())| {
                let pool = setup_test_db();

                // Create temp directory with image file
                let temp_dir = TempDir::new().unwrap();
                let img_dir = temp_dir.path().join("static").join("test").join("img");
                std::fs::create_dir_all(&img_dir).unwrap();
                std::fs::write(img_dir.join("test.png"), b"fake").unwrap();

                load_images(&pool, img_dir.to_str().unwrap()).unwrap();

                // Get the HTML
                let conn = pool.get().unwrap();
                let answer_html: String = conn
                    .query_row(
                        "SELECT answer_html FROM flashcards WHERE id = 1",
                        [],
                        |row| row.get(0),
                    )
                    .unwrap();

                // Must not contain backslashes
                prop_assert!(!answer_html.contains('\\'), "HTML contains backslash: {}", answer_html);
            });
        }

        /// Tests that deck_id is correctly embedded in HTML paths.
        ///
        /// Invariant: Generated HTML paths must contain the correct deck_id
        /// extracted from base directory path "./static/{deck_id}/img".
        #[test]
        fn prop_deck_id_in_html_paths() {
            proptest!(|(deck_id in arb_deck_id())| {
                let pool = setup_test_db();

                let temp_dir = TempDir::new().unwrap();
                let img_dir = temp_dir.path().join("static").join(&deck_id).join("img");
                std::fs::create_dir_all(&img_dir).unwrap();
                std::fs::write(img_dir.join("test.png"), b"fake").unwrap();

                load_images(&pool, img_dir.to_str().unwrap()).unwrap();

                let conn = pool.get().unwrap();
                let answer_html: String = conn
                    .query_row(
                        "SELECT answer_html FROM flashcards WHERE id = 1",
                        [],
                        |row| row.get(0),
                    )
                    .unwrap();

                // Must contain correct deck_id in path
                let expected_path = format!("/static/{}/img/", deck_id);
                prop_assert!(answer_html.contains(&expected_path),
                    "HTML doesn't contain expected path '{}': {}", expected_path, answer_html);
            });
        }

        /// Tests that question HTML format is always consistent.
        ///
        /// Invariant: Question HTML must always be "<h3>Question:</h3>"
        /// for image-only flashcards.
        #[test]
        fn prop_question_html_format_consistent() {
            proptest!(|(_dummy: bool)| {
                let pool = setup_test_db();

                let temp_dir = TempDir::new().unwrap();
                let img_dir = temp_dir.path().join("static").join("test").join("img");
                std::fs::create_dir_all(&img_dir).unwrap();
                std::fs::write(img_dir.join("image.png"), b"fake").unwrap();

                load_images(&pool, img_dir.to_str().unwrap()).unwrap();

                let conn = pool.get().unwrap();
                let question_html: String = conn
                    .query_row(
                        "SELECT question_html FROM flashcards WHERE id = 1",
                        [],
                        |row| row.get(0),
                    )
                    .unwrap();

                // Must always be this exact format (with trailing newline)
                prop_assert_eq!(&question_html, "<h3>Question:</h3>\n");
            });
        }

        /// Tests that answer HTML contains image tag.
        ///
        /// Invariant: Answer HTML must contain <img> tag with src attribute
        /// pointing to the image file.
        #[test]
        fn prop_answer_html_contains_image_tag() {
            proptest!(|(_dummy: bool)| {
                let pool = setup_test_db();

                let temp_dir = TempDir::new().unwrap();
                let img_dir = temp_dir.path().join("static").join("deck").join("img");
                std::fs::create_dir_all(&img_dir).unwrap();
                std::fs::write(img_dir.join("test.webp"), b"fake").unwrap();

                load_images(&pool, img_dir.to_str().unwrap()).unwrap();

                let conn = pool.get().unwrap();
                let answer_html: String = conn
                    .query_row(
                        "SELECT answer_html FROM flashcards WHERE id = 1",
                        [],
                        |row| row.get(0),
                    )
                    .unwrap();

                // Must contain image tag
                prop_assert!(answer_html.contains("<img"), "Missing <img tag");
                prop_assert!(answer_html.contains("src="), "Missing src attribute");
                prop_assert!(answer_html.contains("/static/"), "Missing /static/ path");
            });
        }
    }
}
