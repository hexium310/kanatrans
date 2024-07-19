use anyhow::Result;

pub trait Runner<ArpabetService, KatakanaService> {
    fn run(
        &self,
        arpabet_service: ArpabetService,
        katakana_service: KatakanaService,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}
