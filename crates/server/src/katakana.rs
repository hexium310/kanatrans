use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use service::katakana::KatakanaServiceInterface;

use crate::error::ServerError;

#[derive(Debug, Deserialize)]
pub struct Params {
    pub word: Option<String>,
    pub pronunciation: String,
}

pub async fn get<KatakanaService>(
    State(katakana_service): State<Arc<KatakanaService>>,
    Query(params): Query<Params>,
) -> Result<Response, ServerError>
where
    KatakanaService: KatakanaServiceInterface,
{
    let katakana = katakana_service
        .get(&params.pronunciation.split_whitespace().collect::<Vec<_>>())
        .await
        .map_err(ServerError::KatakanaGetFailed)?;

    Ok(Json(katakana).into_response())
}
