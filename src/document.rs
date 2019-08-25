use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use crate::raw;

pub struct Document {
    pub(crate) inner: *mut raw::RSDoc,
}

impl Document {
    pub fn create(key: &str, score: f64) -> Self {
        let c_key = CString::new(key).unwrap();
        let lang = ptr::null(); // Default language

        let doc = unsafe {
            raw::RediSearch_CreateDocument(c_key.as_ptr() as *const c_void, key.len(), score, lang)
        };

        Self { inner: doc }
    }

    pub fn add_field(&self, name: &str, value: &str) {
        let name = CString::new(name).unwrap();
        let c_value = CString::new(value).unwrap();
        unsafe {
            raw::RediSearch_DocumentAddFieldString(
                self.inner,
                name.as_ptr(),
                c_value.as_ptr(),
                value.len(),
                raw::RSFLDTYPE_FULLTEXT,
            );
        }
    }
}
