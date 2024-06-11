use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Params {
    pub(crate) word: Option<String>,
    pub(crate) pronunciation: String,
}
