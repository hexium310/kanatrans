use anyhow::Result;

use crate::domain::{katakana::Katakana, processor::transliterator::Transliterator};

pub(crate) struct ConversionTable;

impl Transliterator for ConversionTable {
    type Target = Katakana;

    fn transliterate(&self, word: &str) -> Result<Self::Target> {
        todo!()
    }
}

impl ConversionTable {
    pub(crate) fn new() -> Self {
        Self
    }
}
