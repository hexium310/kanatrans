use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Katakana(pub(crate) String);

impl From<Katakana> for String {
    fn from(val: Katakana) -> Self {
        val.0
    }
}

impl Katakana {
    pub(crate) fn new(string: String) -> Self {
        Self(string)
    }
}
