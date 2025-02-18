use domain::processor::transcriber::Transcriber;

use crate::arpabet::{ArpabetService, ArpabetServiceInterface};

mockall::mock! {
    #[derive(Debug)]
    Transcriber {}
    impl Transcriber for Transcriber {
        type Target = Vec<String>;

        fn transcribe(&self, word: &str) -> anyhow::Result<Vec<String>>;
    }
}

#[tokio::test]
async fn arpabet_service_get_success() {
    let mut mock_transcriber = MockTranscriber::new();
    mock_transcriber
        .expect_transcribe()
        .times(1)
        .withf(|x| x == "word")
        .returning(|_| Ok(vec!["w".to_string(), "er1".to_string(), "d".to_string()]));
    let arpabet_service = ArpabetService::new(mock_transcriber);

    let arpabet = arpabet_service.get("word".to_string()).await.unwrap();
    assert_eq!(arpabet.word, "word".to_string());
    assert_eq!(arpabet.pronunciation, ["w".to_string(), "er1".to_string(), "d".to_string()]);
}
