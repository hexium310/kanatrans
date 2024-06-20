use crate::{cst_cart, cst_phoneset, cst_voice};

extern "C" {
    pub fn cmu_indic_lang_init(v: *mut cst_voice);
    pub static cmu_indic_phoneset: cst_phoneset;
    pub static cmu_indic_phrasing_cart: cst_cart;
}
