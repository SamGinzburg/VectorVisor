extern crate base64;

use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
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
struct FuncInput<'a> {
    encoded_str: Cow<'a, str>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    encoded_resp: String
}

#[inline(never)]
fn compress_json(event: FuncInput) -> FuncResponse {
    let mut decoded_str = decode(event.encoded_str.as_bytes()).unwrap();
    let mut encoder = Encoder::new(BufWriter::new(Vec::new()));
    encoder.write(&decoded_str).unwrap();
    let (compressed_bytes, _) = encoder.finish();
    let encoded = encode(compressed_bytes.into_inner().unwrap());
    FuncResponse { encoded_resp: encoded }
}

fn main() {
    let handler = WasmHandler::new(&compress_json);
    handler.run(1024*1024);
}
