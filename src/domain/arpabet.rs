use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Arpabet(pub(crate) Vec<String>);

impl ToString for Arpabet {
    fn to_string(&self) -> String {
        self.0.join("")
    }
}

impl Arpabet {
    pub(crate) fn new(arpabet: &[String]) -> Self {
        Self(arpabet.to_owned())
    }
}
