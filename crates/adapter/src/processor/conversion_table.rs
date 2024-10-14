use anyhow::{Error, Result};
use domain::{katakana::Katakana, processor::transliterator::Transliterator};
use itertools::Itertools;
use phoneme::{assembler::Assembler, phoneme_pair::PhonemePairs};

use crate::converter::Converter;

#[derive(Debug, Default)]
pub struct ConversionTable<Converter = KatakanaConverter> {
    converter: Converter,
}

#[derive(Debug, Default)]
pub struct KatakanaConverter;

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
        let katakana = phoneme_pairs
            .iter()
            .map(|phoneme_pair| phoneme_pair.assemble().map_err(Error::from))
            .collect::<Result<String>>()?;
        let katakana = katakana.chars().dedup_by(|a, b| *a == 'ー' && a == b).collect();

        Ok(katakana)
    }
}

impl<Converter> ConversionTable<Converter> {
    pub fn new(converter: Converter) -> Self {
        Self { converter }
    }
}

#[cfg(test)]
mod tests {
    use domain::{katakana::Katakana, processor::transliterator::Transliterator};

    use crate::{
        converter::{Converter, MockConverter},
        processor::conversion_table::{ConversionTable, KatakanaConverter},
    };

    #[test]
    fn transliterate() {
        let mut mock_converter = MockConverter::new();
        mock_converter
            .expect_convert()
            .times(1)
            .withf(|x| x == ["th", "r", "eh1", "sh", "ow1", "l", "d"])
            .returning(|_| Ok("スレショウルド".to_string()));

        let conversion_table = ConversionTable::new(mock_converter);
        let katakana = conversion_table
            .transliterate(&["th", "r", "eh1", "sh", "ow1", "l", "d"])
            .unwrap();

        assert_eq!(katakana, Katakana("スレショウルド".to_string()));
    }

    #[test]
    fn convert() {
        let converter = KatakanaConverter;

        #[rustfmt::skip]
        let threshold = converter
            .convert(&["th", "r", "eh1", "sh", "ow1", "l", "d"])
            .unwrap();
        assert_eq!(threshold, "スレショウルド");

        #[rustfmt::skip]
        let get = converter
            .convert(&["g", "eh1", "t", "s"])
            .unwrap();
        assert_eq!(get, "ゲツ");

        #[rustfmt::skip]
        let akabane = converter
            .convert(&["k", "r", "iy0", "ey1", "t"])
            .unwrap();
        assert_eq!(akabane, "クリーエイト");

        #[rustfmt::skio]
        let support = converter
            .convert(&["s", "ax0", "p", "ao", "r", "t"])
            .unwrap();
        assert_eq!(support, "サポート");

        #[rustfmt::skio]
        let atmosphere = converter
            .convert(&["ae1", "t", "m", "ax0", "s", "f", "ih1", "r"])
            .unwrap();
        assert_eq!(atmosphere, "アトマスフィアー");

        #[rustfmt::skio]
        let food = converter
            .convert(&["f", "uw1", "d"])
            .unwrap();
        assert_eq!(food, "フード");
    }
}
