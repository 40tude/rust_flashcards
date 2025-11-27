// Rust guideline compliant 2024-01
use serde::{Deserialize, Serialize};

/// Flashcard data model.
///
/// Represents a single flashcard with question, answer, and optional categorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: i64,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub question_html: String,
    pub answer_html: String,
}

impl Flashcard {
    /// Creates a new flashcard instance.
    pub fn new(
        id: i64,
        category: Option<String>,
        subcategory: Option<String>,
        question_html: String,
        answer_html: String,
    ) -> Self {
        Self {
            id,
            category,
            subcategory,
            question_html,
            answer_html,
        }
    }
}

/// Filter criteria for flashcard queries.
///
/// Defines filters to apply when querying flashcards: keywords for full-text search,
/// categories/subcategories for taxonomic filtering, and image inclusion control.
///
/// # Examples
/// ```ignore
/// let criteria = FilterCriteria {
///     keywords: vec!["rust".to_string(), "async".to_string()],
///     categories: Some(vec!["Programming".to_string()]),
///     subcategories: None,  // All subcategories
///     include_images: true,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct FilterCriteria {
    /// Keywords for FTS5 full-text search (AND logic).
    pub keywords: Vec<String>,
    /// Selected categories (None = all categories).
    pub categories: Option<Vec<String>>,
    /// Selected subcategories (None = all subcategories).
    pub subcategories: Option<Vec<String>>,
    /// Whether to include image-only flashcards.
    pub include_images: bool,
}