use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Katakana(pub(crate) String);

impl Into<String> for Katakana {
    fn into(self) -> String {
        self.0
    }
}

impl Katakana {
    pub(crate) fn new(string: String) -> Self {
        Self(string)
    }
}
