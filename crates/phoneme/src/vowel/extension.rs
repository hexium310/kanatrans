#[derive(Clone, Copy, Debug)]
pub enum Extension {
    DiphthongI,
    DiphthongU,
    LongVowel,
}

impl Extension {
    pub fn as_str(&self) -> &str {
        match self {
            Extension::DiphthongI => "イ",
            Extension::DiphthongU => "ウ",
            Extension::LongVowel => "ー",
        }
    }
}
