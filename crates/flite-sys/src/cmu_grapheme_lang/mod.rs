use crate::{cst_cart, cst_phoneset, cst_voice};

extern "C" {
    pub fn cmu_grapheme_lang_init(v: *mut cst_voice);
    pub static cmu_grapheme_phoneset: cst_phoneset;
    pub static cmu_grapheme_phrasing_cart: cst_cart;
}
