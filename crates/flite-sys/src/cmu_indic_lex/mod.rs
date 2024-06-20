use crate::cst_lexicon;

extern "C" {
    pub fn cmu_indic_lex_init() -> *mut cst_lexicon;
    pub static mut indic_char: cmu_indic_char;
}

pub const cmu_indic_char_type_IND_INDEPENDENT_VOWEL: cmu_indic_char_type = 0;
pub const cmu_indic_char_type_IND_CONSONANT: cmu_indic_char_type = 1;
pub const cmu_indic_char_type_IND_VOWEL: cmu_indic_char_type = 2;
pub const cmu_indic_char_type_IND_ANUSWAAR: cmu_indic_char_type = 3;
pub const cmu_indic_char_type_IND_VISARGA: cmu_indic_char_type = 4;
pub const cmu_indic_char_type_IND_NUKTA: cmu_indic_char_type = 5;
pub const cmu_indic_char_type_IND_AVAGRAHA: cmu_indic_char_type = 6;
pub const cmu_indic_char_type_IND_HALANT: cmu_indic_char_type = 7;
pub const cmu_indic_char_type_IND_DIGIT: cmu_indic_char_type = 8;
pub const cmu_indic_char_type_IND_PUNC: cmu_indic_char_type = 9;
pub const cmu_indic_char_type_IND_IGNORE: cmu_indic_char_type = 10;
pub const cmu_indic_char_type_IND_ADDAK: cmu_indic_char_type = 11;

pub type cmu_indic_char_type = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cmu_indic_char {
    pub type_: cmu_indic_char_type,
    pub phoneme: [::std::os::raw::c_char; 12usize],
}
