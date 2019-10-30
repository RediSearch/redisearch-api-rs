extern crate bindgen;

use cmake::Config;

fn main() {
    // Generate bindings for RediSearch

    let bindings = bindgen::Builder::default()
        .header("src/include/redisearch_api.h")
        //.clang_arg("-I src/include") // For redismodule.h
        .whitelist_var("(RS|RediSearch|REDISEARCH_|GC_POLICY).*")
        .whitelist_function("RediSearch.*")
        .blacklist_item("RedisModule.*")
        .raw_line("use redis_module::raw::{RedisModuleCtx, RedisModuleString};")
        .generate()
        .expect("error generating RediSearch bindings");

    bindings
        .write_to_file("src/raw/bindings.rs")
        .expect("failed to write RediSearch bindings to file");

    // Find and link statically to libredisearch.a
    // TODO: Consider splitting the low level libredisearch wrapper off into a separate `-sys` crate.

    let mut dst = Config::new("RediSearch")
        .define("RS_BUILD_STATIC", "ON")
        .build_target("redisearchS")
        .build();

    dst.push("build");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=redisearch");
}
