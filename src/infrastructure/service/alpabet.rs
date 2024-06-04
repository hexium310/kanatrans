use std::{future::Future, sync::Arc};

use axum::{
    extract::{Path, State},
    response::Response,
};

use crate::domain::alpabet::Word;

pub(crate) trait AlpabetServiceInterface: Send + Sync + 'static {
    fn get(&self, word: Word) -> impl Future<Output = Response> + Send;
}

pub(crate) async fn get<AlpabetService>(
    State(alpabet_service): State<Arc<AlpabetService>>,
    Path(word): Path<Word>,
) -> Response
where
    AlpabetService: AlpabetServiceInterface,
{
    alpabet_service.get(word).await
}
