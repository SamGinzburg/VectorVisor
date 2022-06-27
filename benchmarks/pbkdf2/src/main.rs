#[macro_use]
extern crate lazy_static;
use serde::Deserialize;
use serde::Serialize;

use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::MsgPack;
use rand_core::RngCore;

use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Salt},
    Pbkdf2,
    Params,
};
use rand_core::OsRng;
use rand_core::CryptoRng;

lazy_static! {
    static ref SALT: SaltString = SaltString::generate(&mut OsRng);
    static ref PBKDF2_PARAMS: pbkdf2::Params = Params { rounds: 20000, output_length: 32 }; 
}

#[inline(never)]
fn perform_hash(password: &[u8], salt: Salt) -> String {
    Pbkdf2.hash_password(password, None, None, *PBKDF2_PARAMS, salt).unwrap().to_string()
}

#[derive(Debug, Deserialize)]
struct FuncInput {
   password: String,
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    resp: Vec<u8>,
}

#[inline(never)]
pub fn generate(mut rng: impl CryptoRng + RngCore) -> SaltString {
    let mut bytes = [0u8; 10]; // 80 bits
    rng.fill_bytes(&mut bytes);
    SaltString::b64_encode(&bytes).unwrap()
}

#[inline(never)]
fn hash_input_password(event: FuncInput) -> FuncResponse {
    let salt_string = generate(&mut OsRng);
    let password_hash = perform_hash(event.password.clone().as_bytes(), salt_string.as_salt());
    FuncResponse { resp: password_hash.as_bytes().to_vec() }
}

fn main() {
    let handler = WasmHandler::new(&hash_input_password);
    handler.run_with_format(1024*1024, MsgPack);
}
