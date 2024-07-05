use phoneme::error::AssemblerError;

#[derive(thiserror::Error, Debug)]
pub enum ProcessorError {
    #[error("no phonemes in parsed {word:?}")]
    NoPhonemes { word: String },
    #[error("invalid character in {word:?}")]
    InvalidCharacter {
        word: String,
        #[source]
        source: anyhow::Error,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum ConverterError {
    #[error("assembling ARPAbet failed, caused by: {source}")]
    AssembleFailed {
        #[source]
        source: AssemblerError,
    },
}
