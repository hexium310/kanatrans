use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Katakana {
    pub word: Option<String>,
    pub pronunciation: String,
}
