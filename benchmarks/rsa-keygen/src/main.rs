use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde::Deserialize;
use serde::Serialize;
use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand_core::OsRng;
use base64::{encode, decode};
use lazy_static::lazy_static;
use rsa::PrivateKeyEncoding;

#[derive(Debug, Deserialize)]
struct FuncInput {
    encoded_str: String
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    encoded_resp: String
}

fn rsa_keygen(event: FuncInput) -> FuncResponse {
    let private_key = RSAPrivateKey::new(&mut OsRng, 2048).expect("failed to generate a key");
    FuncResponse { encoded_resp: encode(private_key.to_pkcs8().unwrap()) }
}

fn main() {
    let handler = WasmHandler::new(&rsa_keygen);
    handler.run(1024*1024);
}
