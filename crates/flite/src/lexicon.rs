use std::{error::Error, ffi::CString, ptr::null};

use flite_sys::{cmu_lex_init, cst_lexicon, delete_lexicon, lex_lookup};
use once_cell::sync::Lazy;

use crate::{lts::Rules, Value};

pub struct Lexicon(*mut cst_lexicon);

pub static LEXICON: Lazy<Lexicon> = Lazy::new(|| unsafe { Lexicon::from_ptr(cmu_lex_init()) });

impl Drop for Lexicon {
    fn drop(&mut self) {
        unsafe {
            delete_lexicon(self.0);
        }
    }
}

impl Lexicon {
    pub const fn from_ptr(lexicon: *mut cst_lexicon) -> Self {
        Self(lexicon)
    }

    pub fn lookup(&self, word: &str, pos: Option<&str>) -> Result<Vec<String>, Box<dyn Error + Send + Sync + 'static>> {
        let mut lexs = vec![];

        let word = CString::new(word)?;
        let pos = pos.map(CString::new).transpose()?;

        unsafe {
            let word = word.as_ptr();
            let pos = pos.map(|p| p.as_ptr()).unwrap_or(null());

            let phones = Value::from_ptr(lex_lookup(self.0, word, pos, null()));
            for phone in &phones {
                let lex = phone.as_str()?;
                lexs.push(lex.to_string());
            }
        }

        Ok(lexs)
    }

    pub fn lts_rule_set(&self) -> Option<Rules> {
        unsafe { (*self.0).lts_rule_set.as_mut().map(|l| Rules::from_ptr(l)) }
    }
}

unsafe impl Send for Lexicon {}
unsafe impl Sync for Lexicon {}
