use flite_sys::{cst_val, cst_val_consp, delete_val, delete_val_list, val_car, val_cdr};

pub mod lexicon;
pub mod lts;

pub struct Value(*mut cst_val);

pub struct Iter(*const cst_val);

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            if cst_val_consp(self.0) > 0 {
                delete_val_list(self.0);
            } else {
                delete_val(self.0);
            }
        }
    }
}

impl Value {
    pub const fn as_ptr(&self) -> *mut cst_val {
        self.0
    }

    pub const fn from_ptr(value: *mut cst_val) -> Value {
        Value(value)
    }

    pub const fn iter(&self) -> Iter {
        Iter(self.0.cast_const())
    }
}

impl Iterator for Iter {
    type Item = *const cst_val;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match self.0.as_ref() {
                Some(p) => {
                    self.0 = val_cdr(p);
                    Some(val_car(p))
                },
                None => None,
            }
        }
    }
}

impl IntoIterator for &'_ Value {
    type Item = *const cst_val;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
