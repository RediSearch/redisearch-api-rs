use std::ffi::{CStr, CString};
use std::ptr;

use num_traits::ToPrimitive;

use crate::raw::{self, RSFieldID, RSResultsIterator, GC_POLICY_FORK, GC_POLICY_NONE};
use crate::Document;
use redismodule::RedisError;
use std::os::raw::c_char;

pub struct Index {
    inner: *mut raw::RSIndex,
}

pub struct Field<'a> {
    index: &'a Index,
    field_id: RSFieldID,
}

macro_rules! debug {
    ($($arg:tt)*) => ({
        println!($($arg)*);
    })
}

impl Index {
    pub fn create(name: &str) -> Self {
        debug!("Creating index '{}'", name);

        let name = CString::new(name).unwrap();
        let index = unsafe { raw::RediSearch_CreateIndex(name.as_ptr(), ptr::null()) };
        Self { inner: index }
    }

    pub fn create_with_options(name: &str, options: &IndexOptions) -> Self {
        debug!("Creating index with options '{}'", name);
        let index_options =
            unsafe { raw::RediSearch_CreateIndexOptions().as_mut() }.expect("null IndexOptions");

        index_options.gcPolicy = options.gc_policy.to_i32().unwrap();

        let name = CString::new(name).unwrap();
        let index = unsafe { raw::RediSearch_CreateIndex(name.as_ptr(), index_options) };

        unsafe { raw::RediSearch_FreeIndexOptions(index_options) };

        Self { inner: index }
    }

    pub fn create_field(&self, name: &str) -> Field {
        debug!("Creating index field '{}'", name);
        let name = CString::new(name).unwrap();

        let ftype = raw::RSFLDTYPE_FULLTEXT;
        let fopt = raw::RSFLDOPT_NONE;

        let field_id =
            unsafe { raw::RediSearch_CreateField(self.inner, name.as_ptr(), ftype, fopt) };

        Field {
            index: self,
            field_id,
        }
    }

    pub fn add_document(&self, doc: &Document) -> Result<(), RedisError> {
        debug!("Adding document to index");
        let status = unsafe {
            raw::RediSearch_IndexAddDocument(
                self.inner,
                doc.inner,
                raw::REDISEARCH_ADD_REPLACE as i32,
                ptr::null_mut(), // Ignore errors, since not relevant in add/replace mode
            )
        };

        if status == redismodule::raw::REDISMODULE_ERR as i32 {
            Err(RedisError::Str("error adding document"))
        } else {
            Ok(())
        }
    }

    pub fn search(&self, query_string: &str) -> Result<ResultsIterator, RedisError> {
        /*
         * Return an iterator over the results of the specified query string.
         * @param[out] err: if not-NULL, will be set to the error message, if there is a
         *  problem parsing the query
         * @return an iterator over the results, or NULL if no iterator can be had
         *  (see err, or no results).
         */
        debug!("Querying: '{}'", query_string);

        let c_query = CString::new(query_string).unwrap();
        let mut err_buf = Vec::<u8>::with_capacity(1024);

        let results_iter = unsafe {
            raw::RediSearch_IterateQuery(
                self.inner,
                c_query.as_ptr(),
                query_string.len(),
                &mut (err_buf.as_mut_ptr() as *mut c_char),
            )
        };

        Ok(ResultsIterator::from_raw(results_iter, self, err_buf)?)
    }
}

pub struct ResultsIterator<'idx> {
    inner: *mut RSResultsIterator,
    index: &'idx Index,
    is_empty: bool,
}

impl<'idx> ResultsIterator<'idx> {
    fn from_raw(
        results_iter: *mut RSResultsIterator,
        index: &'idx Index,
        err_buf: Vec<u8>,
    ) -> Result<Self, RedisError> {
        let mut is_empty = false;

        if results_iter.is_null() {
            // Either we encountered an error, or there are no results.
            let err = String::from_utf8(err_buf)?;

            // TODO: This is quite ugly. There should be a nicer way to know if there was an error.
            if err.len() > 0 {
                return Err(err.into());
            }

            // No results.
            is_empty = true;
        }

        Ok(Self {
            inner: results_iter,
            index,
            is_empty,
        })
    }
}

impl Iterator for ResultsIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty {
            return None;
        }

        debug!("Getting next result");

        let mut len = 0usize;
        let key = unsafe {
            let raw_key =
                raw::RediSearch_ResultsIteratorNext(self.inner, self.index.inner, &mut len)
                    as *const c_char;

            if raw_key.is_null() {
                return None;
            }

            CStr::from_ptr(raw_key)
                .to_str()
                .expect("invalid UTF-8 data for key")
        };

        Some(key.to_owned())
    }
}

impl Drop for ResultsIterator<'_> {
    fn drop(&mut self) {
        debug!("Freeing results iterator");
        unsafe {
            raw::RediSearch_ResultsIteratorFree(self.inner);
        };
    }
}

// This hack is required since derive(Primitive) requires all values to have the same type,
// and some values are i32 while the rest are u32.
const GC_POLICY_NONE_ISIZE: isize = GC_POLICY_NONE as isize;
const GC_POLICY_FORK_ISIZE: isize = GC_POLICY_FORK as isize;

#[derive(Primitive, Debug, PartialEq)]
pub enum GcPolicy {
    None = GC_POLICY_NONE_ISIZE,
    Fork = GC_POLICY_FORK_ISIZE,
}

pub struct IndexOptions {
    gc_policy: GcPolicy,
}
