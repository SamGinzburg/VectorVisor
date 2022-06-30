extern crate base64;

use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::MsgPack;
use serde_json::Value;
use serde_json::json;
use serde::Deserialize;
use serde::Serialize;
use base64::{encode, decode};
use compress::lz4::*;
use std::io::BufWriter;
use std::io::Write;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct FuncInput {
    tweets: Vec<String>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    encoded_resp: Vec<String>
}

#[inline(never)]
fn compress_input(data: Vec<u8>, mut encoder: Encoder<BufWriter<Vec<u8>>>) -> String {
    encoder.write(&data).unwrap();
    let (compressed_bytes, _) = encoder.finish();
    let encoded = encode(compressed_bytes.into_inner().unwrap());
    return encoded;
}

#[inline(never)]
fn compress_json(event: FuncInput) -> FuncResponse {
    let mut resp = vec![];
    for tweet in event.tweets {
        let mut encoder = Encoder::new(BufWriter::new(Vec::new()));
        let encoded = compress_input(tweet.as_bytes().to_vec(), encoder);
        resp.push(encoded);
    }
    FuncResponse { encoded_resp: resp }
}

fn main() {
    let handler = WasmHandler::new(&compress_json);
    handler.run_with_format(1024*1024, MsgPack);
}
