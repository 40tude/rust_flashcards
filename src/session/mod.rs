// Rust guideline compliant 2024-01
use serde::{Deserialize, Serialize};

/// Session data for tracking user state across requests.
///
/// Stores flashcard browsing state including seen cards, search/filter settings,
/// and cached counts. Session persists filter preferences across page visits.
#[derive(Serialize, Deserialize, Clone)]
pub struct SessionData {
    /// IDs of flashcards already shown in current session.
    pub seen_ids: Vec<i64>,
    /// Legacy: IDs of searched cards (kept for compatibility).
    pub searched_ids: Vec<i64>,
    /// Legacy: search keywords (kept for compatibility).
    pub keywords: Vec<String>,
    /// Cached total card count.
    pub nb_cards: Option<i64>,

    // Filter state (persists across visits)
    /// Keywords for filtered practice sessions.
    pub filter_keywords: Vec<String>,
    /// Selected categories (None = all categories).
    pub filter_categories: Option<Vec<String>>,
    /// Selected subcategories (None = all subcategories).
    pub filter_subcategories: Option<Vec<String>>,
    /// Whether to include image-only cards in practice.
    pub filter_include_images: bool,
    /// Cached count of cards matching current filters.
    pub filtered_card_count: Option<i64>,
    /// Error message to display on landing page.
    pub error_message: Option<String>,
}

impl SessionData {
    /// Creates new session with default values.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for SessionData {
    /// Returns session with filter defaults.
    ///
    /// Filters default to include everything: empty keywords, all categories,
    /// all subcategories, and all images.
    fn default() -> Self {
        Self {
            seen_ids: Vec::new(),
            searched_ids: Vec::new(),
            keywords: Vec::new(),
            nb_cards: None,
            filter_keywords: Vec::new(),
            filter_categories: None,
            filter_subcategories: None,
            filter_include_images: true,
            filtered_card_count: None,
            error_message: None,
        }
    }
}
