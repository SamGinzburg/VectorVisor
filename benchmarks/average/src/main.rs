use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde_json::Value;
use serde_json::json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
struct FuncInput {
    numbers: Vec<f64>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    result: f64
}

// Take in a list of numbers and compute the average
fn average_json(event: FuncInput) -> Value {
    let mut acc = 0.0;
    for item in &event.numbers {
        acc += item;
    }
    let resp = FuncResponse { result: acc / event.numbers.len() as f64 };

    json!(resp)
}

fn main() {
    let handler = WasmHandler::new(&average_json);
    handler.run(1024*1024);
}
