use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use futures::future;
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::{
    arpabet::{Arpabet, MockArpabetServiceInterface},
    katakana::{Katakana, MockKatakanaServiceInterface},
    routing,
};

#[tokio::test]
async fn arpabet() {
    let mut mock_arpabet_service = MockArpabetServiceInterface::new();
    mock_arpabet_service
        .expect_get()
        .times(1)
        .withf(|x| x == "word")
        .returning(|_| {
            Box::pin(future::ok(Arpabet {
                word: "word".to_string(),
                pronunciation: vec!["w".to_string(), "er1".to_string(), "d".to_string()],
            }))
        });

    let mut mock_katakana_service = MockKatakanaServiceInterface::new();
    mock_katakana_service
        .expect_get()
        .times(0);

    let router = routing::router(mock_arpabet_service, mock_katakana_service);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/arpabet/word")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(req).await.unwrap();

    assert!(response.status().is_success());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    assert_eq!(
        body,
        serde_json::json!({
            "word": "word",
            "pronunciation": ["w", "er1", "d"]
        })
    );
}

#[tokio::test]
async fn katakana() {
    let mut mock_arpabet_service = MockArpabetServiceInterface::new();
    mock_arpabet_service
        .expect_get()
        .times(0);

    let mut mock_katakana_service = MockKatakanaServiceInterface::new();
    mock_katakana_service
        .expect_get()
        .times(1)
        .withf(|x| x == ["w", "er1", "d"])
        .returning(|_| {
            Box::pin(future::ok(Katakana {
                pronunciation: "ワード".to_string(),
            }))
        });

    let router = routing::router(mock_arpabet_service, mock_katakana_service);

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
            "pronunciation":
            "ワード"
        })
    );
}

#[tokio::test]
async fn not_found() {
    let mut mock_arpabet_service = MockArpabetServiceInterface::new();
    mock_arpabet_service
        .expect_get()
        .times(0);

    let mut mock_katakana_service = MockKatakanaServiceInterface::new();
    mock_katakana_service
        .expect_get()
        .times(0);

    let router = routing::router(mock_arpabet_service, mock_katakana_service);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/does-not-exist")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}
