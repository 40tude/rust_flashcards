// Rust guideline compliant 2024-01
use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    // Form,
};
// use serde::Deserialize;
use tower_sessions::Session;

use crate::db::{models::FilterCriteria, queries};
use crate::routes::AppState;
use crate::session::SessionData;

/// Checks if any filters are active (non-default).
fn has_active_filters(session: &SessionData) -> bool {
    !session.filter_keywords.is_empty() || session.filter_categories.is_some() || session.filter_subcategories.is_some() || !session.filter_include_images
}

/// Category with selection state.
#[derive(Clone)]
struct CategoryItem {
    name: String,
    selected: bool,
}

/// Subcategory with parent category and selection state.
#[derive(Clone)]
struct SubcategoryItem {
    name: String,
    category: String,
    selected: bool,
}

/// Template for landing page filter form.
#[derive(Template)]
#[template(path = "landing.html")]
struct LandingTemplate {
    deck_name: String,
    categories: Vec<CategoryItem>,
    subcategories: Vec<SubcategoryItem>,
    total_count: i64,
    filtered_count: Option<i64>,
    filter_keywords: String,
    all_categories_checked: bool,
    all_subcategories_checked: bool,
    filter_include_images: bool,
    error_message: Option<String>,
}

/// Form data from filter submission.
///
/// Note: This struct is manually populated from raw form data in `apply_filters`
/// to avoid serde_urlencoded deserialization issues with repeated field names.
#[derive(Debug)]
pub struct FilterForm {
    pub keywords: String,
    pub all_categories: Option<String>,
    pub categories: Vec<String>,
    pub all_subcategories: Option<String>,
    pub subcategories: Vec<String>,
    pub all_images: Option<String>,
}

/// Displays landing page with filter form.
///
/// Shows category/subcategory selection, keyword input, and image inclusion toggle.
/// Displays current filter state from session. Shows error message if present.
///
/// # Errors
/// Returns error if database query or session operation fails.
pub async fn landing(State(state): State<AppState>, session: Session) -> Result<impl IntoResponse, String> {
    let pool = &state.pool;
    let mut session_data: SessionData = session.get("data").await.map_err(|e| format!("Session get error: {}", e))?.unwrap_or_default();

    // Get error message from session (if any) and clear it
    let error_message = session_data.error_message.clone();
    session_data.error_message = None;
    session.insert("data", &session_data).await.map_err(|e| format!("Session insert error: {}", e))?;

    // Query available categories
    let all_categories = queries::get_distinct_categories(&pool).map_err(|e| format!("Failed to get categories: {}", e))?;

    // Build category items with selection state
    let all_categories_checked = session_data.filter_categories.is_none();
    let categories: Vec<CategoryItem> = all_categories
        .into_iter()
        .map(|name| CategoryItem {
            selected: session_data.filter_categories.as_ref().map(|cats| cats.contains(&name)).unwrap_or(false),
            name,
        })
        .collect();

    // ALWAYS render ALL subcategories regardless of category filter
    // JavaScript will handle client-side filtering for visibility
    let all_subcategories_list = queries::get_distinct_subcategories(&pool, None).map_err(|e| format!("Failed to get subcategories: {}", e))?;

    // Build subcategory items with selection state and parent category
    let all_subcategories_checked = session_data.filter_subcategories.is_none();
    let subcategories: Vec<SubcategoryItem> = all_subcategories_list
        .into_iter()
        .map(|(name, category)| SubcategoryItem {
            selected: session_data.filter_subcategories.as_ref().map(|subcats| subcats.contains(&name)).unwrap_or(false),
            name,
            category,
        })
        .collect();

    let total_count = queries::get_total_count(&pool).map_err(|e| format!("Failed to get total count: {}", e))?;

    // Count filtered cards if filters active
    let filtered_count = if has_active_filters(&session_data) {
        let criteria = FilterCriteria {
            keywords: session_data.filter_keywords.clone(),
            categories: session_data.filter_categories.clone(),
            subcategories: session_data.filter_subcategories.clone(),
            include_images: session_data.filter_include_images,
        };
        let count = queries::count_filtered_flashcards(&pool, &criteria).map_err(|e| format!("Failed to count filtered cards: {}", e))?;
        Some(count)
    } else {
        None
    };

    let template = LandingTemplate {
        deck_name: state.config.deck_display_name.clone(),
        categories,
        subcategories,
        total_count,
        filtered_count,
        filter_keywords: session_data.filter_keywords.join(" "),
        all_categories_checked,
        all_subcategories_checked,
        filter_include_images: session_data.filter_include_images,
        error_message,
    };

    let html = template.render().map_err(|e| format!("Template render error: {}", e))?;

    Ok(Html(html))
}

/// Processes filter form submission and redirects to practice.
///
/// Parses form data, saves filter state to session, and redirects to /practice.
/// Resets seen cards list for new practice session.
///
/// # Errors
/// Returns error if session operation fails.
pub async fn apply_filters(session: Session, body: String) -> Result<impl IntoResponse, String> {
    // Manual parsing of form data to handle repeated field names (categories[], subcategories[])
    // Standard serde_urlencoded has issues with untagged enums and repeated fields
    let mut form = FilterForm {
        keywords: String::new(),
        all_categories: None,
        categories: Vec::new(),
        all_subcategories: None,
        subcategories: Vec::new(),
        all_images: None,
    };

    // Simple URL decoder: replaces '+' with space (sufficient for our form data)
    let url_decode = |s: &str| s.replace('+', " ");

    // Parse form body: split on '&' for pairs, then '=' for key/value
    // Multiple values with same key are collected into Vec
    for pair in body.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = url_decode(key);
            let value = url_decode(value);

            match key.as_str() {
                "keywords" => form.keywords = value,
                "all_categories" => form.all_categories = Some(value),
                "categories" => form.categories.push(value), // Collect multiple values
                "all_subcategories" => form.all_subcategories = Some(value),
                "subcategories" => form.subcategories.push(value), // Collect multiple values
                "all_images" => form.all_images = Some(value),
                _ => {} // Ignore unknown fields
            }
        }
    }
    let mut session_data: SessionData = session.get("data").await.map_err(|e| format!("Session get error: {}", e))?.unwrap_or_default();

    // Parse keywords
    session_data.filter_keywords = form.keywords.split_whitespace().map(String::from).collect();

    // Parse categories: None means "all categories", Some(vec![]) means "no categories"
    session_data.filter_categories = if form.all_categories.is_some() {
        None // "All categories" checkbox was checked
    } else {
        Some(form.categories) // Specific categories selected (may be empty for images-only)
    };

    // Parse subcategories with server-side validation
    session_data.filter_subcategories = if form.all_subcategories.is_some() {
        None // "All subcategories" checkbox was checked
    } else if form.subcategories.is_empty() {
        // Validation: if specific categories selected (non-empty) but NO subcategories, error
        // Exception: empty categories + images-only mode is allowed
        if let Some(ref cats) = session_data.filter_categories {
            if !cats.is_empty() {
                session_data.error_message = Some("Please select at least one subcategory for the selected categories".to_string());
                session.insert("data", &session_data).await.map_err(|e| format!("Session insert error: {}", e))?;
                return Ok(Redirect::to("/")); // Redirect back to form with error
            } else {
                None // Empty categories (images-only mode), no subcats needed
            }
        } else {
            None // All categories selected, empty subcats means all
        }
    } else {
        Some(form.subcategories) // Specific subcategories selected
    };

    // Parse images
    session_data.filter_include_images = form.all_images.is_some();

    // Reset seen cards for new practice session
    session_data.seen_ids.clear();
    session_data.filtered_card_count = None;

    session.insert("data", &session_data).await.map_err(|e| format!("Session insert error: {}", e))?;

    Ok(Redirect::to("/practice"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========== Tests for has_active_filters ==========

    #[test]
    fn test_has_active_filters_all_defaults() {
        let session = SessionData::default();
        assert!(!has_active_filters(&session));
    }

    #[test]
    fn test_has_active_filters_with_keywords() {
        let mut session = SessionData::default();
        session.filter_keywords = vec!["rust".to_string()];
        assert!(has_active_filters(&session));
    }

    #[test]
    fn test_has_active_filters_with_categories() {
        let mut session = SessionData::default();
        session.filter_categories = Some(vec!["Programming".to_string()]);
        assert!(has_active_filters(&session));
    }

    #[test]
    fn test_has_active_filters_with_subcategories() {
        let mut session = SessionData::default();
        session.filter_subcategories = Some(vec!["Rust".to_string()]);
        assert!(has_active_filters(&session));
    }

    #[test]
    fn test_has_active_filters_images_excluded() {
        let mut session = SessionData::default();
        session.filter_include_images = false;
        assert!(has_active_filters(&session));
    }

    #[test]
    fn test_has_active_filters_empty_categories_list() {
        let mut session = SessionData::default();
        // Empty categories = images-only mode
        session.filter_categories = Some(vec![]);
        assert!(has_active_filters(&session));
    }

    // ========== Tests for URL Decoding ==========

    #[test]
    fn test_url_decode_plus_to_space() {
        let url_decode = |s: &str| s.replace('+', " ");
        assert_eq!(url_decode("machine+learning"), "machine learning");
        assert_eq!(url_decode("foo+bar+baz"), "foo bar baz");
        assert_eq!(url_decode("no-spaces"), "no-spaces");
    }

    // ========== Tests for Form Parsing Logic ==========

    /// Helper to parse form body using the same logic as apply_filters
    fn parse_form_body(body: &str) -> FilterForm {
        let mut form = FilterForm {
            keywords: String::new(),
            all_categories: None,
            categories: Vec::new(),
            all_subcategories: None,
            subcategories: Vec::new(),
            all_images: None,
        };

        let url_decode = |s: &str| s.replace('+', " ");

        for pair in body.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                let key = url_decode(key);
                let value = url_decode(value);

                match key.as_str() {
                    "keywords" => form.keywords = value,
                    "all_categories" => form.all_categories = Some(value),
                    "categories" => form.categories.push(value),
                    "all_subcategories" => form.all_subcategories = Some(value),
                    "subcategories" => form.subcategories.push(value),
                    "all_images" => form.all_images = Some(value),
                    _ => {}
                }
            }
        }

        form
    }

    #[test]
    fn test_parse_form_keywords() {
        let body = "keywords=machine+learning&all_categories=on&all_subcategories=on&all_images=on";
        let form = parse_form_body(body);

        assert_eq!(form.keywords, "machine learning");
        assert_eq!(form.all_categories, Some("on".to_string()));
        assert_eq!(form.all_subcategories, Some("on".to_string()));
        assert_eq!(form.all_images, Some("on".to_string()));
    }

    #[test]
    fn test_parse_form_specific_categories() {
        let body = "categories=Math&categories=Science&all_subcategories=on&all_images=on";
        let form = parse_form_body(body);

        assert_eq!(form.categories, vec!["Math", "Science"]);
        assert_eq!(form.all_categories, None); // Not checked
    }

    #[test]
    fn test_parse_form_specific_subcategories() {
        let body = "all_categories=on&subcategories=Algebra&subcategories=Geometry&all_images=on";
        let form = parse_form_body(body);

        assert_eq!(form.subcategories, vec!["Algebra", "Geometry"]);
        assert_eq!(form.all_subcategories, None); // Not checked
    }

    #[test]
    fn test_parse_form_images_excluded() {
        let body = "keywords=&all_categories=on&all_subcategories=on";
        let form = parse_form_body(body);

        assert_eq!(form.all_images, None); // Checkbox not checked
    }

    #[test]
    fn test_parse_form_empty_categories_images_only() {
        let body = "all_subcategories=on&all_images=on";
        let form = parse_form_body(body);

        // No categories selected (images-only mode)
        assert!(form.categories.is_empty());
        assert_eq!(form.all_categories, None);
    }

    // ========== Tests for Validation Logic ==========

    #[rstest]
    #[case(None, None, true)] // All cats, all subcats → valid
    #[case(None, Some(vec![]), true)] // All cats, empty subcats → valid (means all)
    #[case(Some(vec![]), None, true)] // Empty cats (images-only), all subcats → valid
    #[case(Some(vec![]), Some(vec![]), true)] // Empty cats (images-only), empty subcats → valid
    #[case(Some(vec!["Math".to_string()]), Some(vec!["Algebra".to_string()]), true)] // Specific cats + subcats → valid
    #[case(Some(vec!["Math".to_string()]), None, true)] // Specific cats, all subcats → valid
    #[case(Some(vec!["Math".to_string()]), Some(vec![]), false)] // Specific cats, NO subcats → INVALID
    fn test_validation_logic(
        #[case] categories: Option<Vec<String>>,
        #[case] subcategories: Option<Vec<String>>,
        #[case] should_be_valid: bool,
    ) {
        // Simulate validation logic from apply_filters
        let is_valid = if subcategories.as_ref().map(|s| s.is_empty()).unwrap_or(false) {
            // Empty subcats list
            if let Some(ref cats) = categories {
                if !cats.is_empty() {
                    false // Specific cats + empty subcats = INVALID
                } else {
                    true // Empty cats (images-only) + empty subcats = VALID
                }
            } else {
                true // All cats + empty subcats = VALID
            }
        } else {
            true // Non-empty subcats or None = VALID
        };

        assert_eq!(
            is_valid, should_be_valid,
            "Validation failed for cats={:?}, subcats={:?}",
            categories, subcategories
        );
    }

    #[test]
    fn test_keywords_parsing_whitespace() {
        let keywords_str = "machine learning neural network";
        let keywords: Vec<String> = keywords_str.split_whitespace().map(String::from).collect();

        assert_eq!(keywords, vec!["machine", "learning", "neural", "network"]);
    }

    #[test]
    fn test_keywords_parsing_empty() {
        let keywords_str = "";
        let keywords: Vec<String> = keywords_str.split_whitespace().map(String::from).collect();

        assert!(keywords.is_empty());
    }

    #[test]
    fn test_keywords_parsing_extra_whitespace() {
        let keywords_str = "  rust    async  ";
        let keywords: Vec<String> = keywords_str.split_whitespace().map(String::from).collect();

        assert_eq!(keywords, vec!["rust", "async"]);
    }

    // ========== Tests for Category/Subcategory Items ==========

    #[test]
    fn test_category_item_creation() {
        let item = CategoryItem {
            name: "Math".to_string(),
            selected: true,
        };

        assert_eq!(item.name, "Math");
        assert!(item.selected);
    }

    #[test]
    fn test_subcategory_item_creation() {
        let item = SubcategoryItem {
            name: "Algebra".to_string(),
            category: "Math".to_string(),
            selected: false,
        };

        assert_eq!(item.name, "Algebra");
        assert_eq!(item.category, "Math");
        assert!(!item.selected);
    }

    #[test]
    fn test_category_item_clone() {
        let item = CategoryItem {
            name: "Science".to_string(),
            selected: true,
        };
        let cloned = item.clone();

        assert_eq!(cloned.name, "Science");
        assert_eq!(cloned.selected, item.selected);
    }
}

