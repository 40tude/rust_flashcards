use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use tower_sessions::Session;

use crate::db::{connection::DbPool, queries};
use crate::session::SessionData;

/// Determines if a flashcard is PNG-only (no question content).
///
/// PNG-only cards have minimal question HTML from image loading.
/// These cards should display answer immediately without hide/reveal logic.
fn is_png_only_card(question_html: &str) -> bool {
    question_html.trim() == "<h3>Question :</h3>"
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    category: Option<String>,
    subcategory: Option<String>,
    q_html: String,
    a_html: String,
    nb_cards: i64,
    is_png_only: bool,
}

pub async fn index(
    State(pool): State<DbPool>,
    session: Session,
) -> Result<impl IntoResponse, String> {
    // Get or initialize session data
    let mut session_data: SessionData = session
        .get("data")
        .await
        .map_err(|e| format!("Session get error: {}", e))?
        .unwrap_or_default();

    // Get total count (cached in session or from DB)
    let nb_cards = if let Some(cached) = session_data.nb_cards {
        cached
    } else {
        let count = queries::get_total_count(&pool)
            .map_err(|e| format!("Failed to get total count: {}", e))?;
        session_data.nb_cards = Some(count);
        count
    };

    // Reset seen_ids if all cards have been seen
    if session_data.seen_ids.len() >= nb_cards as usize {
        session_data.seen_ids.clear();
    }

    // Get random flashcard excluding already seen
    let card = queries::get_random_flashcard(&pool, &session_data.seen_ids)
        .map_err(|e| format!("Failed to get flashcard: {}", e))?
        .ok_or_else(|| "No flashcards available".to_string())?;

    // Add card to seen list
    session_data.seen_ids.push(card.id);

    // Save session
    session
        .insert("data", &session_data)
        .await
        .map_err(|e| format!("Session insert error: {}", e))?;

    let template = IndexTemplate {
        category: card.category.clone(),
        subcategory: card.subcategory.clone(),
        q_html: card.question_html.clone(),
        a_html: card.answer_html,
        nb_cards,
        is_png_only: is_png_only_card(&card.question_html),
    };

    let html = template
        .render()
        .map_err(|e| format!("Template render error: {}", e))?;

    Ok(Html(html))
}
