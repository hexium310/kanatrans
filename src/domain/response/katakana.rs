use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Katakana {
    pub(crate) word: String,
    pub(crate) pronunciation: String,
}
