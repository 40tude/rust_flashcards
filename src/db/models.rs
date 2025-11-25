use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: i64,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub question_html: String,
    pub answer_html: String,
}

impl Flashcard {
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