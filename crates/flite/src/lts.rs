use std::{error::Error, ffi::CString};

use flite_sys::{cst_lts_rules, lts_apply};

use crate::Value;

pub struct Rules(*mut cst_lts_rules);

impl Rules {
    pub const fn from_ptr(rules: *mut cst_lts_rules) -> Self {
        Self(rules)
    }

    pub fn apply(&self, word: &str, feats: &str) -> Result<Vec<String>, Box<dyn Error + Send + Sync + 'static>> {
        let mut lexs = vec![];

        let word = CString::new(word)?;
        let feats = CString::new(feats)?;

        unsafe {
            let word = word.as_ptr();
            let feats = feats.as_ptr();

            let phones = Value::from_ptr(lts_apply(word, feats, self.0));
            for phone in &phones {
                let lex = phone.as_str()?;
                lexs.push(lex.to_string());
            }
        }

        Ok(lexs)
    }
}
