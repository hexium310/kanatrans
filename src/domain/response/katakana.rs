use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Katakana {
    pub(crate) word: Option<String>,
    pub(crate) pronunciation: String,
}
