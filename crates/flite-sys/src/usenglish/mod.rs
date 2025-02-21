use crate::{cst_cart, cst_phoneset, cst_utterance, cst_voice, dur_stat};

unsafe extern "C" {
    pub fn usenglish_init(v: *mut cst_voice);
    pub fn us_f0_model(u: *mut cst_utterance) -> *mut cst_utterance;
    pub static us_phoneset: cst_phoneset;
    pub static us_phrasing_cart: cst_cart;
    pub static us_int_accent_cart: cst_cart;
    pub static us_int_tone_cart: cst_cart;
    pub static us_durz_cart: cst_cart;
    pub static us_pos_cart: cst_cart;
    pub static us_dur_stats: [*const dur_stat; 0usize];
}
