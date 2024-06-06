use std::{future::Future, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::Response,
};

use crate::domain::errors::AlpabetGetFailed;

pub(crate) trait AlpabetServiceInterface: Send + Sync + 'static {
    fn get(&self, word: String) -> impl Future<Output = Result<Response>> + Send;
}

pub(crate) async fn get<AlpabetService>(
    State(alpabet_service): State<Arc<AlpabetService>>,
    Path(word): Path<String>,
) -> std::result::Result<Response, AlpabetGetFailed>
where
    AlpabetService: AlpabetServiceInterface,
{
    Ok(alpabet_service.get(word).await?)
}
