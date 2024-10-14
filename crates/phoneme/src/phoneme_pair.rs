use std::{borrow::Cow, ops::Deref};

use anyhow::Result;

use crate::{
    assembler::Assembler,
    consonant::{consonants, ConsonantPattern},
    error::AssemblerError,
    phoneme::Phoneme,
    vowel::vowel::vowels,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhonemePair<'a> {
    Both(Phoneme<'a>, Phoneme<'a>),
    Consonant(Phoneme<'a>),
    Vowel(Phoneme<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhonemePairs<'a>(Vec<PhonemePair<'a>>);

impl<'a> Deref for PhonemePairs<'a> {
    type Target = Vec<PhonemePair<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<&'a [&str]> for PhonemePairs<'a> {
    fn from(value: &'a [&str]) -> Self {
        let phoneme_pairs = value
            .iter()
            .enumerate()
            .fold(vec![], |mut accumulator: Vec<PhonemePair>, (index, phoneme)| {
                let previous_pair = accumulator.last();
                let phoneme = Phoneme::from(phoneme.trim_matches(char::is_numeric));

                let previous_pair = match previous_pair {
                    Some(previous_pair) => previous_pair,
                    None => return vec![PhonemePair::from(phoneme)],
                };

                let phoneme_pair = match (phoneme, *previous_pair) {
                    (Phoneme::Consonant(consonant), PhonemePair::Consonant(previous_consonant)) => {
                        let consonant_cluster = consonants()
                            .get_cluster(&previous_consonant, consonant)
                            .map(|cluster| Phoneme::from(cluster.cluster));

                        match consonant_cluster {
                            Some(consonant_cluster) => {
                                accumulator.truncate(accumulator.len() - 1);

                                PhonemePair::Consonant(consonant_cluster)
                            },
                            None => PhonemePair::Consonant(phoneme),
                        }
                    },
                    (Phoneme::Consonant("r"), _) if index + 1 == value.len() => PhonemePair::Vowel(Phoneme::Vowel("er")),
                    (Phoneme::Consonant(_), _) => PhonemePair::Consonant(phoneme),
                    (Phoneme::Vowel(_), PhonemePair::Consonant(previous_consonant)) => {
                        let phoneme_pair = PhonemePair::Both(previous_consonant, phoneme);

                        accumulator.truncate(accumulator.len() - 1);

                        phoneme_pair
                    },
                    (Phoneme::Vowel(_), PhonemePair::Both(previous_consonant, Phoneme::Vowel("er"))) => {
                        let phoneme_pair = PhonemePair::Both(previous_consonant, Phoneme::Vowel("eh"));

                        accumulator.truncate(accumulator.len() - 1);
                        accumulator.push(phoneme_pair);

                        PhonemePair::Both(Phoneme::Consonant("r"), phoneme)
                    },
                    (Phoneme::Vowel(_), PhonemePair::Vowel(Phoneme::Vowel("er"))) => {
                        accumulator.truncate(accumulator.len() - 1);
                        accumulator.push(PhonemePair::Vowel(Phoneme::Vowel("aa")));

                        PhonemePair::Both(Phoneme::Consonant("r"), phoneme)
                    },
                    (Phoneme::Vowel(_), _) => PhonemePair::Vowel(phoneme),
                    (Phoneme::Unexpected(unexpected), _) => {
                        tracing::warn!("unexpected {unexpected} appeared in {value:?}, parsing skipped");

                        return accumulator;
                    },
                };

                accumulator.push(phoneme_pair);
                accumulator
            });

        Self(phoneme_pairs)
    }
}

impl<'a> From<Phoneme<'a>> for PhonemePair<'a> {
    fn from(value: Phoneme<'a>) -> Self {
        match value {
            Phoneme::Consonant(_) => Self::Consonant(value),
            Phoneme::Vowel(_) => Self::Vowel(value),
            Phoneme::Unexpected(_) => Self::Consonant(value),
        }
    }
}

impl<'a> Assembler for PhonemePair<'a> {
    fn assemble(&self) -> Result<Cow<'_, str>, AssemblerError> {
        match *self {
            Self::Both(consonant, vowel) => self.assemble_both(consonant, vowel),
            Self::Consonant(consonant) => self.assemble_consonant(consonant),
            Self::Vowel(vowel) => self.assemble_vowel(vowel, vowels().base()),
        }
    }

    fn assemble_both(&self, consonant: Phoneme<'_>, vowel: Phoneme<'_>) -> Result<Cow<'_, str>, AssemblerError> {
        let Some(ConsonantPattern { with_vowel, .. }) = consonants().get(*consonant) else {
            return Err(AssemblerError::UnexpectedConsonant(consonant.to_string()));
        };

        self.assemble_vowel(vowel, with_vowel)
    }

    fn assemble_consonant(&self, consonant: Phoneme<'_>) -> Result<Cow<'_, str>, AssemblerError> {
        let Some(ConsonantPattern { unit, .. }) = consonants().get(*consonant) else {
            return Err(AssemblerError::UnexpectedConsonant(consonant.to_string()));
        };

        Ok(Cow::Borrowed(unit))
    }

    fn assemble_vowel(&self, vowel: Phoneme<'_>, list: &[&str]) -> Result<Cow<'_, str>, AssemblerError> {
        let Some(vowel_pattern) = vowels().get(*vowel) else {
            return Err(AssemblerError::UnexpectedVowel(vowel.to_string()));
        };

        let base = list[vowel_pattern.position];
        let extension = vowel_pattern.extension.as_ref().map_or("", |v| v.as_str());
        let katakana = [base, extension].join("");

        Ok(Cow::Owned(katakana))
    }
}

impl<'a> PhonemePair<'a> {
    pub fn consonant(self) -> Option<Phoneme<'a>> {
        match self {
            Self::Consonant(consonant) | Self::Both(consonant, _) => Some(consonant),
            _ => None,
        }
    }

    pub fn vowel(self) -> Option<Phoneme<'a>> {
        match self {
            Self::Vowel(vowel) | Self::Both(_, vowel) => Some(vowel),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        phoneme::Phoneme,
        phoneme_pair::{PhonemePair, PhonemePairs},
    };

    #[test]
    fn phoneme_pairs_from() {
        let arpabet = ["th", "r", "eh1", "sh", "ow1", "l", "d"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![
                PhonemePair::Consonant(Phoneme::Consonant("th")),
                PhonemePair::Both(Phoneme::Consonant("r"), Phoneme::Vowel("eh")),
                PhonemePair::Both(Phoneme::Consonant("sh"), Phoneme::Vowel("ow")),
                PhonemePair::Consonant(Phoneme::Consonant("l")),
                PhonemePair::Consonant(Phoneme::Consonant("d")),
            ])
        );
    }

    #[test]
    fn ts() {
        let arpabet = ["t", "s"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("ts"))])
        );
    }

    #[test]
    fn tz() {
        let arpabet = ["t", "z"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("tz"))])
        );
    }

    #[test]
    fn ds() {
        let arpabet = ["d", "s"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("ds"))])
        );
    }

    #[test]
    fn dz() {
        let arpabet = ["d", "z"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("dz"))])
        );
    }

    #[test]
    fn mb() {
        let arpabet = ["m", "b"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("mb"))])
        );
    }

    #[test]
    fn mm() {
        let arpabet = ["m", "m"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("mm"))])
        );
    }

    #[test]
    fn mp() {
        let arpabet = ["m", "p"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("mp"))])
        );
    }

    #[test]
    fn by() {
        let arpabet = ["b", "y"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("by"))])
        );
    }

    #[test]
    fn fy() {
        let arpabet = ["f", "y"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("fy"))])
        );
    }

    #[test]
    fn gy() {
        let arpabet = ["g", "y"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("gy"))])
        );
    }

    #[test]
    fn hhy() {
        let arpabet = ["hh", "y"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("hhy"))])
        );
    }

    #[test]
    fn ky() {
        let arpabet = ["k", "y"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("ky"))])
        );
    }

    #[test]
    fn ly() {
        let arpabet = ["l", "y"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("ly"))])
        );
    }

    #[test]
    fn my() {
        let arpabet = ["m", "y"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("my"))])
        );
    }

    #[test]
    fn ny() {
        let arpabet = ["n", "y"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("ny"))])
        );
    }

    #[test]
    fn py() {
        let arpabet = ["p", "y"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![PhonemePair::Consonant(Phoneme::Consonant("py"))])
        );
    }

    #[test]
    fn two_or_more_vowels() {
        let arpabet = ["iy0", "ey1"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![
                PhonemePair::Vowel(Phoneme::Vowel("iy")),
                PhonemePair::Vowel(Phoneme::Vowel("ey")),
            ])
        );
    }

    #[test]
    fn two_or_more_vowels_after_consonant() {
        let arpabet = ["r", "iy0", "ey1"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![
                PhonemePair::Both(Phoneme::Consonant("r"), Phoneme::Vowel("iy")),
                PhonemePair::Vowel(Phoneme::Vowel("ey")),
            ])
        );
    }

    #[test]
    fn s_after_d_with_vowel() {
        let arpabet = ["d", "er", "s"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![
                PhonemePair::Both(Phoneme::Consonant("d"), Phoneme::Vowel("er")),
                PhonemePair::Consonant(Phoneme::Consonant("s")),
            ])
        );
    }

    #[test]
    fn er_before_vowel_and_after_consonant() {
        let arpabet = ["n", "er", "ax"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![
                PhonemePair::Both(Phoneme::Consonant("n"), Phoneme::Vowel("eh")),
                PhonemePair::Both(Phoneme::Consonant("r"), Phoneme::Vowel("ax")),
            ])
        );
    }


    #[test]
    fn heading_er_with_vowel() {
        let arpabet = ["er", "ax"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![
                PhonemePair::Vowel(Phoneme::Vowel("aa")),
                PhonemePair::Both(Phoneme::Consonant("r"), Phoneme::Vowel("ax")),
            ])
        );
    }

    #[test]
    fn trailing_r() {
        let arpabet = ["s", "f", "ih1", "r"].as_slice();
        let phoneme_pairs = PhonemePairs::from(arpabet);
        assert_eq!(
            phoneme_pairs,
            PhonemePairs(vec![
                PhonemePair::Consonant(Phoneme::Consonant("s")),
                PhonemePair::Both(Phoneme::Consonant("f"), Phoneme::Vowel("ih")),
                PhonemePair::Vowel(Phoneme::Vowel("er")),
            ])
        );
    }
}
