use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Alpabet(String);

impl AsRef<str> for Alpabet {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
