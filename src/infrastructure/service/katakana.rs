use std::{future::Future, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::Response,
};

use crate::domain::errors::KatakanaGetFailed;

pub(crate) trait KatakanaServiceInterface: Send + Sync + 'static {
    fn get(&self, word: String) -> impl Future<Output = Result<Response>> + Send;
}

pub(crate) async fn get<KatakanaService>(
    State(katakana_service): State<Arc<KatakanaService>>,
    Path(word): Path<String>,
) -> std::result::Result<Response, KatakanaGetFailed>
where
    KatakanaService: KatakanaServiceInterface,
{
    Ok(katakana_service.get(word).await?)
}
