use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use service::arpabet::ArpabetServiceInterface;

use crate::error::ServerError;

pub async fn get<ArpabetService>(
    State(arpabet_service): State<Arc<ArpabetService>>,
    Path(word): Path<String>,
) -> Result<Response, ServerError>
where
    ArpabetService: ArpabetServiceInterface,
{
    let arpabet = arpabet_service.get(word).await.map_err(ServerError)?;

    Ok(Json(arpabet).into_response())
}
