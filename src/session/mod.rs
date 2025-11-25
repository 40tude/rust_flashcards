use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SessionData {
    pub seen_ids: Vec<i64>,
    pub searched_ids: Vec<i64>,
    pub keywords: Vec<String>,
    pub nb_cards: Option<i64>,
}

impl SessionData {
    pub fn new() -> Self {
        Self::default()
    }
}
