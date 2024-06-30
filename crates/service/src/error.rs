#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("getting arpabet failed")]
    ArpabetGetFailed(#[source] anyhow::Error),
    #[error("getting katakana failed")]
    KatakanaGetFailed(#[source] anyhow::Error),
}
