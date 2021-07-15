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

fn tokenize_inputs(event: FuncInput) -> FuncResponse {
    let mut results = vec![];
    let tok = VTextTokenizerParams::default().lang("en").build().unwrap();
    for tweet in event.tweets {
        let mut vec = vec![];
        let mut str_vec: Vec<String> = vec![];
        for token in tok.tokenize(&tweet) {
            vec.push(token);
        }
        for s in vec {
            str_vec.push(String::from(s));
        }
        results.push(str_vec);
    }
    FuncResponse { tokenized: results }
}

fn main() {
    let handler = WasmHandler::new(&tokenize_inputs);
    handler.run(1024*1024);
}
