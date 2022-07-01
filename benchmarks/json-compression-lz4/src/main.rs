use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::MsgPack;
use serde::Deserialize;
use serde::Serialize;
use lz4_flex::{compress};

#[derive(Debug, Deserialize)]
struct FuncInput {
    tweets: Vec<String>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    encoded_resp: Vec<Vec<u8>>
}

#[inline(never)]
fn compress_msgpack(event: FuncInput) -> FuncResponse {
    let mut resp = vec![];
    for tweet in event.tweets {
        let compressed_str = compress(&tweet.as_bytes());
        resp.push(compressed_str);
    }
    FuncResponse { encoded_resp: resp }
}

fn main() {
    let handler = WasmHandler::new(&compress_msgpack);
    handler.run_with_format(1024*512, MsgPack);
}
