use std::{future::Future, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Query, State},
    response::Response,
};

use crate::domain::{errors::ApiError, request::katakana::Params};

pub(crate) trait KatakanaServiceInterface: Send + Sync + 'static {
    fn get(&self, params: Params) -> impl Future<Output = Result<Response>> + Send;
}

pub(crate) async fn get<KatakanaService>(
    State(katakana_service): State<Arc<KatakanaService>>,
    Query(params): Query<Params>,
) -> std::result::Result<Response, ApiError>
where
    KatakanaService: KatakanaServiceInterface,
{
    katakana_service.get(params).await.map_err(ApiError::KatakanaGetFailed)
}
