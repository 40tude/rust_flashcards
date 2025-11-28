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

