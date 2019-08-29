# redisearch-api-rs

## Rust API for RediSearch

TODO:

- Use it from RedisDoc to index JSON docs

## Building

```
git clone https://github.com/RedisLabsModules/redismodule-rs.git
git clone https://github.com/RediSearch/redisearch-api-rs.git
cd redisearch-api-rs
cargo build
```

On macOS:
- `brew install llvm`

* Make sure you have `libredisearch.a` built. This will be done automatically in the future.
For more details see: https://github.com/RediSearch/RediSearch

### Build example

`cargo build --example hello_redisearch`
