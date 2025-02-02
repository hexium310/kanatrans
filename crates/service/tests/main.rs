use bytes::Bytes;
use http::{Method, Request, StatusCode};
use http_body_util::{BodyExt, Empty};
use service::routing::Router;

#[tokio::test]
async fn default_routing_arpabet() {
    let router = Router::<Empty::<Bytes>>::default();

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost/arpabet/route")
        .body(Empty::default())
        .unwrap();

    let res = router.route(req).await.unwrap();

    assert!(res.status().is_success());
    assert_eq!(&res.body().clone().collect().await.unwrap().to_bytes()[..], b"{\"word\":\"route\",\"pronunciation\":[\"r\",\"uw1\",\"t\"]}");
}

#[tokio::test]
async fn default_routing_katakana() {
    let router = Router::<Empty::<Bytes>>::default();

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost/katakana?pronunciation=r+uw1+t")
        .body(Empty::default())
        .unwrap();

    let res = router.route(req).await.unwrap();

    assert!(res.status().is_success());
    assert_eq!(&res.body().clone().collect().await.unwrap().to_bytes()[..], "{\"pronunciation\":\"ルート\"}".as_bytes());
}

#[tokio::test]
async fn default_routing_not_found() {
    let router = Router::<Empty::<Bytes>>::default();

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost")
        .body(Empty::default())
        .unwrap();

    let res = router.route(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(&res.body().clone().collect().await.unwrap().to_bytes()[..], b"");

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost/")
        .body(Empty::default())
        .unwrap();

    let res = router.route(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(&res.body().clone().collect().await.unwrap().to_bytes()[..], b"");

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost/arpabet")
        .body(Empty::default())
        .unwrap();

    let res = router.route(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(&res.body().clone().collect().await.unwrap().to_bytes()[..], b"");

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost/arpabet/")
        .body(Empty::default())
        .unwrap();

    let res = router.route(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(&res.body().clone().collect().await.unwrap().to_bytes()[..], b"");

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost/arpabet/route/route")
        .body(Empty::default())
        .unwrap();

    let res = router.route(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(&res.body().clone().collect().await.unwrap().to_bytes()[..], b"");

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost/katakana/")
        .body(Empty::default())
        .unwrap();

    let res = router.route(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(&res.body().clone().collect().await.unwrap().to_bytes()[..], b"");

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost/katakana/route")
        .body(Empty::default())
        .unwrap();

    let res = router.route(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(&res.body().clone().collect().await.unwrap().to_bytes()[..], b"");
}
