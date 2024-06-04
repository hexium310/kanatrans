use std::{future::Future, sync::Arc};

use axum::{
    extract::{Path, State},
    response::Response,
};

use crate::domain::katakana::Word;

pub(crate) trait KatakanaServiceInterface: Send + Sync + 'static {
    fn get(&self, word: Word) -> impl Future<Output = Response> + Send;
}

pub(crate) async fn get<KatakanaService>(
    State(katakana_service): State<Arc<KatakanaService>>,
    Path(word): Path<Word>,
) -> Response
where
    KatakanaService: KatakanaServiceInterface,
{
    katakana_service.get(word).await
}
