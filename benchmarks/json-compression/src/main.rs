extern crate base64;

use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde_json::Value;
use serde_json::json;

use compression::prelude::*;
use base64::encode;


fn compress_json(event: Value) -> Value {
    println!("{:?}", event);
    let response = match event.get("text") {
        Some(Value::String(str)) => {
            let compressed_str = str.as_bytes()
                                .into_iter()
                                .cloned()
                                .encode(&mut BZip2Encoder::new(9), Action::Finish)
                                .collect::<Result<Vec<_>, _>>()
                                .unwrap();
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
    handler.run();
}