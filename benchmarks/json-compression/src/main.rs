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

#[derive(Debug, Deserialize)]
struct FuncInput {
    encoded_str: String
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    encoded_resp: String
}

fn compress_json(event: FuncInput) -> FuncResponse {
    let mut decoded_str = decode(event.encoded_str).unwrap();
    let mut encoder = Encoder::new(BufWriter::new(Vec::new()));
    encoder.write(&decoded_str).unwrap();
    let (compressed_bytes, _) = encoder.finish();
    FuncResponse { encoded_resp: encode(compressed_bytes.into_inner().unwrap()) }
}

fn main() {
    let handler = WasmHandler::new(&compress_json);
    handler.run(1024*1024);
}
