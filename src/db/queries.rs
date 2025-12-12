use anyhow::{Context, Result};
use rusqlite::{params, OptionalExtension};

use super::connection::DbPool;
use super::models::Flashcard;

/// Insert a flashcard into the database
///
/// Note: FTS table sync happens via `populate_fts_table()` after all inserts complete.
pub fn insert_flashcard(
    pool: &DbPool,
    category: Option<&str>,
    subcategory: Option<&str>,
    question_html: &str,
    answer_html: &str,
) -> Result<i64> {
    let conn = pool.get().context("Failed to get DB connection")?;

    conn.execute(
        "INSERT INTO flashcards (category, subcategory, question_html, answer_html) VALUES (?1, ?2, ?3, ?4)",
        params![category, subcategory, question_html, answer_html],
    )
    .context("Failed to insert flashcard")?;

    Ok(conn.last_insert_rowid())
}

/// Clear all flashcards from both tables
pub fn clear_flashcards(pool: &DbPool) -> Result<()> {
    let conn = pool.get().context("Failed to get DB connection")?;

    conn.execute("DELETE FROM flashcards", [])
        .context("Failed to clear flashcards table")?;

    conn.execute("DELETE FROM flashcards_fts", [])
        .context("Failed to clear flashcards_fts table")?;

    tracing::info!("Cleared all flashcards");
    Ok(())
}

/// Populate FTS table from main flashcards table
pub fn populate_fts_table(pool: &DbPool) -> Result<()> {
    let conn = pool.get().context("Failed to get DB connection")?;

    conn.execute(
        "INSERT INTO flashcards_fts (id, category, subcategory, question_html, answer_html)
         SELECT id, category, subcategory, question_html, answer_html FROM flashcards",
        [],
    )
    .context("Failed to populate FTS table")?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM flashcards_fts", [], |row| {
        row.get(0)
    })?;

    tracing::info!("Populated FTS table with {} flashcards", count);
    Ok(())
}

/// Get total count of flashcards
pub fn get_total_count(pool: &DbPool) -> Result<i64> {
    let conn = pool.get().context("Failed to get DB connection")?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))?;

    Ok(count)
}


/// Checks if the database is empty.
///
/// # Errors
/// Returns error if database query fails.
pub fn is_database_empty(pool: &DbPool) -> Result<bool> {
    let count = get_total_count(pool)?;
    Ok(count == 0)
}

/// Retrieves distinct categories from database.
///
/// Returns sorted list of unique non-null categories.
///
/// # Errors
/// Returns error if database query fails.
pub fn get_distinct_categories(pool: &DbPool) -> Result<Vec<String>> {
    let conn = pool.get().context("Failed to get DB connection")?;

    let mut stmt = conn
        .prepare("SELECT DISTINCT category FROM flashcards WHERE category IS NOT NULL ORDER BY category")
        .context("Failed to prepare category query")?;

    let categories = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .context("Failed to query categories")?
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("Failed to collect categories")?;

    Ok(categories)
}

/// Retrieves distinct subcategories with their parent categories.
///
/// Returns tuples of (subcategory_name, parent_category_name) to enable
/// client-side filtering of subcategories based on selected categories.
/// If `categories` is None, returns all subcategories. If Some, returns only
/// subcategories belonging to specified categories.
///
/// # Errors
/// Returns error if database query fails.
pub fn get_distinct_subcategories(
    pool: &DbPool,
    categories: Option<&[String]>,
) -> Result<Vec<(String, String)>> {
    let conn = pool.get().context("Failed to get DB connection")?;

    let (query, params): (String, Vec<String>) = if let Some(cats) = categories {
        let placeholders = cats.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT DISTINCT subcategory, category FROM flashcards WHERE subcategory IS NOT NULL AND category IN ({}) ORDER BY subcategory",
            placeholders
        );
        (query, cats.to_vec())
    } else {
        (
            "SELECT DISTINCT subcategory, category FROM flashcards WHERE subcategory IS NOT NULL ORDER BY subcategory".to_string(),
            Vec::new(),
        )
    };

    let mut stmt = conn
        .prepare(&query)
        .context("Failed to prepare subcategory query")?;

    let subcategories = stmt
        .query_map(rusqlite::params_from_iter(params.iter()), |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .context("Failed to query subcategories")?
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("Failed to collect subcategories")?;

    Ok(subcategories)
}

/// Counts flashcards matching filter criteria.
///
/// # Errors
/// Returns error if database query fails.
pub fn count_filtered_flashcards(
    pool: &DbPool,
    filters: &super::models::FilterCriteria,
) -> Result<i64> {
    let conn = pool.get().context("Failed to get DB connection")?;

    let mut query_parts = vec!["SELECT COUNT(*) FROM flashcards WHERE 1=1".to_string()];
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    // Keywords filter (FTS5 subquery)
    if !filters.keywords.is_empty() {
        let match_query = filters.keywords.join(" AND ");
        query_parts.push("AND id IN (SELECT id FROM flashcards_fts WHERE flashcards_fts MATCH ?)".to_string());
        params.push(Box::new(match_query));
    }

    // Category filter
    if let Some(ref cats) = filters.categories {
        if cats.is_empty() {
            // Empty vec means "no categories" - only match image-only cards
            query_parts.push("AND category IS NULL".to_string());
        } else {
            let placeholders = cats.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query_parts.push(format!("AND category IN ({})", placeholders));
            for cat in cats {
                params.push(Box::new(cat.clone()));
            }
        }
    }

    // Subcategory filter
    if let Some(ref subcats) = filters.subcategories {
        if !subcats.is_empty() {
            let placeholders = subcats.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query_parts.push(format!("AND subcategory IN ({})", placeholders));
            for subcat in subcats {
                params.push(Box::new(subcat.clone()));
            }
        }
    }

    // Image filter
    if !filters.include_images {
        query_parts.push("AND question_html != '<h3>Question:</h3>'".to_string());
    }

    let query = query_parts.join(" ");

    let count: i64 = conn
        .query_row(
            &query,
            rusqlite::params_from_iter(params.iter().map(|p| p.as_ref())),
            |row| row.get(0),
        )
        .context("Failed to count filtered flashcards")?;

    Ok(count)
}

/// Retrieves random flashcard matching filter criteria, excluding specified IDs.
///
/// Combines all filter criteria (keywords, categories, subcategories, images) with
/// AND logic. Returns None if no matching cards found.
///
/// # Errors
/// Returns error if database query fails.
pub fn get_filtered_random_flashcard(
    pool: &DbPool,
    exclude: &[i64],
    filters: &super::models::FilterCriteria,
) -> Result<Option<Flashcard>> {
    let conn = pool.get().context("Failed to get DB connection")?;

    let mut query_parts = vec!["SELECT id, category, subcategory, question_html, answer_html FROM flashcards WHERE 1=1".to_string()];
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    // Keywords filter (FTS5 subquery)
    if !filters.keywords.is_empty() {
        let match_query = filters.keywords.join(" AND ");
        query_parts.push("AND id IN (SELECT id FROM flashcards_fts WHERE flashcards_fts MATCH ?)".to_string());
        params.push(Box::new(match_query));
    }

    // Category filter
    if let Some(ref cats) = filters.categories {
        if cats.is_empty() {
            // Empty vec means "no categories" - only match image-only cards
            query_parts.push("AND category IS NULL".to_string());
        } else {
            let placeholders = cats.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query_parts.push(format!("AND category IN ({})", placeholders));
            for cat in cats {
                params.push(Box::new(cat.clone()));
            }
        }
    }

    // Subcategory filter
    if let Some(ref subcats) = filters.subcategories {
        if !subcats.is_empty() {
            let placeholders = subcats.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query_parts.push(format!("AND subcategory IN ({})", placeholders));
            for subcat in subcats {
                params.push(Box::new(subcat.clone()));
            }
        }
    }

    // Image filter
    if !filters.include_images {
        query_parts.push("AND question_html != '<h3>Question:</h3>'".to_string());
    }

    // Exclude seen cards
    if !exclude.is_empty() {
        let placeholders = exclude.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        query_parts.push(format!("AND id NOT IN ({})", placeholders));
        for id in exclude {
            params.push(Box::new(*id));
        }
    }

    query_parts.push("ORDER BY RANDOM() LIMIT 1".to_string());

    let query = query_parts.join(" ");

    let card = conn
        .query_row(
            &query,
            rusqlite::params_from_iter(params.iter().map(|p| p.as_ref())),
            |row| {
                Ok(Flashcard {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    subcategory: row.get(2)?,
                    question_html: row.get(3)?,
                    answer_html: row.get(4)?,
                })
            },
        )
        .optional()
        .context("Failed to query filtered flashcard")?;

    Ok(card)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::FilterCriteria;
    use rstest::rstest;

    /// Creates in-memory database with schema for testing.
    fn setup_test_db() -> DbPool {
        let manager = r2d2_sqlite::SqliteConnectionManager::memory();
        let pool = r2d2::Pool::builder().max_size(1).build(manager).unwrap();

        // Initialize schema
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

        conn.execute(
            "CREATE VIRTUAL TABLE flashcards_fts USING fts5(
                id UNINDEXED,
                category,
                subcategory,
                question_html,
                answer_html
            )",
            [],
        )
        .unwrap();

        pool
    }

    /// Inserts sample flashcards for filter testing.
    ///
    /// Returns pool with 10 cards: 3 Math (2 Algebra, 1 Geometry),
    /// 3 Science (2 Physics, 1 Chemistry), 2 Programming, 2 image-only.
    fn setup_test_data() -> DbPool {
        let pool = setup_test_db();

        // Math - Algebra (2 cards)
        insert_flashcard(
            &pool,
            Some("Math"),
            Some("Algebra"),
            "<h3>Question:</h3><p>What is 2+2?</p>",
            "<h3>Answer:</h3><p>4</p>",
        )
        .unwrap();

        insert_flashcard(
            &pool,
            Some("Math"),
            Some("Algebra"),
            "<h3>Question:</h3><p>What is 3+3?</p>",
            "<h3>Answer:</h3><p>6</p>",
        )
        .unwrap();

        // Math - Geometry (1 card)
        insert_flashcard(
            &pool,
            Some("Math"),
            Some("Geometry"),
            "<h3>Question:</h3><p>What is pi?</p>",
            "<h3>Answer:</h3><p>Approximately 3.14159</p>",
        )
        .unwrap();

        // Science - Physics (2 cards, one with "gravity" keyword)
        insert_flashcard(
            &pool,
            Some("Science"),
            Some("Physics"),
            "<h3>Question:</h3><p>What is gravity?</p>",
            "<h3>Answer:</h3><p>Force of attraction</p>",
        )
        .unwrap();

        insert_flashcard(
            &pool,
            Some("Science"),
            Some("Physics"),
            "<h3>Question:</h3><p>Speed of light?</p>",
            "<h3>Answer:</h3><p>299,792,458 m/s</p>",
        )
        .unwrap();

        // Science - Chemistry (1 card with "formula" keyword)
        insert_flashcard(
            &pool,
            Some("Science"),
            Some("Chemistry"),
            "<h3>Question:</h3><p>Water formula?</p>",
            "<h3>Answer:</h3><p>H2O</p>",
        )
        .unwrap();

        // Programming - Rust (1 card)
        insert_flashcard(
            &pool,
            Some("Programming"),
            Some("Rust"),
            "<h3>Question:</h3><p>What is ownership?</p>",
            "<h3>Answer:</h3><p>Memory safety guarantee</p>",
        )
        .unwrap();

        // Programming - Python (1 card)
        insert_flashcard(
            &pool,
            Some("Programming"),
            Some("Python"),
            "<h3>Question:</h3><p>List comprehension?</p>",
            "<h3>Answer:</h3><p>Concise way to create lists</p>",
        )
        .unwrap();

        // Image-only cards (2 cards, no category/subcategory)
        insert_flashcard(
            &pool,
            None,
            None,
            "<h3>Question:</h3>",
            "<h3>Answer:</h3><img src='/static/deck/img/diagram.png'>",
        )
        .unwrap();

        insert_flashcard(
            &pool,
            None,
            None,
            "<h3>Question:</h3>",
            "<h3>Answer:</h3><img src='/static/deck/img/chart.webp'>",
        )
        .unwrap();

        // Populate FTS table for keyword searches
        populate_fts_table(&pool).unwrap();

        pool
    }

    // ========== Basic Function Tests ==========

    #[test]
    fn test_insert_flashcard_returns_incrementing_id() {
        let pool = setup_test_db();

        let id1 = insert_flashcard(
            &pool,
            Some("Category1"),
            Some("Subcat1"),
            "<p>Q1</p>",
            "<p>A1</p>",
        )
        .unwrap();

        let id2 = insert_flashcard(
            &pool,
            Some("Category2"),
            Some("Subcat2"),
            "<p>Q2</p>",
            "<p>A2</p>",
        )
        .unwrap();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_insert_flashcard_with_null_category() {
        let pool = setup_test_db();

        let id = insert_flashcard(&pool, None, None, "<p>Question</p>", "<p>Answer</p>").unwrap();

        assert_eq!(id, 1);

        let conn = pool.get().unwrap();
        let (cat, subcat): (Option<String>, Option<String>) = conn
            .query_row(
                "SELECT category, subcategory FROM flashcards WHERE id = ?",
                [id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();

        assert!(cat.is_none());
        assert!(subcat.is_none());
    }

    #[test]
    fn test_clear_flashcards_removes_from_both_tables() {
        let pool = setup_test_db();

        insert_flashcard(&pool, Some("Cat"), Some("Sub"), "<p>Q</p>", "<p>A</p>").unwrap();
        populate_fts_table(&pool).unwrap();

        // Verify data exists
        assert_eq!(get_total_count(&pool).unwrap(), 1);

        clear_flashcards(&pool).unwrap();

        // Verify both tables cleared
        assert_eq!(get_total_count(&pool).unwrap(), 0);

        let conn = pool.get().unwrap();
        let fts_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards_fts", [], |row| row.get(0))
            .unwrap();
        assert_eq!(fts_count, 0);
    }

    #[test]
    fn test_populate_fts_table_syncs_with_main_table() {
        let pool = setup_test_db();

        insert_flashcard(&pool, Some("Cat1"), Some("Sub1"), "<p>Q1</p>", "<p>A1</p>").unwrap();
        insert_flashcard(&pool, Some("Cat2"), Some("Sub2"), "<p>Q2</p>", "<p>A2</p>").unwrap();

        populate_fts_table(&pool).unwrap();

        let conn = pool.get().unwrap();
        let fts_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards_fts", [], |row| row.get(0))
            .unwrap();

        assert_eq!(fts_count, 2);
    }

    #[test]
    fn test_get_total_count_empty_database() {
        let pool = setup_test_db();
        assert_eq!(get_total_count(&pool).unwrap(), 0);
    }

    #[test]
    fn test_get_total_count_with_cards() {
        let pool = setup_test_data();
        assert_eq!(get_total_count(&pool).unwrap(), 10);
    }

    #[test]
    fn test_is_database_empty_returns_true_when_empty() {
        let pool = setup_test_db();
        assert!(is_database_empty(&pool).unwrap());
    }

    #[test]
    fn test_is_database_empty_returns_false_when_not_empty() {
        let pool = setup_test_data();
        assert!(!is_database_empty(&pool).unwrap());
    }

    #[test]
    fn test_get_distinct_categories_sorted() {
        let pool = setup_test_data();

        let categories = get_distinct_categories(&pool).unwrap();

        assert_eq!(categories.len(), 3);
        assert_eq!(categories[0], "Math");
        assert_eq!(categories[1], "Programming");
        assert_eq!(categories[2], "Science");
    }

    #[test]
    fn test_get_distinct_categories_filters_null() {
        let pool = setup_test_data();

        let categories = get_distinct_categories(&pool).unwrap();

        // Should not include NULL categories (image-only cards)
        assert_eq!(categories.len(), 3);
        assert!(!categories.contains(&"".to_string()));
    }

    #[test]
    fn test_get_distinct_subcategories_all() {
        let pool = setup_test_data();

        let subcats = get_distinct_subcategories(&pool, None).unwrap();

        // 6 subcategories: Algebra, Geometry, Physics, Chemistry, Rust, Python
        assert_eq!(subcats.len(), 6);
        // Should be sorted by subcategory name
        assert_eq!(subcats[0].0, "Algebra");
        assert_eq!(subcats[0].1, "Math");
    }

    #[test]
    fn test_get_distinct_subcategories_filtered_by_category() {
        let pool = setup_test_data();

        let math_filter = vec!["Math".to_string()];
        let subcats = get_distinct_subcategories(&pool, Some(&math_filter)).unwrap();

        assert_eq!(subcats.len(), 2);
        assert!(subcats.iter().any(|(s, c)| s == "Algebra" && c == "Math"));
        assert!(subcats.iter().any(|(s, c)| s == "Geometry" && c == "Math"));
    }

    // ========== Parametrized Tests for count_filtered_flashcards ==========

    #[rstest]
    #[case(vec![], None, None, true, 10)] // No filters - all cards
    #[case(vec![], None, None, false, 8)] // Exclude images (10 - 2 images)
    #[case(vec![], Some(vec!["Math".to_string()]), None, true, 3)] // Category: Math
    #[case(vec![], Some(vec!["Science".to_string()]), None, true, 3)] // Category: Science
    #[case(vec![], Some(vec![]), None, true, 2)] // Empty categories = images only
    #[case(vec![], Some(vec!["Math".to_string()]), Some(vec!["Algebra".to_string()]), true, 2)] // Math - Algebra
    #[case(vec![], Some(vec!["Science".to_string()]), Some(vec!["Physics".to_string()]), true, 2)] // Science - Physics
    #[case(vec![], Some(vec!["Math".to_string(), "Science".to_string()]), None, true, 6)] // Multiple categories
    #[case(vec!["gravity".to_string()], None, None, true, 1)] // Keyword: gravity
    #[case(vec!["formula".to_string()], Some(vec!["Science".to_string()]), None, true, 1)] // Keyword + Category
    fn test_count_filtered_flashcards(
        #[case] keywords: Vec<String>,
        #[case] categories: Option<Vec<String>>,
        #[case] subcategories: Option<Vec<String>>,
        #[case] include_images: bool,
        #[case] expected_count: i64,
    ) {
        let pool = setup_test_data();

        let filters = FilterCriteria {
            keywords,
            categories,
            subcategories,
            include_images,
        };

        let count = count_filtered_flashcards(&pool, &filters).unwrap();
        assert_eq!(count, expected_count);
    }

    // ========== Parametrized Tests for get_filtered_random_flashcard ==========

    #[rstest]
    #[case(vec![], None, None, true, true)] // No filters - should return card
    #[case(vec![], None, None, false, true)] // Exclude images - should return card
    #[case(vec![], Some(vec!["Math".to_string()]), None, true, true)] // Category: Math
    #[case(vec![], Some(vec![]), None, true, true)] // Empty categories = images only
    #[case(vec!["gravity".to_string()], None, None, true, true)] // Keyword: gravity
    #[case(vec!["nonexistent".to_string()], None, None, true, false)] // No match - should return None
    fn test_get_filtered_random_flashcard(
        #[case] keywords: Vec<String>,
        #[case] categories: Option<Vec<String>>,
        #[case] subcategories: Option<Vec<String>>,
        #[case] include_images: bool,
        #[case] should_return_card: bool,
    ) {
        let pool = setup_test_data();

        let filters = FilterCriteria {
            keywords,
            categories,
            subcategories,
            include_images,
        };

        let card = get_filtered_random_flashcard(&pool, &[], &filters).unwrap();

        if should_return_card {
            assert!(card.is_some(), "Expected to find matching card");
        } else {
            assert!(card.is_none(), "Expected no matching card");
        }
    }

    #[test]
    fn test_get_filtered_random_flashcard_respects_exclude_list() {
        let pool = setup_test_data();

        let filters = FilterCriteria {
            keywords: vec![],
            categories: Some(vec!["Math".to_string()]),
            subcategories: Some(vec!["Algebra".to_string()]),
            include_images: true,
        };

        // Math - Algebra has 2 cards (IDs 1 and 2)
        // Get first card
        let card1 = get_filtered_random_flashcard(&pool, &[], &filters)
            .unwrap()
            .unwrap();

        // Exclude first card, should get the other one
        let card2 = get_filtered_random_flashcard(&pool, &[card1.id], &filters)
            .unwrap()
            .unwrap();

        assert_ne!(card1.id, card2.id);

        // Exclude both, should return None
        let card3 = get_filtered_random_flashcard(&pool, &[card1.id, card2.id], &filters).unwrap();

        assert!(card3.is_none());
    }

    #[test]
    fn test_get_filtered_random_flashcard_matches_category() {
        let pool = setup_test_data();

        let filters = FilterCriteria {
            keywords: vec![],
            categories: Some(vec!["Math".to_string()]),
            subcategories: None,
            include_images: true,
        };

        let card = get_filtered_random_flashcard(&pool, &[], &filters)
            .unwrap()
            .unwrap();

        assert_eq!(card.category, Some("Math".to_string()));
    }

    #[test]
    fn test_get_filtered_random_flashcard_matches_subcategory() {
        let pool = setup_test_data();

        let filters = FilterCriteria {
            keywords: vec![],
            categories: Some(vec!["Science".to_string()]),
            subcategories: Some(vec!["Chemistry".to_string()]),
            include_images: true,
        };

        let card = get_filtered_random_flashcard(&pool, &[], &filters)
            .unwrap()
            .unwrap();

        assert_eq!(card.category, Some("Science".to_string()));
        assert_eq!(card.subcategory, Some("Chemistry".to_string()));
    }

    #[test]
    fn test_get_filtered_random_flashcard_image_only_filter() {
        let pool = setup_test_data();

        let filters = FilterCriteria {
            keywords: vec![],
            categories: Some(vec![]), // Empty = images only
            subcategories: None,
            include_images: true,
        };

        let card = get_filtered_random_flashcard(&pool, &[], &filters)
            .unwrap()
            .unwrap();

        // Should be image-only card (no category)
        assert!(card.category.is_none());
        assert_eq!(card.question_html, "<h3>Question:</h3>");
    }

    // ========== Property-Based Tests ==========

    /// Property-based tests using proptest for query invariants.
    ///
    /// Verifies that database queries maintain critical properties across
    /// randomly generated filter criteria.
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        /// Generates random keyword lists.
        fn arb_keywords() -> impl Strategy<Value = Vec<String>> {
            prop::collection::vec("[a-z]{3,10}", 0..5)
        }

        /// Generates random exclude ID lists.
        fn arb_exclude_ids() -> impl Strategy<Value = Vec<i64>> {
            prop::collection::vec(1i64..100, 0..10)
        }

        /// Tests that filtered count never exceeds total count.
        ///
        /// Invariant: For any filter criteria, count_filtered_flashcards(filters)
        /// must be ≤ get_total_count().
        #[test]
        fn prop_filtered_count_le_total() {
            proptest!(|(keywords in arb_keywords(), include_images: bool)| {
                let pool = setup_test_data();

                let filters = FilterCriteria {
                    keywords,
                    categories: None,
                    subcategories: None,
                    include_images,
                };

                let filtered_count = count_filtered_flashcards(&pool, &filters).unwrap();
                let total_count = get_total_count(&pool).unwrap();

                // Filtered count must never exceed total
                prop_assert!(filtered_count <= total_count);
            });
        }

        /// Tests that random flashcard respects exclude list.
        ///
        /// Invariant: If get_filtered_random_flashcard returns a card,
        /// its ID must NOT be in the exclude_ids list.
        #[test]
        fn prop_random_card_respects_exclude_list() {
            proptest!(|(exclude_ids in arb_exclude_ids())| {
                let pool = setup_test_data();

                let filters = FilterCriteria {
                    keywords: vec![],
                    categories: None,
                    subcategories: None,
                    include_images: true,
                };

                if let Some(card) = get_filtered_random_flashcard(&pool, &exclude_ids, &filters).unwrap() {
                    // Returned card ID must not be in exclude list
                    prop_assert!(!exclude_ids.contains(&card.id));
                }
            });
        }

        /// Tests that category filter actually filters.
        ///
        /// Invariant: When categories filter is Some(specific list),
        /// any returned card must have category in that list (or None if empty).
        #[test]
        fn prop_category_filter_works() {
            proptest!(|(include_images: bool)| {
                let pool = setup_test_data();

                // Filter for Math category only
                let filters = FilterCriteria {
                    keywords: vec![],
                    categories: Some(vec!["Math".to_string()]),
                    subcategories: None,
                    include_images,
                };

                if let Some(card) = get_filtered_random_flashcard(&pool, &[], &filters).unwrap() {
                    // Card must either be Math or have no category (if images included)
                    if include_images {
                        prop_assert!(
                            card.category.as_deref() == Some("Math") || card.category.is_none()
                        );
                    } else {
                        prop_assert_eq!(card.category.as_deref(), Some("Math"));
                    }
                }
            });
        }

        /// Tests that empty filter matches at least as many as specific filter.
        ///
        /// Invariant: count(no_filter) ≥ count(with_filter) for any filter.
        #[test]
        fn prop_no_filter_ge_filtered() {
            proptest!(|(keywords in arb_keywords())| {
                let pool = setup_test_data();

                let no_filter = FilterCriteria {
                    keywords: vec![],
                    categories: None,
                    subcategories: None,
                    include_images: true,
                };

                let with_filter = FilterCriteria {
                    keywords,
                    categories: Some(vec!["Math".to_string()]),
                    subcategories: None,
                    include_images: true,
                };

                let count_no_filter = count_filtered_flashcards(&pool, &no_filter).unwrap();
                let count_with_filter = count_filtered_flashcards(&pool, &with_filter).unwrap();

                // No filter should match at least as many cards as with filter
                prop_assert!(count_no_filter >= count_with_filter);
            });
        }

        /// Tests that exclude list reduces available cards.
        ///
        /// Invariant: Excluding N existing IDs should reduce available cards
        /// (unless all cards are excluded).
        #[test]
        fn prop_exclude_list_reduces_available() {
            proptest!(|(n_exclude in 1usize..5)| {
                let pool = setup_test_data();

                let filters = FilterCriteria {
                    keywords: vec![],
                    categories: None,
                    subcategories: None,
                    include_images: true,
                };

                // Get total count
                let total = count_filtered_flashcards(&pool, &filters).unwrap();

                // Exclude first n_exclude IDs
                let exclude_ids: Vec<i64> = (1..=n_exclude as i64).collect();

                // Try to get a card with exclusions
                let card = get_filtered_random_flashcard(&pool, &exclude_ids, &filters).unwrap();

                // If we got a card, there must be more cards than excluded
                if card.is_some() {
                    prop_assert!(total > n_exclude as i64);
                }
            });
        }
    }
}
