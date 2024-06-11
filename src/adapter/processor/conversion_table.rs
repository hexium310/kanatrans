use anyhow::Result;

use crate::{
    adapter::converter::Converter,
    domain::{
        katakana::Katakana,
        phoneme_pair::{Assembler, PhonemePairs},
        processor::transliterator::Transliterator,
    },
};

pub(crate) struct ConversionTable<Converter> {
    converter: Converter,
}

pub(crate) struct KatakanaConverter;

impl<KatakanaConverter> Transliterator for ConversionTable<KatakanaConverter>
where
    KatakanaConverter: Converter,
{
    type Target = Katakana;

    fn transliterate(&self, pronunciation: &[&str]) -> Result<Self::Target> {
        let output = self.converter.convert(pronunciation)?;

        Ok(Katakana::new(output))
    }
}

impl Converter for KatakanaConverter {
    fn convert(&self, pronunciation: &[&str]) -> Result<String> {
        let phoneme_pairs = PhonemePairs::from(pronunciation);
        let katakana_list = phoneme_pairs
            .iter()
            .map(|phoneme_pair| phoneme_pair.assemble())
            .collect::<Result<Vec<_>>>()?;
        Ok(katakana_list.join(""))
    }
}

impl<Converter> ConversionTable<Converter> {
    pub(crate) fn new(converter: Converter) -> Self {
        Self { converter }
    }
}

impl KatakanaConverter {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::KatakanaConverter;
    use crate::adapter::converter::Converter;

    #[test]
    fn convert() {
        let converter = KatakanaConverter::new();

        #[rustfmt::skip]
        let threshold = converter
            .convert(&["th", "r", "eh1", "sh", "ow1", "l", "d"].map(Into::into))
            .unwrap();
        assert_eq!(threshold, "スレショウルド");

        #[rustfmt::skip]
        let akabane = converter
            .convert(&["g", "eh1", "t", "s"].map(Into::into))
            .unwrap();
        assert_eq!(akabane, "ゲツ");

        #[rustfmt::skip]
        let akabane = converter
            .convert(&["k", "r", "iy0", "ey1", "t"].map(Into::into))
            .unwrap();
        assert_eq!(akabane, "クリーエイト");
    }
}
