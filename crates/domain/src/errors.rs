use anyhow::Error;

pub enum ApiError {
    ArpabetGetFailed(Error),
    CannotParseAsArpabet(Error),
    KatakanaGetFailed(Error),
}
