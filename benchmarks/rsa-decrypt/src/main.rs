use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde::Deserialize;
use serde::Serialize;
use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand_core::OsRng;
use base64::{encode, decode};
use lazy_static::lazy_static;

#[derive(Debug, Deserialize)]
struct FuncInput {
    encoded_str: String
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    encoded_resp: String
}

lazy_static! {
    static ref RSA_PKEY: RSAPrivateKey = RSAPrivateKey::new(&mut OsRng, 2048).expect("failed to generate a key");
}

fn rsa_decrypt(event: FuncInput) -> FuncResponse {
    let mut decoded_str = decode(event.encoded_str).unwrap();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = RSA_PKEY.decrypt(padding, &decoded_str).expect("failed to decrypt");
    FuncResponse { encoded_resp: encode(dec_data) }
}

fn main() {
    let handler = WasmHandler::new(&rsa_decrypt);
    handler.run(1024*1024);
}
