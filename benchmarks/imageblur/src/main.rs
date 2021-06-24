use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde_json::Value;
use serde_json::json;
use serde::Deserialize;
use serde::Serialize;
use base64::encode;
use base64::decode;
use std::io::Cursor;

use image::io::Reader as ImageReader;
use image::{GenericImageView, imageops};
use image::codecs::png::PngEncoder;
use image::EncodableLayout;
use image::load_from_memory_with_format;
use image::ImageFormat;
use image::ImageOutputFormat;
use image::ColorType;

#[derive(Debug, Deserialize)]
struct FuncInput {
    // Image comes in base64 encoded
    image: String
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    image: String
}

fn image_blur(event: FuncInput) -> FuncResponse {
    let mut image = decode(event.image).unwrap();
    let mut decoded_image = load_from_memory_with_format(&image, ImageFormat::Png).unwrap();

    let mut blurred = imageops::blur(&mut decoded_image, 4.0);

    let mut output_buf = vec![];
    let png_encoder = PngEncoder::new(&mut output_buf);

    let (nwidth, nheight) = blurred.dimensions();
    match png_encoder.encode(&mut blurred.as_bytes(), nwidth, nheight, ColorType::Rgba8) {
        Ok(_) => (),
        Err(err) => println!("Unable to encode image to PNG: {:?}", err),
    }

    FuncResponse { image: encode(output_buf) }
}

fn main() {
    let handler = WasmHandler::new(&image_blur);
    handler.run(1024*1024);
}
