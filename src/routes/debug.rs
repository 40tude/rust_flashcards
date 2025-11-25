use axum::response::{Html, IntoResponse};
use tower_sessions::Session;

pub async fn reset_session(session: Session) -> Result<impl IntoResponse, String> {
    // Clear session
    session
        .flush()
        .await
        .map_err(|e| format!("Failed to clear session: {}", e))?;

    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css" rel="stylesheet">
    <title>Session Reset</title>
</head>
<body>
    <div class="container">
        <h1 class="mt-5">Session Reset</h1>
        <p class="mt-3">Your session has been cleared.</p>
        <a href="/" class="btn btn-primary">Go Home</a>
    </div>
</body>
</html>
    "#;

    Ok(Html(html))
}
