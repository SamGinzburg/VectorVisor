use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::MsgPack;
use serde::Deserialize;
use serde::Serialize;
use scrypt::scrypt;
use lazy_static::lazy_static;

use scrypt::{
    password_hash::{
        rand_core::OsRng,
        Decimal, Error, Ident, Output, PasswordHash, PasswordHasher, Result, Salt, SaltString
    },
    Scrypt,
    ALG_ID
};

lazy_static! {
    static ref SALT: SaltString = SaltString::generate(&mut OsRng);
}

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
        let salt = Salt::new(&*SALT.as_ref()).unwrap();
        let hash = Scrypt.hash_password_customized(val.as_bytes(), Some(ALG_ID), None, params, salt).unwrap();
        results.push(hash.to_string());
    }
    FuncResponse { hashed_results: results }
}

fn main() {
    let handler = WasmHandler::new(&hash_password);
    handler.run_with_format(1024*512, MsgPack);
}
