use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Katakana(pub(crate) String);

impl AsRef<str> for Katakana {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Katakana {
    pub(crate) fn new(string: String) -> Self {
        Self(string)
    }
}
