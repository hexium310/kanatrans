use crate::cst_lexicon;

unsafe extern "C" {
    pub fn cmu_grapheme_lex_init() -> *mut cst_lexicon;
    pub static num_unicode_sampa_mapping: ::std::os::raw::c_int;
    pub static mut unicode_sampa_mapping: [[*const ::std::os::raw::c_char; 5usize]; 16798usize];
}
