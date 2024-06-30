use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Katakana(pub String);

impl From<Katakana> for String {
    fn from(val: Katakana) -> Self {
        val.0
    }
}

impl Katakana {
    pub fn new(string: String) -> Self {
        Self(string)
    }
}
