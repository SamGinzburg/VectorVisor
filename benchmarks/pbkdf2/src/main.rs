#[macro_use]
extern crate lazy_static;

use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde_json::Value;
use serde_json::json;

use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Salt},
    Pbkdf2,
    Params,
};
use rand_core::OsRng;

lazy_static! {
    static ref SALT: SaltString = SaltString::generate(&mut OsRng);
    static ref PBKDF2_PARAMS: pbkdf2::Params = Params { rounds: 100100, output_length: 32 }; 
}

fn hash_input_password(event: Value) -> Value {
    let response = match event.get("password") {
        Some(Value::String(password)) => {
            let salt = Salt::new(&*SALT.as_ref()).unwrap();
            let password_hash = Pbkdf2.hash_password(password.as_bytes(), None, None, *PBKDF2_PARAMS, salt).unwrap().to_string();

            json!(password_hash)
        },
        _ => {
            json!(null)
        }
    };

    response
}

fn main() {
    let handler = WasmHandler::new(&hash_input_password);
    handler.run(1024*1024);
}
