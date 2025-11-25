use anyhow::{Context, Result};
use rusqlite::{params, OptionalExtension};

use super::connection::DbPool;
use super::models::Flashcard;

/// Insert a flashcard into the database
pub fn insert_flashcard(pool: &DbPool, question_html: &str, answer_html: &str) -> Result<i64> {
    let conn = pool.get().context("Failed to get DB connection")?;

    conn.execute(
        "INSERT INTO flashcards (question_html, answer_html) VALUES (?1, ?2)",
        params![question_html, answer_html],
    )
    .context("Failed to insert flashcard")?;

    let id = conn.last_insert_rowid();
    Ok(id)
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
        "INSERT INTO flashcards_fts (id, question_html, answer_html)
         SELECT id, question_html, answer_html FROM flashcards",
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
        "SELECT id, question_html, answer_html FROM flashcards ORDER BY RANDOM() LIMIT 1"
            .to_string()
    } else {
        format!(
            "SELECT id, question_html, answer_html FROM flashcards
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
                    question_html: row.get(1)?,
                    answer_html: row.get(2)?,
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
                "SELECT id, question_html, answer_html FROM flashcards_fts
                 WHERE {} ORDER BY RANDOM() LIMIT 1",
                where_clause
            ),
            rusqlite::params_from_iter(exclude),
            |row| {
                Ok(Flashcard {
                    id: row.get(0)?,
                    question_html: row.get(1)?,
                    answer_html: row.get(2)?,
                })
            },
        )
        .optional()
        .context("Failed to query random searched flashcard")?;

    Ok((card, count))
}