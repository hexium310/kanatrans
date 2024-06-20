use std::{future::Future, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::Response,
};

use crate::domain::errors::ApiError;

pub(crate) trait ArpabetServiceInterface: Send + Sync + 'static {
    fn get(&self, word: String) -> impl Future<Output = Result<Response>> + Send;
}

pub(crate) async fn get<ArpabetService>(
    State(arpabet_service): State<Arc<ArpabetService>>,
    Path(word): Path<String>,
) -> std::result::Result<Response, ApiError>
where
    ArpabetService: ArpabetServiceInterface,
{
    Ok(arpabet_service.get(word).await.map_err(|err| {
        if err.to_string() == "cannot convert to ARPAbet" {
            return ApiError::CannotParseAsArpabet(err);
        }

        ApiError::ArpabetGetFailed(err)
    })?)
}
