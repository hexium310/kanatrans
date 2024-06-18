use std::{borrow::Cow, ops::Deref};

use anyhow::{bail, Result};
use hashbrown::HashMap;
use once_cell::sync::Lazy;

pub(crate) const KATAKANA_BASE: [&str; 5] = ["ア", "イ", "ウ", "エ", "オ"];

pub(crate) static CONSONANTS: Lazy<Consonants> = Lazy::new(|| {
    [
        (
            "p",
            ConsonantPattern {
                with_vowel: ["パ", "ピ", "ピュ", "ペ", "ポ"],
                unit: "プ",
            },
        ),
        (
            "b",
            ConsonantPattern {
                with_vowel: ["バ", "ビ", "ビュ", "ベ", "ボ"],
                unit: "ブ",
            },
        ),
        (
            "t",
            ConsonantPattern {
                with_vowel: ["タ", "ティ", "トゥ", "テ", "ト"],
                unit: "ト",
            },
        ),
        (
            "d",
            ConsonantPattern {
                with_vowel: ["ダ", "ディ", "ドゥ", "デ", "ド"],
                unit: "ド",
            },
        ),
        (
            "ts",
            ConsonantPattern {
                with_vowel: ["タ", "ティ", "ツ", "テ", "ト"],
                unit: "ツ",
            },
        ),
        (
            "ds",
            ConsonantPattern {
                with_vowel: ["ダ", "ディ", "ズ", "デ", "ド"],
                unit: "ズ",
            },
        ),
        (
            "ch",
            ConsonantPattern {
                with_vowel: ["チャ", "チ", "チュ", "チェ", "チョ"],
                unit: "チ",
            },
        ),
        (
            "dh",
            ConsonantPattern {
                with_vowel: ["ジャ", "ジ", "ジュ", "ジェ", "ジョ"],
                unit: "ジ",
            },
        ),
        (
            "k",
            ConsonantPattern {
                with_vowel: ["カ", "キ", "ク", "ケ", "コ"],
                unit: "ク",
            },
        ),
        (
            "g",
            ConsonantPattern {
                with_vowel: ["ガ", "ギ", "グ", "ゲ", "ゴ"],
                unit: "グ",
            },
        ),
        (
            "f",
            ConsonantPattern {
                with_vowel: ["ファ", "フィ", "フュ", "フェ", "フォ"],
                unit: "フ",
            },
        ),
        (
            "v",
            ConsonantPattern {
                with_vowel: ["ヴァ", "ヴィ", "ヴ", "ヴェ", "ヴォ"],
                unit: "ヴ",
            },
        ),
        (
            "s",
            ConsonantPattern {
                with_vowel: ["サ", "シ", "ス", "セ", "ソ"],
                unit: "ス",
            },
        ),
        (
            "z",
            ConsonantPattern {
                with_vowel: ["ザ", "ジ", "ズ", "ゼ", "ゾ"],
                unit: "ズ",
            },
        ),
        (
            "th",
            ConsonantPattern {
                with_vowel: ["サ", "シ", "ス", "セ", "ソ"],
                unit: "ス",
            },
        ),
        (
            "dh",
            ConsonantPattern {
                with_vowel: ["ザ", "ジ", "ズ", "ゼ", "ゾ"],
                unit: "ズ",
            },
        ),
        (
            "sh",
            ConsonantPattern {
                with_vowel: ["シャ", "シ", "シュ", "シェ", "ショ"],
                unit: "シュ",
            },
        ),
        (
            "zh",
            ConsonantPattern {
                with_vowel: ["ジャ", "ジ", "ジュ", "ジェ", "ジョ"],
                unit: "ジュ",
            },
        ),
        (
            "jh",
            ConsonantPattern {
                with_vowel: ["ジャ", "ジ", "ジュ", "ジェ", "ジョ"],
                unit: "ジ",
            },
        ),
        (
            "hh",
            ConsonantPattern {
                with_vowel: ["ハ", "ヒ", "ヒュ", "ヘ", "ホ"],
                unit: "フ",
            },
        ),
        (
            "m",
            ConsonantPattern {
                with_vowel: ["マ", "ミ", "ム", "メ", "モ"],
                unit: "ン",
            },
        ),
        (
            "n",
            ConsonantPattern {
                with_vowel: ["ナ", "ニ", "ヌ", "ネ", "ノ"],
                unit: "ン",
            },
        ),
        (
            "ng",
            ConsonantPattern {
                with_vowel: ["ン", "ン", "ン", "ン", "ン"],
                unit: "ン",
            },
        ),
        (
            "l",
            ConsonantPattern {
                with_vowel: ["ラ", "リ", "ル", "レ", "ロ"],
                unit: "ル",
            },
        ),
        (
            "r",
            ConsonantPattern {
                with_vowel: ["ラ", "リ", "ル", "レ", "ロ"],
                unit: "アー",
            },
        ),
        (
            "w",
            ConsonantPattern {
                with_vowel: ["ワ", "ウィ", "ウ", "ウェ", "ウォ"],
                unit: "ウ",
            },
        ),
        (
            "y",
            ConsonantPattern {
                with_vowel: ["ヤ", "イ", "ユ", "イェ", "ヨ"],
                unit: "ユ",
            },
        ),
    ]
    .into()
});

pub(crate) static CONSONANT_TS_DS: Lazy<ConsonantTsDs> = Lazy::new(|| [("t", "ts"), ("d", "ds")].into());

pub(crate) static VOWELS: Lazy<Vowels> = Lazy::new(|| {
    [
        (
            "ae",
            VowelPattern {
                position: 0,
                extension: None,
            },
        ),
        (
            "aa",
            VowelPattern {
                position: 0,
                extension: None,
            },
        ),
        (
            "ao",
            VowelPattern {
                position: 4,
                extension: Some(Extension::LongVowel),
            },
        ),
        (
            "ax",
            VowelPattern {
                position: 0,
                extension: None,
            },
        ),
        (
            "ih",
            VowelPattern {
                position: 1,
                extension: None,
            },
        ),
        (
            "iy",
            VowelPattern {
                position: 1,
                extension: Some(Extension::LongVowel),
            },
        ),
        (
            "ey",
            VowelPattern {
                position: 3,
                extension: Some(Extension::DiphthongI),
            },
        ),
        (
            "eh",
            VowelPattern {
                position: 3,
                extension: None,
            },
        ),
        (
            "ah",
            VowelPattern {
                position: 0,
                extension: None,
            },
        ),
        (
            "uh",
            VowelPattern {
                position: 2,
                extension: None,
            },
        ),
        (
            "uw",
            VowelPattern {
                position: 2,
                extension: Some(Extension::LongVowel),
            },
        ),
        (
            "ay",
            VowelPattern {
                position: 0,
                extension: Some(Extension::DiphthongI),
            },
        ),
        (
            "oy",
            VowelPattern {
                position: 4,
                extension: Some(Extension::DiphthongI),
            },
        ),
        (
            "ow",
            VowelPattern {
                position: 4,
                extension: Some(Extension::DiphthongU),
            },
        ),
        (
            "aw",
            VowelPattern {
                position: 0,
                extension: Some(Extension::DiphthongU),
            },
        ),
        (
            "er",
            VowelPattern {
                position: 0,
                extension: Some(Extension::LongVowel),
            },
        ),
    ]
    .into()
});

pub(crate) struct ConsonantPattern {
    pub(crate) with_vowel: [&'static str; 5],
    pub(crate) unit: &'static str,
}

pub(crate) struct VowelPattern {
    pub(crate) position: usize,
    pub(crate) extension: Option<Extension>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PhonemePair<'a> {
    consonant: Option<&'a str>,
    vowel: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PhonemePairs<'a>(Vec<PhonemePair<'a>>);

pub(crate) enum Extension {
    DiphthongI,
    DiphthongU,
    LongVowel,
}

pub(crate) type Consonants = HashMap<&'static str, ConsonantPattern>;
pub(crate) type ConsonantTsDs = HashMap<&'static str, &'static str>;
pub(crate) type Vowels = HashMap<&'static str, VowelPattern>;

pub(crate) trait Assembler {
    fn assemble(&self) -> Result<Cow<'_, str>>;
    fn assemble_both(&self) -> Result<Cow<'_, str>>;
    fn assemble_consonant(&self) -> Result<Cow<'_, str>>;
    fn assemble_vowel(&self, list: &[&str]) -> Result<Cow<'_, str>>;
}

impl<'a> Assembler for PhonemePair<'a> {
    fn assemble(&self) -> Result<Cow<'_, str>> {
        match (self.consonant, self.vowel) {
            (Some(_), Some(_)) => self.assemble_both(),
            (Some(_), None) => self.assemble_consonant(),
            (None, Some(_)) => self.assemble_vowel(&KATAKANA_BASE),
            (None, None) => unreachable!(),
        }
    }

    fn assemble_both(&self) -> Result<Cow<'_, str>> {
        let consonant = self.consonant.unwrap();

        let Some(ConsonantPattern { with_vowel, .. }) = CONSONANTS.get(consonant) else {
            bail!("unexpected consonant: {consonant}");
        };

        self.assemble_vowel(with_vowel)
    }

    fn assemble_consonant(&self) -> Result<Cow<'_, str>> {
        let consonant = self.consonant.unwrap();

        let Some(ConsonantPattern { unit, .. }) = CONSONANTS.get(consonant) else {
            bail!("unexpected consonant: {consonant}");
        };

        Ok(Cow::Borrowed(unit))
    }

    fn assemble_vowel(&self, list: &[&str]) -> Result<Cow<'_, str>> {
        let vowel = self.vowel.unwrap();

        let Some(vowel_pattern) = VOWELS.get(vowel) else {
            bail!("unexpected vowel: {vowel}");
        };

        let base = list[vowel_pattern.position];
        let extension = vowel_pattern.extension.as_ref().map_or("", |v| v.as_str());
        let katakana = [base, extension].join("");

        Ok(Cow::Owned(katakana))
    }
}

impl<'a> From<&'a [&str]> for PhonemePairs<'a> {
    fn from(value: &'a [&str]) -> Self {
        let phoneme_pairs = value.iter().fold(vec![], |mut accumulator: Vec<PhonemePair>, phoneme| {
            let previous_consonant = accumulator.last().and_then(|previous_pair| previous_pair.consonant());
            let previous_vowel = accumulator.last().and_then(|previous_pair| previous_pair.vowel());
            let phoneme = phoneme.trim_matches(char::is_numeric);

            let phoneme_pair = match phoneme {
                phoneme if CONSONANTS.contains_key(phoneme) => {
                    let consonant_ts_ds = previous_consonant
                        .filter(|_| phoneme == "s")
                        .filter(|_| previous_vowel.is_none())
                        .and_then(|consonant| CONSONANT_TS_DS.get(&consonant))
                        .inspect(|_| {
                            accumulator.truncate(accumulator.len() - 1);
                        })
                        .cloned();

                    PhonemePair::new(consonant_ts_ds.or(Some(phoneme)), None)
                },
                phoneme if VOWELS.contains_key(phoneme) => {
                    let previous_consonant = previous_consonant.filter(|_| previous_vowel.is_none()).inspect(|_| {
                        accumulator.truncate(accumulator.len() - 1);
                    });

                    PhonemePair::new(previous_consonant, Some(phoneme))
                },
                _ => return accumulator,
            };

            accumulator.push(phoneme_pair);
            accumulator
        });

        Self(phoneme_pairs)
    }
}

impl<'a> Deref for PhonemePairs<'a> {
    type Target = Vec<PhonemePair<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Extension {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Extension::DiphthongI => "イ",
            Extension::DiphthongU => "ウ",
            Extension::LongVowel => "ー",
        }
    }
}

impl<'a> PhonemePair<'a> {
    pub(crate) fn new(consonant: Option<&'a str>, vowel: Option<&'a str>) -> Self {
        Self { consonant, vowel }
    }

    pub(crate) fn consonant(&self) -> Option<&'a str> {
        self.consonant
    }

    pub(crate) fn vowel(&self) -> Option<&'a str> {
        self.vowel
    }
}

#[cfg(test)]
mod tests {
    mod phoneme_pairs_from {
        use pretty_assertions::assert_eq;

        use crate::domain::phoneme_pair::{PhonemePair, PhonemePairs};

        #[test]
        fn phoneme_pairs_from() {
            let arpabet = ["th", "r", "eh1", "sh", "ow1", "l", "d"];
            let arpabet = arpabet.as_slice();
            let phoneme_pairs = PhonemePairs::from(arpabet);
            assert_eq!(
                phoneme_pairs,
                PhonemePairs(vec![
                    PhonemePair {
                        consonant: Some("th"),
                        vowel: None,
                    },
                    PhonemePair {
                        consonant: Some("r"),
                        vowel: Some("eh"),
                    },
                    PhonemePair {
                        consonant: Some("sh"),
                        vowel: Some("ow"),
                    },
                    PhonemePair {
                        consonant: Some("l"),
                        vowel: None,
                    },
                    PhonemePair {
                        consonant: Some("d"),
                        vowel: None,
                    },
                ])
            );
        }

        #[test]
        fn ts() {
            let arpabet = ["t", "s"];
            let arpabet = arpabet.as_slice();
            let phoneme_pairs = PhonemePairs::from(arpabet);
            assert_eq!(
                phoneme_pairs,
                PhonemePairs(vec![PhonemePair {
                    consonant: Some("ts"),
                    vowel: None,
                },])
            );
        }

        #[test]
        fn two_or_more_vowels() {
            let arpabet = ["iy0", "ey1"];
            let arpabet = arpabet.as_slice();
            let phoneme_pairs = PhonemePairs::from(arpabet);
            assert_eq!(
                phoneme_pairs,
                PhonemePairs(vec![
                    PhonemePair {
                        consonant: None,
                        vowel: Some("iy"),
                    },
                    PhonemePair {
                        consonant: None,
                        vowel: Some("ey"),
                    },
                ])
            );
        }

        #[test]
        fn two_or_more_vowels_after_consonant() {
            let arpabet = ["r", "iy0", "ey1"];
            let arpabet = arpabet.as_slice();
            let phoneme_pairs = PhonemePairs::from(arpabet);
            assert_eq!(
                phoneme_pairs,
                PhonemePairs(vec![
                    PhonemePair {
                        consonant: Some("r"),
                        vowel: Some("iy"),
                    },
                    PhonemePair {
                        consonant: None,
                        vowel: Some("ey"),
                    },
                ])
            );
        }

        #[test]
        fn s_after_d_with_vowel() {
            let arpabet = ["d", "er", "s"];
            let arpabet = arpabet.as_slice();
            let phoneme_pairs = PhonemePairs::from(arpabet);
            assert_eq!(
                phoneme_pairs,
                PhonemePairs(vec![
                    PhonemePair {
                        consonant: Some("d"),
                        vowel: Some("er"),
                    },
                    PhonemePair {
                        consonant: Some("s"),
                        vowel: None,
                    },
                ])
            );
        }
    }
}
