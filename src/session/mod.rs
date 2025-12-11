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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_default_values() {
        let session = SessionData::default();

        // Verify all vectors start empty
        assert!(session.seen_ids.is_empty());
        assert!(session.searched_ids.is_empty());
        assert!(session.keywords.is_empty());
        assert!(session.filter_keywords.is_empty());

        // Verify all Options start as None or specific defaults
        assert_eq!(session.nb_cards, None);
        assert_eq!(session.filter_categories, None);
        assert_eq!(session.filter_subcategories, None);
        assert_eq!(session.filtered_card_count, None);
        assert_eq!(session.error_message, None);

        // Verify images included by default
        assert!(session.filter_include_images);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut session = SessionData::default();
        session.seen_ids = vec![1, 2, 3];
        session.filter_keywords = vec!["rust".to_string(), "async".to_string()];
        session.filter_categories = Some(vec!["Programming".to_string()]);
        session.filter_subcategories = Some(vec!["Rust".to_string()]);
        session.filter_include_images = false;
        session.filtered_card_count = Some(42);
        session.error_message = Some("Test error".to_string());

        // Serialize to JSON
        let json = serde_json::to_string(&session).unwrap();

        // Deserialize back
        let deserialized: SessionData = serde_json::from_str(&json).unwrap();

        // Verify all fields match
        assert_eq!(deserialized.seen_ids, vec![1, 2, 3]);
        assert_eq!(
            deserialized.filter_keywords,
            vec!["rust".to_string(), "async".to_string()]
        );
        assert_eq!(
            deserialized.filter_categories,
            Some(vec!["Programming".to_string()])
        );
        assert_eq!(
            deserialized.filter_subcategories,
            Some(vec!["Rust".to_string()])
        );
        assert!(!deserialized.filter_include_images);
        assert_eq!(deserialized.filtered_card_count, Some(42));
        assert_eq!(deserialized.error_message, Some("Test error".to_string()));
    }

    #[test]
    fn test_clone_trait() {
        let mut session = SessionData::default();
        session.seen_ids = vec![1, 2, 3];
        session.filter_keywords = vec!["test".to_string()];

        let cloned = session.clone();

        // Verify clone has same values
        assert_eq!(cloned.seen_ids, vec![1, 2, 3]);
        assert_eq!(cloned.filter_keywords, vec!["test".to_string()]);
    }

    #[test]
    fn test_filter_state_persistence() {
        let mut session = SessionData::default();

        // Simulate user setting filters
        session.filter_keywords = vec!["machine learning".to_string()];
        session.filter_categories = Some(vec!["Science".to_string()]);
        session.filter_subcategories = Some(vec!["AI".to_string()]);
        session.filter_include_images = false;

        // Simulate caching filter count
        session.filtered_card_count = Some(15);

        // Verify filter state is maintained
        assert_eq!(
            session.filter_keywords,
            vec!["machine learning".to_string()]
        );
        assert_eq!(
            session.filter_categories,
            Some(vec!["Science".to_string()])
        );
        assert_eq!(session.filter_subcategories, Some(vec!["AI".to_string()]));
        assert!(!session.filter_include_images);
        assert_eq!(session.filtered_card_count, Some(15));
    }

    #[test]
    fn test_seen_ids_tracking() {
        let mut session = SessionData::default();

        // Simulate viewing flashcards
        session.seen_ids.push(1);
        session.seen_ids.push(2);
        session.seen_ids.push(3);

        assert_eq!(session.seen_ids.len(), 3);
        assert_eq!(session.seen_ids, vec![1, 2, 3]);

        // Simulate reset
        session.seen_ids.clear();
        assert!(session.seen_ids.is_empty());
    }

    #[test]
    fn test_error_message_handling() {
        let mut session = SessionData::default();

        // Initially no error
        assert_eq!(session.error_message, None);

        // Set error message
        session.error_message = Some("No cards match filters".to_string());
        assert_eq!(
            session.error_message,
            Some("No cards match filters".to_string())
        );

        // Clear error
        session.error_message = None;
        assert_eq!(session.error_message, None);
    }

    #[test]
    fn test_empty_categories_filter() {
        let mut session = SessionData::default();

        // Empty categories means "images only" mode
        session.filter_categories = Some(vec![]);
        session.filter_subcategories = None;

        assert_eq!(session.filter_categories, Some(vec![]));
        assert_eq!(session.filter_subcategories, None);
    }

    #[test]
    fn test_cached_counts() {
        let mut session = SessionData::default();

        // Initially no cached counts
        assert_eq!(session.nb_cards, None);
        assert_eq!(session.filtered_card_count, None);

        // Set cached counts
        session.nb_cards = Some(100);
        session.filtered_card_count = Some(25);

        assert_eq!(session.nb_cards, Some(100));
        assert_eq!(session.filtered_card_count, Some(25));
    }
}

