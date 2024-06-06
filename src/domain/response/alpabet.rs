use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Alpabet {
    pub(crate) word: String,
    pub(crate) pronunciation: String,
}
