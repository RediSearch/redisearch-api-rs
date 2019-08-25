use std::os::raw::c_int;

#[macro_use]
extern crate enum_primitive_derive;

use redismodule::raw as rawmod;

mod document;
mod index;
pub mod raw;

pub use document::Document;
pub use index::Index;

pub fn get_c_api_version() -> i32 {
    unsafe { raw::RediSearch_GetCApiVersion() }
}

pub extern "C" fn init(raw_ctx: *mut rawmod::RedisModuleCtx) -> c_int {
    let ctx = redismodule::Context::new(raw_ctx);
    ctx.log_debug("Initializing RediSearch...");

    let result = unsafe { raw::RediSearch_Init(raw_ctx, raw::REDISEARCH_INIT_LIBRARY as c_int) };

    if result == rawmod::REDISMODULE_OK as c_int {
        ctx.log_debug("RediSearch initialized successfully.");
    } else {
        ctx.log_debug("Failed initializing RediSearch.");
    }

    result
}
