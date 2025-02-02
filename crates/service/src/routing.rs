use std::{collections::HashMap, convert::Infallible, future};

use adapter::processor::{conversion_table::ConversionTable, lex_lookup::LexLookup};
use http::{Method, Request, Response, StatusCode};
use http_body::Body as HttpBody;
use tower::{service_fn, Service as _};

use crate::{
    arpabet::ArpabetService,
    katakana::KatakanaService,
    service::{Body, Handler, Service},
};

type InnerRouter<RequestBody> = HashMap<Method, matchit::Router<Service<RequestBody>>>;

#[derive(Debug)]
pub struct Router<RequestBody> {
    inner: InnerRouter<RequestBody>,
}

impl<RequestBody> Default for Router<RequestBody>
where
    RequestBody: HttpBody + Send + 'static,
{
    fn default() -> Self {
        let mut router = InnerRouter::new();

        router
            .entry(Method::GET)
            .or_default()
            .insert(
                "/arpabet/{word}",
                Service::new(service_fn(|req: Request<RequestBody>| async move {
                    ArpabetService::<LexLookup>::default().handle(req).await
                })),
            )
            .expect("Failed to set up router for `/arpabet/{word}`");

        router
            .entry(Method::GET)
            .or_default()
            .insert(
                "/katakana",
                Service::new(service_fn(|req: Request<RequestBody>| async move {
                    KatakanaService::<ConversionTable>::default().handle(req).await
                })),
            )
            .expect("Failed to set up router for `/katakana`");

        Self { inner: router }
    }
}

impl<RequestBody> Router<RequestBody>
where
    RequestBody: HttpBody + Send,
{
    pub fn new() -> Self {
        Self {
            inner: InnerRouter::new(),
        }
    }

    pub async fn route(&self, mut req: Request<RequestBody>) -> Result<Response<Body>, Infallible> {
        let Some(router) = self.inner.get(req.method()) else {
            return Ok(Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::default())
                .expect("Failed to build METHOD NOT ALLOWED response"));
        };

        let Ok(found) = router.at(req.uri().path()) else {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::default())
                .expect("Failed to build NOT FOUND response"));
        };

        let mut service = found.value.clone();
        let params = found
            .params
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect::<HashMap<String, String>>();
        req.extensions_mut().insert(params);

        future::poll_fn(|cx| service.poll_ready(cx)).await?;

        service.call(req).await
    }
}
