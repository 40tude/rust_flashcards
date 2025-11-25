use askama::Template;
use axum::{
    extract::Form,
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate {}

#[derive(Deserialize)]
pub struct SearchForm {
    pub keywords: String,
}

/// GET /search - Display search form
pub async fn search_form() -> impl IntoResponse {
    let template = SearchTemplate {};
    Html(template.render().unwrap())
}

/// POST /search - Submit search keywords
pub async fn search_submit(
    session: Session,
    Form(form): Form<SearchForm>,
) -> Result<Redirect, String> {
    let keywords: Vec<String> = form
        .keywords
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    session
        .insert("keywords", &keywords)
        .await
        .map_err(|e| format!("Failed to insert keywords: {}", e))?;

    session
        .insert("searched_ids", &Vec::<i64>::new())
        .await
        .map_err(|e| format!("Failed to insert searched_ids: {}", e))?;

    Ok(Redirect::to("/search_results"))
}