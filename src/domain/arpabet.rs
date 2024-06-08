use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Arpabet(String);

impl AsRef<str> for Arpabet {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
