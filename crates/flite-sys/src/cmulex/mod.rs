use crate::{cst_lexicon, cst_utterance};

extern "C" {
    pub fn cmu_lex_init() -> *mut cst_lexicon;
    pub fn cmu_postlex(u: *mut cst_utterance) -> *mut cst_utterance;
}
