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
use image::codecs::bmp::BmpEncoder;
use image::EncodableLayout;
use image::load_from_memory_with_format;
use image::ImageFormat;
use image::ImageOutputFormat;
use image::ColorType;
use image::ImageBuffer;
use image::Rgba;

#[derive(Debug, Deserialize)]
struct FuncInput<'a> {
    // Image comes in base64 encoded
    image: Cow<'a, str>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    image: String
}

#[inline(never)]
fn inline_test(image: image::DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    return imageops::blur(&image, 4.0);
}

#[inline(never)]
fn image_blur(event: FuncInput) -> FuncResponse {
    let mut image = decode(event.image.as_bytes()).unwrap();
    let mut decoded_image = load_from_memory_with_format(&image, ImageFormat::Bmp).unwrap();
    let mut blurred = inline_test(decoded_image);

    let mut output_buf = vec![];
    let mut bmp_encoder = BmpEncoder::new(&mut output_buf);

    let (nwidth, nheight) = blurred.dimensions();
    match bmp_encoder.encode(&mut blurred.as_bytes(), nwidth, nheight, ColorType::Rgba8) {
        Ok(_) => (),
        Err(_) => (),
    }
    FuncResponse { image: encode(output_buf) }
}

fn main() {
    let handler = WasmHandler::new(&image_blur);
    handler.run_with_format(1024*512, MsgPack);
}
