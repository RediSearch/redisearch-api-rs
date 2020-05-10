#[macro_use]
extern crate redis_module;

use redis_module::{Context, NextArg, RedisError, RedisResult};
use redisearch_api::{self, init, Document, FieldType, Index};

fn hello_redisearch(_: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let search_term = args.next_string()?;

    // FT.CREATE my_index
    //     SCHEMA
    //         $a.b.c TEXT WEIGHT 5.0
    //         $a.b.d TEXT
    //         $b.a TEXT

    // see RediSearch: t_llapi.cpp

    let index_name = "index";
    let field_name = "foo";
    let score = 1.0;

    let index = Index::create(index_name);
    index.create_field(field_name, 1.0, None);

    let doc = Document::create("doc1", score);
    doc.add_field(field_name, "bar", FieldType::FULLTEXT);
    index.add_document(&doc)?;

    let doc2 = Document::create("doc2", score);
    doc2.add_field(field_name, "quux", FieldType::FULLTEXT);
    index.add_document(&doc2)?;

    let keys: Vec<_> = index.search(search_term.as_str())?.collect();

    Ok(keys.into())
}

redis_module! {
    name: "hello_redisearch",
    version: 1,
    data_types: [],
    init: init,
    commands: [
        ["hello_redisearch", hello_redisearch, ""],
    ],
}
