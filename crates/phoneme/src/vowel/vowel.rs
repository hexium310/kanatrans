use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use super::pattern::{VowelPattern, VOWEL_BASE, VOWEL_PATTERNS};

static VOWELS: LazyLock<Vowel> = LazyLock::new(Vowel::default);

#[derive(Debug)]
pub struct Vowel {
    vowels: HashMap<&'static str, VowelPattern>,
    base: &'static [&'static str],
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

        Self {
            vowels,
            base: &VOWEL_BASE,
        }
    }
}

impl Vowel {
    pub fn base(&self) -> &[&str] {
        self.base
    }
}

pub fn vowels() -> &'static Vowel {
    &VOWELS
}
