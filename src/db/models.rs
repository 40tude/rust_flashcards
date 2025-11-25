use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: i64,
    pub question_html: String,
    pub answer_html: String,
}

impl Flashcard {
    pub fn new(id: i64, question_html: String, answer_html: String) -> Self {
        Self {
            id,
            question_html,
            answer_html,
        }
    }
}