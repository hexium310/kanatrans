use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Arpabet(pub(crate) Vec<String>);

impl Deref for Arpabet {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<String>> for Arpabet {
    fn from(arpabet: Vec<String>) -> Self {
        Self(arpabet)
    }
}
