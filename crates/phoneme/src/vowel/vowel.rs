use std::ops::Deref;

use hashbrown::HashMap;
use once_cell::sync::Lazy;

use super::pattern::{VowelPattern, VOWEL_PATTERNS};

static VOWELS: Lazy<Vowel> = Lazy::new(Vowel::default);

#[derive(Debug)]
pub struct Vowel {
    vowels: HashMap<&'static str, VowelPattern>,
}

impl Deref for Vowel {
    type Target = HashMap<&'static str, VowelPattern>;

    fn deref(&self) -> &Self::Target {
        &self.vowels
    }
}

impl Default for Vowel {
    fn default() -> Self {
        let vowels = VOWEL_PATTERNS.into();

        Self { vowels }
    }
}

pub fn vowels() -> &'static Vowel {
    &VOWELS
}
