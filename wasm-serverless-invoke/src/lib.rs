pub mod wasm_handler;

#[cfg(test)]
mod tests {
    use crate::wasm_handler::WasmHandler;
    use serde_json::Value;
    use serde_json::json;

    fn test_serverless_function(event: Value) -> Value {
        println!("{:?}", event);
        json!(null)
    }

    #[test]
    fn test_serverless_harness() {
        // this is really just testing that the types match properly
        let handler = WasmHandler::new(&test_serverless_function);
    }
}
