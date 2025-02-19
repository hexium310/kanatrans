use domain::processor::transliterator::Transliterator;

use crate::katakana::{KatakanaService, KatakanaServiceInterface};

mockall::mock! {
    #[derive(Debug)]
    Transliterator {}
    impl Transliterator for Transliterator {
        type Target = String;

        fn transliterate<'a>(&self, word: &[&'a str]) -> anyhow::Result<String>;
    }
}

#[tokio::test]
async fn katakana_service_get_success() {
    let mut mock_transliterator = MockTransliterator::new();
    mock_transliterator
        .expect_transliterate()
        .times(1)
        .withf(|x| x == ["w".to_string(), "er1".to_string(), "d".to_string()])
        .returning(|_| Ok("ワード".to_string()));
    let katakana_service = KatakanaService::new(mock_transliterator);

    let katakana = katakana_service.get(&["w", "er1", "d"]).await.unwrap();
    assert_eq!(katakana.pronunciation, "ワード".to_string());
}
