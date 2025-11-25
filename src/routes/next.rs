use axum::{response::Redirect};

pub async fn next() -> Redirect {
    // Simple redirect to home page
    // Phase 5 will add session logic
    Redirect::to("/")
}
