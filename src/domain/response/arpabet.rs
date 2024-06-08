use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Arpabet {
    pub(crate) word: String,
    pub(crate) pronunciation: String,
}
