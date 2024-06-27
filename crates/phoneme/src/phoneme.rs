use std::ops::Deref;

use crate::{consonant::consonants, vowel::pattern::VOWEL_PATTERNS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phoneme<'a> {
    Consonant(&'a str),
    Vowel(&'a str),
    Unexpected(&'a str),
}

impl<'a> Deref for Phoneme<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        match self {
            Phoneme::Consonant(value) => value,
            Phoneme::Vowel(value) => value,
            Phoneme::Unexpected(value) => value,
        }
    }
}

impl<'a> From<&'a str> for Phoneme<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            value if consonants().iter().any(|(key, _)| key == &value) => Self::Consonant(value),
            value if VOWEL_PATTERNS.iter().any(|(key, _)| key == &value) => Self::Vowel(value),
            value => Self::Unexpected(value),
        }
    }
}

impl<'a> Phoneme<'a> {}
