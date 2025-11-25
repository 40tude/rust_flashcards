use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use crate::db::{connection::DbPool, queries};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    q_html: String,
    a_html: String,
    nb_cards: i64,
}

pub async fn index(State(pool): State<DbPool>) -> Result<impl IntoResponse, String> {
    // Get total count
    let nb_cards = queries::get_total_count(&pool)
        .map_err(|e| format!("Failed to get total count: {}", e))?;

    // Get random flashcard (no exclusions for now, Phase 5 will add session)
    let card = queries::get_random_flashcard(&pool, &[])
        .map_err(|e| format!("Failed to get flashcard: {}", e))?
        .ok_or_else(|| "No flashcards available".to_string())?;

    let template = IndexTemplate {
        q_html: card.question_html,
        a_html: card.answer_html,
        nb_cards,
    };

    let html = template
        .render()
        .map_err(|e| format!("Template render error: {}", e))?;

    Ok(Html(html))
}
