use std::borrow::Cow;

use crate::{error::AssemblerError, phoneme::Phoneme};

pub trait Assembler {
    fn assemble(&self) -> Result<Cow<'_, str>, AssemblerError>;
    fn assemble_both(&self, consonant: Phoneme<'_>, vowel: Phoneme<'_>) -> Result<Cow<'_, str>, AssemblerError>;
    fn assemble_consonant(&self, consonant: Phoneme<'_>) -> Result<Cow<'_, str>, AssemblerError>;
    fn assemble_vowel(&self, vowel: Phoneme<'_>, list: &[&str]) -> Result<Cow<'_, str>, AssemblerError>;
}
