use super::connection::DbPool;

pub fn init_database(pool: &DbPool) -> anyhow::Result<()> {
    let conn = pool.get()?;

    // Create main flashcards table
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

    tracing::info!("Created flashcards table");

    // Create FTS5 virtual table for full-text search
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS flashcards_fts
         USING fts5(id UNINDEXED, category, subcategory, question_html, answer_html)",
        [],
    )?;

    tracing::info!("Created flashcards_fts virtual table");

    Ok(())
}

#[allow(dead_code)]
pub fn populate_fts_table(pool: &DbPool) -> anyhow::Result<()> {
    let conn = pool.get()?;

    // Clear existing FTS data
    conn.execute("DELETE FROM flashcards_fts", [])?;

    // Copy data from flashcards to flashcards_fts
    conn.execute(
        "INSERT INTO flashcards_fts(id, category, subcategory, question_html, answer_html)
         SELECT id, category, subcategory, question_html, answer_html FROM flashcards",
        [],
    )?;

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM flashcards_fts",
        [],
        |row| row.get(0),
    )?;

    tracing::info!("Populated FTS table with {} entries", count);

    Ok(())
}