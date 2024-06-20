use std::{ffi::CStr, marker::PhantomData, str::Utf8Error};

use flite_sys::{cst_val, cst_val_consp, delete_val, delete_val_list, val_car, val_cdr, val_string};

pub mod lexicon;
pub mod lts;

pub struct Value(*mut cst_val);

pub struct Val<'a> {
    ptr: *const cst_val,
    _phantom: PhantomData<&'a ()>,
}

pub struct Iter<'a>(Val<'a>);

impl Drop for Value {
    fn drop(&mut self) {
        if self.0.is_null() {
            return;
        }
        unsafe {
            match cst_val_consp(self.0) {
                0 => delete_val(self.0),
                _ => delete_val_list(self.0),
            }
        }
    }
}

impl Value {
    pub const fn from_ptr(ptr: *mut cst_val) -> Value {
        Value(ptr)
    }

    pub const fn as_ptr(&self) -> *mut cst_val {
        self.0
    }

    pub const fn iter(&self) -> Iter<'_> {
        Iter(Val::from_ptr(self.0.cast_const()))
    }
}

impl Val<'_> {
    pub const fn from_ptr<'a>(ptr: *const cst_val) -> Val<'a> {
        Val { ptr, _phantom: PhantomData }
    }

    pub const fn as_ptr(&self) -> *const cst_val {
        self.ptr
    }

    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(val_string(self.ptr)).to_str() }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Val<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match self.0.as_ptr().as_ref() {
                Some(p) => {
                    self.0 = Val::from_ptr(val_cdr(p));
                    Some(Val::from_ptr(val_car(p)))
                },
                None => None,
            }
        }
    }
}

impl<'a> IntoIterator for &'a Value {
    type Item = Val<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
