use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    pub word: Option<String>,
    pub pronunciation: String,
}
