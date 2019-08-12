extern crate bindgen;
extern crate cc;

fn main() {
    //    cc::Build::new()
    //        .file("src/redisearch/redisearch_api.c")
    //        //.include("src/include/") // For redismodule.h
    //        .include("src/include/")
    //        .compile("redisearch_api");

    let bindings = bindgen::Builder::default()
        .header("src/include/redisearch_api.h")
        //.clang_arg("-I src/include") // For redismodule.h
        .whitelist_var("(RS|RediSearch).*")
        .generate()
        .expect("error generating RediSearch bindings");

    bindings
        .write_to_file("src/raw/bindings.rs")
        .expect("failed to write RediSearch bindings to file");
}
