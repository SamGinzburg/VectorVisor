use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde_json::Value;
use serde_json::json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FuncInput {
    numbers: Vec<f64>
}


// Take in a list of numbers and compute the average
fn average_json(event: FuncInput) -> Value {
    println!("unparsed event: {:?}", &event);
    json!(null)
    /*
    let response = match event.get("numbers") {
        Some(Value::Array(number_vec)) => {
            println!("number vec: {:?}", &number_vec);
            let vec: Vec<f64> = serde_json::from_value(Value::Array(number_vec.clone())).unwrap();
            // make sure its a vec of numbers
            println!("parsed vec: {:?}", &vec);
            let mut acc: f64 = 0.0;
            for item in &vec {
                println!("vec item: {}", &item);
                acc += item;
            }
            println!("acc: {}, vec.len(): {}", acc, &vec.len());
            json!(acc / vec.len() as f64)
        },
        _ => {
            // input is not a string we can compress!, no-op
            json!(null)
        }
    };
    */

    //response
}

fn main() {
    let handler = WasmHandler::new(&average_json);
    handler.run(1024*1024);
}
