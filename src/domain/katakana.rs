use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Katakana(String);

impl AsRef<str> for Katakana {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
