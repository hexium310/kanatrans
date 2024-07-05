#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("cannot parse {word:?} as ARPAbet, caused by: {source}")]
    ParseArpabetError {
        word: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("cannot convert {pronunciation:?} to Katakana, caused by: {source}")]
    ConvertKatakanaError {
        pronunciation: Vec<String>,
        #[source]
        source: anyhow::Error,
    },
}
