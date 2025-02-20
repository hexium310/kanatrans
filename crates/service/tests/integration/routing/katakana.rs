use adapter::processor::{conversion_table::ConversionTable, lex_lookup::LexLookup};
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use service::{arpabet::ArpabetService, katakana::KatakanaService, routing, Body};
use tower::ServiceExt;

#[tokio::test]
async fn ok() {
    let arpabet_service = ArpabetService::<LexLookup>::default();
    let katakana_service = KatakanaService::<ConversionTable>::default();
    let router = routing::router(arpabet_service, katakana_service);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/katakana?pronunciation=w+er1+d")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(req).await.unwrap();

    assert!(response.status().is_success());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    assert_eq!(
        body,
        serde_json::json!({
            "pronunciation": "ワード"
        })
    );
}

#[tokio::test]
async fn invalid_pronunciation() {
    let arpabet_service = ArpabetService::<LexLookup>::default();
    let katakana_service = KatakanaService::<ConversionTable>::default();
    let router = routing::router(arpabet_service, katakana_service);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/katakana?pronunciation=wer1d")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    assert_eq!(
        body,
        serde_json::json!({
            "status": 500,
            "title": "Internal Server Error",
            "detail": "cannot convert [\"wer1d\"] to Katakana, caused by: unexpected consonant: wer1d"
        })
    );
}
