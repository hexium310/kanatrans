#![allow(
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports
)]

#[cfg(not(feature = "vendored"))]
mod cmu_grapheme_lang;
#[cfg(not(feature = "vendored"))]
mod cmu_grapheme_lex;
#[cfg(not(feature = "vendored"))]
mod cmu_indic_lang;
#[cfg(not(feature = "vendored"))]
mod cmu_indic_lex;
#[cfg(not(feature = "vendored"))]
mod cmulex;
#[cfg(not(feature = "vendored"))]
mod usenglish;

#[cfg(not(feature = "vendored"))]
pub use cmu_grapheme_lang::*;
#[cfg(not(feature = "vendored"))]
pub use cmu_grapheme_lex::*;
#[cfg(not(feature = "vendored"))]
pub use cmu_indic_lang::*;
#[cfg(not(feature = "vendored"))]
pub use cmu_indic_lex::*;
#[cfg(not(feature = "vendored"))]
pub use cmulex::*;
#[cfg(not(feature = "vendored"))]
pub use usenglish::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
