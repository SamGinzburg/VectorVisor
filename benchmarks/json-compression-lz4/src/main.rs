extern crate base64;

use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde_json::Value;
use serde_json::json;


use base64::encode;
use lz4_flex::{compress};

fn compress_json(event: Value) -> Value {
    let response = match event.get("text") {
        Some(Value::String(str)) => {
            let compressed_str = compress(&str.as_bytes());
            json!(encode(compressed_str))
        },
        _ => {
            // input is not a string we can compress!, no-op
            json!(null)
        }
    };

    response
}

fn main() {
    let handler = WasmHandler::new(&compress_json);
    handler.run(1024*1024);
}
