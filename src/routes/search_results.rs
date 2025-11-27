use askama::Template;
use axum::{extract::State, response::{Html, IntoResponse}};
use tower_sessions::Session;

use crate::db::{connection::DbPool, queries::get_random_searched_flashcard};

/// Determines if a flashcard is PNG-only (no question content).
///
/// PNG-only cards have minimal question HTML from image loading.
/// These cards should display answer immediately without hide/reveal logic.
fn is_png_only_card(question_html: &str) -> bool {
    question_html.trim() == "<h3>Question :</h3>"
}

#[derive(Template)]
#[template(path = "search_results.html")]
struct SearchResultsTemplate {
    category: Option<String>,
    subcategory: Option<String>,
    q_html: String,
    a_html: String,
    nb_results: i64,
    is_png_only: bool,
}

/// GET /search_results - Display random card from search results
pub async fn search_results(
    State(pool): State<DbPool>,
    session: Session,
) -> Result<impl IntoResponse, String> {
    let keywords: Vec<String> = session
        .get("keywords")
        .await
        .map_err(|e| format!("Failed to get keywords: {}", e))?
        .unwrap_or_default();

    if keywords.is_empty() {
        return Err("No search keywords found".to_string());
    }

    let mut searched_ids: Vec<i64> = session
        .get("searched_ids")
        .await
        .map_err(|e| format!("Failed to get searched_ids: {}", e))?
        .unwrap_or_default();

    let (card_opt, count) =
        get_random_searched_flashcard(&pool, &searched_ids, &keywords)
            .map_err(|e| format!("Failed to search flashcards: {}", e))?;

    // Reset searched_ids if all results seen
    if searched_ids.len() >= count as usize {
        searched_ids.clear();

        // Retry after reset
        let (card_opt_retry, _) =
            get_random_searched_flashcard(&pool, &searched_ids, &keywords)
                .map_err(|e| format!("Failed to search flashcards: {}", e))?;

        if let Some(card) = card_opt_retry {
            searched_ids.push(card.id);
            session
                .insert("searched_ids", &searched_ids)
                .await
                .map_err(|e| format!("Failed to update searched_ids: {}", e))?;

            let template = SearchResultsTemplate {
                category: card.category.clone(),
                subcategory: card.subcategory.clone(),
                q_html: card.question_html.clone(),
                a_html: card.answer_html,
                nb_results: count,
                is_png_only: is_png_only_card(&card.question_html),
            };
            return Ok(Html(template.render().unwrap()));
        } else {
            return Err("No matching cards found".to_string());
        }
    }

    let card = card_opt.ok_or("No matching cards found")?;

    searched_ids.push(card.id);
    session
        .insert("searched_ids", &searched_ids)
        .await
        .map_err(|e| format!("Failed to update searched_ids: {}", e))?;

    let template = SearchResultsTemplate {
        category: card.category.clone(),
        subcategory: card.subcategory.clone(),
        q_html: card.question_html.clone(),
        a_html: card.answer_html,
        nb_results: count,
        is_png_only: is_png_only_card(&card.question_html),
    };

    Ok(Html(template.render().unwrap()))
}