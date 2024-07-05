use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Arpabet {
    pub word: String,
    pub pronunciation: Vec<String>,
}
