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

/// Get a random flashcard, excluding specified IDs
pub fn get_random_flashcard(pool: &DbPool, exclude: &[i64]) -> Result<Option<Flashcard>> {
    let conn = pool.get().context("Failed to get DB connection")?;

    let query = if exclude.is_empty() {
        "SELECT id, category, subcategory, question_html, answer_html FROM flashcards ORDER BY RANDOM() LIMIT 1"
            .to_string()
    } else {
        format!(
            "SELECT id, category, subcategory, question_html, answer_html FROM flashcards
             WHERE id NOT IN ({}) ORDER BY RANDOM() LIMIT 1",
            exclude
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(",")
        )
    };

    let card = conn
        .query_row(
            &query,
            rusqlite::params_from_iter(exclude),
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
        .context("Failed to query random flashcard")?;

    Ok(card)
}

/// Get a random flashcard from search results, excluding specified IDs
pub fn get_random_searched_flashcard(
    pool: &DbPool,
    exclude: &[i64],
    keywords: &[String],
) -> Result<(Option<Flashcard>, i64)> {
    let conn = pool.get().context("Failed to get DB connection")?;

    // Build FTS5 query: "keyword1 AND keyword2"
    let match_query = keywords.join(" AND ");

    let where_clause = if exclude.is_empty() {
        format!("flashcards_fts MATCH '{}'", match_query)
    } else {
        format!(
            "flashcards_fts MATCH '{}' AND id NOT IN ({})",
            match_query,
            exclude
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(",")
        )
    };

    // Count matching results
    let count: i64 = conn
        .query_row(
            &format!("SELECT COUNT(*) FROM flashcards_fts WHERE {}", where_clause),
            rusqlite::params_from_iter(exclude),
            |row| row.get(0),
        )
        .context("Failed to count search results")?;

    // Get random matching card
    let card = conn
        .query_row(
            &format!(
                "SELECT id, category, subcategory, question_html, answer_html FROM flashcards_fts
                 WHERE {} ORDER BY RANDOM() LIMIT 1",
                where_clause
            ),
            rusqlite::params_from_iter(exclude),
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
        .context("Failed to query random searched flashcard")?;

    Ok((card, count))
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

/// Retrieves distinct subcategories, optionally filtered by categories.
///
/// If `categories` is None, returns all subcategories. If Some, returns only
/// subcategories belonging to specified categories.
///
/// # Errors
/// Returns error if database query fails.
pub fn get_distinct_subcategories(
    pool: &DbPool,
    categories: Option<&[String]>,
) -> Result<Vec<String>> {
    let conn = pool.get().context("Failed to get DB connection")?;

    let (query, params): (String, Vec<String>) = if let Some(cats) = categories {
        let placeholders = cats.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT DISTINCT subcategory FROM flashcards WHERE subcategory IS NOT NULL AND category IN ({}) ORDER BY subcategory",
            placeholders
        );
        (query, cats.to_vec())
    } else {
        (
            "SELECT DISTINCT subcategory FROM flashcards WHERE subcategory IS NOT NULL ORDER BY subcategory".to_string(),
            Vec::new(),
        )
    };

    let mut stmt = conn
        .prepare(&query)
        .context("Failed to prepare subcategory query")?;

    let subcategories = stmt
        .query_map(rusqlite::params_from_iter(params.iter()), |row| {
            row.get::<_, String>(0)
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
        if !cats.is_empty() {
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
        query_parts.push("AND question_html != '<h3>Question :</h3>'".to_string());
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
        if !cats.is_empty() {
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
        query_parts.push("AND question_html != '<h3>Question :</h3>'".to_string());
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