use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::MsgPack;
use serde_json::Value;
use serde_json::json;
use serde::Deserialize;
use serde::Serialize;
use base64::encode;
use base64::decode;
use std::io::Cursor;
use std::borrow::Cow;
use image::io::Reader as ImageReader;
use image::{GenericImageView, imageops};
use image::codecs::jpeg::JpegEncoder;
use image::EncodableLayout;
use image::load_from_memory_with_format;
use image::ImageFormat;
use image::ImageOutputFormat;
use image::ColorType;
use img_hash::{HasherConfig, HashAlg};

#[derive(Debug, Deserialize)]
struct FuncInput {
    image: String
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    hash: Vec<u8> 
}

#[inline(never)]
fn image_hash(event: FuncInput) -> FuncResponse {
    let mut image = decode(event.image.as_bytes()).unwrap();
    let mut decoded_image = load_from_memory_with_format(&image, ImageFormat::Jpeg).unwrap();
    let hasher = HasherConfig::new().to_hasher();
    let hash = hasher.hash_image(&decoded_image);
    
    FuncResponse { hash: hash.as_bytes().to_vec() }
}

fn main() {
    let handler = WasmHandler::new(&image_hash);
    handler.run_with_format(1024*512, MsgPack);
}
