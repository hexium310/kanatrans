use std::convert::Infallible;

use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;
use tower::util::BoxCloneSyncService;

pub type Body = Full<Bytes>;
pub type Service<RequestBody> = BoxCloneSyncService<Request<RequestBody>, Response<Body>, Infallible>;

pub trait Handler<RequsetBody> {
    fn handle(
        &self,
        req: Request<RequsetBody>,
    ) -> impl std::future::Future<Output = Result<Response<Body>, Infallible>> + Send;
}
