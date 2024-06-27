use super::extension::Extension;

pub const VOWEL_BASE: [&str; 5] = ["ア", "イ", "ウ", "エ", "オ"];

pub static VOWEL_PATTERNS: [(&str, VowelPattern); 16] = [
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
];

#[derive(Clone, Copy, Debug)]
pub struct VowelPattern {
    pub position: usize,
    pub extension: Option<Extension>,
}
