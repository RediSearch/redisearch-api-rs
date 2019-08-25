extern crate bindgen;
extern crate cc;

use std::env;

fn main() {
    // Generate bindings for RediSearch

    let bindings = bindgen::Builder::default()
        .header("src/include/redisearch_api.h")
        //.clang_arg("-I src/include") // For redismodule.h
        .whitelist_var("(RS|RediSearch|REDISEARCH_|GC_POLICY).*")
        .whitelist_function("RediSearch.*")
        .blacklist_item("RedisModule.*")
        .raw_line("use redismodule::raw::{RedisModuleCtx, RedisModuleString};")
        .generate()
        .expect("error generating RediSearch bindings");

    bindings
        .write_to_file("src/raw/bindings.rs")
        .expect("failed to write RediSearch bindings to file");

    // Find and link statically to libredisearch.a

    // TODO: Instead of relying on a pre-built library,
    // we should build RediSearch as a static lib, like this:
    //
    //    mkdir -p build
    //    cd build
    //    cmake -DRS_BUILD_STATIC=ON ..
    //    make
    //
    // Take a look at some `-sys` crates for system library wrappers, and build using the
    // same methods that they use.
    // In fact, consider splitting this crate into a `-sys` crate for the low level wrappers
    // and another one for the high-level bindings.

    let lib_search_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", lib_search_dir);
    println!("cargo:rustc-link-lib=static=redisearch");
}
