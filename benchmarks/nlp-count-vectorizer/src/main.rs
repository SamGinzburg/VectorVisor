use vtext::tokenize::{VTextTokenizerParams,Tokenizer};
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct FuncInput {
    tweets: Vec<String>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    tokenized: Vec<Vec<String>>
}

fn tokenize(inputs: Vec<String>, tok: &Tokenizer) -> Vec<Vec<String>> {
    print!("");
    let mut results = vec![];
    for tweet in inputs {
        let mut str_vec: Vec<String> = vec![];
        for token in tok.tokenize(&tweet) {
            str_vec.push(token.to_string());
        }
        results.push(str_vec);
    }
    results
}

fn tokenize_inputs(event: FuncInput) -> FuncResponse {
    let tok = VTextTokenizerParams::default().lang("en").build().unwrap();
    FuncResponse { tokenized: tokenize(event.tweets, &tok) }
}

fn main() {
    let handler = WasmHandler::new(&tokenize_inputs);
    handler.run(1024*1024);
}
