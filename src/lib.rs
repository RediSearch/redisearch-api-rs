use std::os::raw::c_int;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate enum_primitive_derive;

use redis_module::raw as rawmod;

mod document;
mod index;
pub mod raw;

pub use document::Document;
pub use index::{Index, TagOptions};

bitflags! {
    pub struct FieldType: u32 {
        const FULLTEXT = raw::RSFLDTYPE_FULLTEXT;
        const NUMERIC = raw::RSFLDTYPE_NUMERIC;
        const GEO = raw::RSFLDTYPE_GEO;
        const TAG = raw::RSFLDTYPE_TAG;
    }
}

pub fn get_c_api_version() -> i32 {
    unsafe { raw::RediSearch_GetCApiVersion() }
}

pub extern "C" fn init(raw_ctx: *mut rawmod::RedisModuleCtx) -> c_int {
    let ctx = redis_module::Context::new(raw_ctx);
    ctx.log_debug("Initializing RediSearch...");

    let result = unsafe { raw::RediSearch_Init(raw_ctx, raw::REDISEARCH_INIT_LIBRARY as c_int) };

    if result == rawmod::REDISMODULE_OK as c_int {
        ctx.log_debug("RediSearch initialized successfully.");
    } else {
        ctx.log_debug("Failed initializing RediSearch.");
    }

    result
}
