[![license](https://img.shields.io/github/license/RediSearch/redisearch-api-rs.svg)](https://github.com/RediSearch/redisearch-api-rs)
[![GitHub issues](https://img.shields.io/github/release/RediSearch/redisearch-api-rs.svg)](https://github.com/RediSearch/redisearch-api-rs/releases/latest)
[![CircleCI](https://circleci.com/gh/RediSearch/redisearch-api-rs/tree/master.svg?style=svg)](https://circleci.com/gh/RediSearch/redisearch-api-rs/tree/master)
[![macos](https://github.com/RediSearch/redisearch-api-rs/workflows/macos/badge.svg)](https://github.com/RediSearch/redisearch-api-rs/actions?query=workflow%3Amacos)
[![Forum](https://img.shields.io/badge/Forum-RediSearch-blue)](https://forum.redislabs.com/c/modules/redisearch/)
[![Discord](https://img.shields.io/discord/697882427875393627?style=flat-square)](https://discord.gg/xTbqgTB)

# redisearch-api-rs

## Rust API for RediSearch


## Building

Checkout submodules

    git submodule update --init
   
    cargo build

On macOS:

    brew install llvm

### Build example

    cargo build --example hello_redisearch
