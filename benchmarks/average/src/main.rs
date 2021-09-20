use wasm_serverless_invoke::*;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::{MsgPack, Json};
use serde_json::Value;
use serde_json::json;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct FuncInput {
    numbers: Vec<f64>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    result: f64
}

// Take in a list of numbers and compute the average
fn average_json(event: FuncInput) -> FuncResponse {
    let mut acc = 0.0;
    for item in &event.numbers {
        acc += item;
    }
    FuncResponse { result: acc / event.numbers.len() as f64 }
}

fn main() {
    let handler = WasmHandler::new(&average_json);
    handler.run_with_format(1024*1024, MsgPack);
}
