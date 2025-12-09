// Rust guideline compliant 2024-01
use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use tower_sessions::Session;

use crate::db::{models::FilterCriteria, queries};
use crate::routes::AppState;
use crate::session::SessionData;

/// Determines if flashcard is PNG-only (no question content).
///
/// PNG-only cards have minimal question HTML from image loading.
/// These cards display answer immediately without hide/reveal logic.
fn is_png_only_card(question_html: &str) -> bool {
    question_html.trim() == "<h3>Question :</h3>"
}

/// Template for practice session flashcard display.
#[derive(Template)]
#[template(path = "practice.html")]
struct PracticeTemplate {
    deck_name: String,
    category: Option<String>,
    subcategory: Option<String>,
    q_html: String,
    a_html: String,
    nb_cards: i64,
    is_png_only: bool,
}

/// Displays filtered flashcard for practice session.
///
/// Retrieves random flashcard matching current filter criteria from session.
/// Tracks seen cards to avoid repetition. Resets seen list when all filtered
/// cards viewed.
///
/// # Errors
/// Returns error if database query fails. Redirects to landing page if no cards match filters.
pub async fn practice(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, String> {
    let pool = &state.pool;
    let mut session_data: SessionData = session
        .get("data")
        .await
        .map_err(|e| format!("Session get error: {}", e))?
        .unwrap_or_default();

    // Build filter criteria from session
    let criteria = FilterCriteria {
        keywords: session_data.filter_keywords.clone(),
        categories: session_data.filter_categories.clone(),
        subcategories: session_data.filter_subcategories.clone(),
        include_images: session_data.filter_include_images,
    };

    // Get or calculate filtered card count
    let nb_cards = if let Some(cached) = session_data.filtered_card_count {
        cached
    } else {
        let count = queries::count_filtered_flashcards(&pool, &criteria)
            .map_err(|e| format!("Failed to count cards: {}", e))?;
        session_data.filtered_card_count = Some(count);
        count
    };

    if nb_cards == 0 {
        // Store error message in session and redirect to landing page
        session_data.error_message = Some(
            "No cards match your filters. Please adjust your selection.".to_string()
        );
        session
            .insert("data", &session_data)
            .await
            .map_err(|e| format!("Session insert error: {}", e))?;
        return Ok(Redirect::to("/").into_response());
    }

    // Reset seen_ids if all filtered cards seen
    if session_data.seen_ids.len() >= nb_cards as usize {
        session_data.seen_ids.clear();
    }

    // Get random filtered flashcard
    let card = queries::get_filtered_random_flashcard(&pool, &session_data.seen_ids, &criteria)
        .map_err(|e| format!("Failed to get flashcard: {}", e))?
        .ok_or_else(|| "No cards available".to_string())?;

    // Add card to seen list
    session_data.seen_ids.push(card.id);

    // Save session
    session
        .insert("data", &session_data)
        .await
        .map_err(|e| format!("Session insert error: {}", e))?;

    let template = PracticeTemplate {
        deck_name: state.config.deck_display_name.clone(),
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

    Ok(Html(html).into_response())
}
