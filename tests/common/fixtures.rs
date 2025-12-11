// Rust guideline compliant 2025-01
//! Test fixtures for markdown content and filter criteria.
//!
//! Provides sample markdown files, filter combinations, and expected
//! HTML output patterns for comprehensive testing.

use crate::common::TestCard;

/// Valid markdown with category, subcategory, and question.
///
/// Format: "Question : CATEGORY - SUBCATEGORY - QUESTION_TEXT\nAnswer : ANSWER_TEXT"
pub const VALID_MARKDOWN: &str = r#"Question : Math - Algebra - What is 2+2?
Answer : The answer is 4."#;

/// Valid markdown without category (non-compliant format).
///
/// Should parse but category/subcategory will be None.
pub const MARKDOWN_NO_CATEGORY: &str = r#"Question : What is the capital of France?
Answer : Paris is the capital of France."#;

/// Markdown with HTML comment that should be stripped.
pub const MARKDOWN_WITH_COMMENT: &str = r#"<!-- This is a comment -->
Question : Science - Physics - What is gravity?
Answer : Gravity is a force of attraction."#;

/// Markdown with category name containing hyphens.
///
/// Tests that category extraction handles hyphens in names correctly.
pub const MARKDOWN_HYPHENATED_CATEGORY: &str = r#"Question : Machine-Learning - Supervised - What is a neural network?
Answer : A neural network is a computational model."#;

/// Markdown with multiple spaces in "Answer  :" pattern.
pub const MARKDOWN_MULTIPLE_SPACES: &str = r#"Question : Programming - Rust - What is ownership?
Answer  : Ownership is Rust's memory safety guarantee."#;

/// Markdown with code block for syntax highlighting tests.
pub const MARKDOWN_WITH_CODE: &str = r#"Question : Programming - Rust - How to print in Rust?
Answer : Use the println! macro:

```rust
fn main() {
    println!("Hello, world!");
}
```
"#;

/// Multiple Q&A pairs in single markdown file.
pub const MARKDOWN_MULTIPLE_CARDS: &str = r#"Question : Math - Algebra - What is 2+2?
Answer : 4

Question : Math - Geometry - What is pi?
Answer : Pi is approximately 3.14159.

Question : Science - Chemistry - What is water's formula?
Answer : H2O"#;

/// Empty Q&A that should be skipped during parsing.
pub const MARKDOWN_EMPTY_QA: &str = r#"Question :
Answer :

Question : Math - Algebra - Valid question
Answer : Valid answer"#;

/// Malformed markdown with missing answer section.
pub const MARKDOWN_MISSING_ANSWER: &str = r#"Question : Math - Algebra - What is 2+2?"#;

/// Creates sample flashcards for testing database operations.
///
/// Returns vector of TestCard with various categories, subcategories,
/// and one image-only card.
///
/// # Examples
/// ```no_run
/// let cards = sample_flashcards();
/// assert_eq!(cards.len(), 10);
/// ```
pub fn sample_flashcards() -> Vec<TestCard> {
    vec![
        TestCard::new()
            .category("Math")
            .subcategory("Algebra")
            .question("What is 2+2?")
            .answer("4")
            .build(),
        TestCard::new()
            .category("Math")
            .subcategory("Algebra")
            .question("What is 3+3?")
            .answer("6")
            .build(),
        TestCard::new()
            .category("Math")
            .subcategory("Geometry")
            .question("What is pi?")
            .answer("Approximately 3.14159")
            .build(),
        TestCard::new()
            .category("Science")
            .subcategory("Physics")
            .question("What is gravity?")
            .answer("Force of attraction")
            .build(),
        TestCard::new()
            .category("Science")
            .subcategory("Physics")
            .question("What is speed of light?")
            .answer("299,792,458 m/s")
            .build(),
        TestCard::new()
            .category("Science")
            .subcategory("Chemistry")
            .question("What is water's formula?")
            .answer("H2O")
            .build(),
        TestCard::new()
            .category("Programming")
            .subcategory("Rust")
            .question("What is ownership?")
            .answer("Memory safety guarantee")
            .build(),
        TestCard::new()
            .category("Programming")
            .subcategory("Python")
            .question("What is a list comprehension?")
            .answer("Concise way to create lists")
            .build(),
        // Image-only card (no category/subcategory)
        TestCard::new()
            .image_only("/static/deck/img/diagram.png")
            .build(),
        TestCard::new()
            .image_only("/static/deck/img/chart.webp")
            .build(),
    ]
}

/// Sample filter criteria combinations for parametrized tests.
///
/// Returns vector of tuples: (description, keywords, categories, subcategories, include_images, expected_count)
/// Expected count based on sample_flashcards() data.
///
/// # Examples
/// ```no_run
/// let filters = sample_filter_criteria();
/// for (desc, keywords, cats, subcats, images, expected) in filters {
///     // Test each filter combination
/// }
/// ```
pub fn sample_filter_criteria() -> Vec<(
    &'static str,
    Vec<String>,
    Option<Vec<String>>,
    Option<Vec<String>>,
    bool,
    i64,
)> {
    vec![
        // (description, keywords, categories, subcategories, include_images, expected_count)
        ("No filters - all cards", vec![], None, None, true, 10),
        (
            "Exclude images",
            vec![],
            None,
            None,
            false,
            8, // 10 total - 2 images
        ),
        (
            "Category: Math",
            vec![],
            Some(vec!["Math".to_string()]),
            None,
            true,
            3, // 3 Math cards
        ),
        (
            "Category: Science",
            vec![],
            Some(vec!["Science".to_string()]),
            None,
            true,
            3, // 3 Science cards
        ),
        (
            "Empty categories (images only)",
            vec![],
            Some(vec![]),
            None,
            true,
            2, // 2 image-only cards
        ),
        (
            "Subcategory: Algebra",
            vec![],
            Some(vec!["Math".to_string()]),
            Some(vec!["Algebra".to_string()]),
            true,
            2, // 2 Algebra cards
        ),
        (
            "Subcategory: Physics",
            vec![],
            Some(vec!["Science".to_string()]),
            Some(vec!["Physics".to_string()]),
            true,
            2, // 2 Physics cards
        ),
        (
            "Multiple categories",
            vec![],
            Some(vec!["Math".to_string(), "Science".to_string()]),
            None,
            true,
            6, // 3 Math + 3 Science
        ),
        (
            "Keyword: gravity",
            vec!["gravity".to_string()],
            None,
            None,
            true,
            1, // 1 card with "gravity"
        ),
        (
            "Keyword + Category",
            vec!["formula".to_string()],
            Some(vec!["Science".to_string()]),
            None,
            true,
            1, // 1 Science card with "formula"
        ),
    ]
}

/// Expected HTML patterns for markdown conversion tests.
///
/// Returns vector of tuples: (markdown_input, expected_html_fragment)
pub fn expected_html_patterns() -> Vec<(&'static str, &'static str)> {
    vec![
        // Basic markdown to HTML
        ("**bold**", "<strong>bold</strong>"),
        ("*italic*", "<em>italic</em>"),
        ("~~strikethrough~~", "<del>strikethrough</del>"),
        // Code inline
        ("`code`", "<code>code</code>"),
        // Headers
        ("### Header", "<h3>Header</h3>"),
        // Links
        (
            "[link](http://example.com)",
            r#"<a href="http://example.com">link</a>"#,
        ),
    ]
}

/// Sample markdown files for integration tests.
///
/// Returns vector of tuples: (filename, content)
pub fn sample_markdown_files() -> Vec<(&'static str, &'static str)> {
    vec![
        ("01_math.md", VALID_MARKDOWN),
        ("02_science.md", MARKDOWN_WITH_CODE),
        ("03_no_category.md", MARKDOWN_NO_CATEGORY),
        ("04_multiple.md", MARKDOWN_MULTIPLE_CARDS),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_flashcards_count() {
        let cards = sample_flashcards();
        assert_eq!(cards.len(), 10, "Should have 10 sample flashcards");

        // Verify category distribution
        let math_cards = cards
            .iter()
            .filter(|c| c.category.as_deref() == Some("Math"))
            .count();
        assert_eq!(math_cards, 3, "Should have 3 Math cards");

        // Verify image-only cards (no category)
        let image_cards = cards.iter().filter(|c| c.category.is_none()).count();
        assert_eq!(image_cards, 2, "Should have 2 image-only cards");
    }

    #[test]
    fn test_sample_filter_criteria_count() {
        let filters = sample_filter_criteria();
        assert_eq!(
            filters.len(),
            10,
            "Should have 10 sample filter combinations"
        );
    }

    #[test]
    fn test_markdown_constants_not_empty() {
        assert!(!VALID_MARKDOWN.is_empty());
        assert!(!MARKDOWN_NO_CATEGORY.is_empty());
        assert!(!MARKDOWN_WITH_COMMENT.is_empty());
        assert!(!MARKDOWN_HYPHENATED_CATEGORY.is_empty());
    }
}
