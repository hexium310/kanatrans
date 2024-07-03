#[derive(thiserror::Error, Debug)]
pub enum AssemblerError {
    #[error("unexpected consonant: {0}")]
    UnexpectedConsonant(String),
    #[error("unexpected vowel: {0}")]
    UnexpectedVowel(String),
}
