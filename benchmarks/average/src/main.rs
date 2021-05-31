use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde_json::Value;
use serde_json::json;

// Take in a list of numbers and compute the average
fn average_json(event: Value) -> Value {
    let response = match event.get("numbers") {
        Some(Value::Array(number_vec)) => {
            let vec: Vec<f64> = serde_json::from_value(Value::Array(number_vec.clone())).unwrap();
            // make sure its a vec of numbers
            let mut acc: f64 = 0.0;
            for item in vec {
                acc += item;
            }
            json!(acc / number_vec.len() as f64)
        },
        _ => {
            // input is not a string we can compress!, no-op
            json!(null)
        }
    };

    response
}

fn main() {
    let handler = WasmHandler::new(&average_json);
    handler.run(1024*1024);
}