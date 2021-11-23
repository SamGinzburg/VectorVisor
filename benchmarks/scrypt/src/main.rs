use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::MsgPack;
use serde::Deserialize;
use serde::Serialize;
use scrypt::scrypt;

use scrypt::{
    password_hash::{
        rand_core::OsRng,
        Decimal, Error, Ident, Output, PasswordHash, PasswordHasher, Result, Salt, SaltString
    },
    Scrypt,
    ALG_ID
};

#[derive(Debug, Deserialize)]
struct FuncInput {
    input_vec: Vec<String>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    hashed_results: Vec<String>
}

#[inline(never)]
fn hash_password(input: FuncInput) -> FuncResponse {
    // Litecoin scrypt parameters: N=1024 (this lib takes log2(N)), p = 1, r = 1
    let mut results = vec![];
    for val in input.input_vec {
        let params = scrypt::Params::new(10, 1, 1).unwrap();
        //let salt = Salt::new(&*SALT.as_ref()).unwrap();
        // Litecoin uses the same input bytes as the salt value
        // https://litecoin.info/index.php/Scrypt
        //let salt = Salt::new(&val).unwrap();
        let mut output = [0u8; 32];
        //let hash = Scrypt.hash_password_customized(val.as_bytes(), Some(ALG_ID), None, params, salt).unwrap();
        scrypt(val.as_bytes(), val.as_bytes(), &params, &mut output).unwrap();
        let output_fmt = Output::new(&output).unwrap();
        results.push(output_fmt.to_string());
    }
    FuncResponse { hashed_results: results }
}

fn main() {
    let handler = WasmHandler::new(&hash_password);
    handler.run_with_format(1024*512, MsgPack);
}
