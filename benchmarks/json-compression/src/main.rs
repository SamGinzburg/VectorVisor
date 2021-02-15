use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde_json::Value;
use serde_json::json;

fn compress_json(event: Value) -> Value {
    println!("{:?}", event);
    json!(null)
}

fn main() {
    let handler = WasmHandler::new(&compress_json);
    handler.run();
}