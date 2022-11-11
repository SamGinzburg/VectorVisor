#[macro_use]
extern crate lazy_static;

use pdf_writer::*;
use pdf_writer::types::{ActionType, AnnotationType, BorderType};
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use wasm_serverless_invoke::wasm_handler::*;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::MsgPack;
use serde::Deserialize;
use serde::Serialize;
use image::{ColorType, GenericImageView, ImageFormat};
use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};

lazy_static! {
    static ref EMBED_IMAGE: &'static [u8] = include_bytes!("test.png");
}

#[derive(Debug, Deserialize)]
struct FuncInput {
    name: String,
    purchases: Vec<String>,
    price: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct BatchInput {
    inputs: Vec<FuncInput>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    resp: Vec<u8>
}

#[derive(Debug, Serialize)]
struct BatchFuncResponse {
    resp: Vec<FuncResponse>
}

#[inline(never)]
fn makePdf(event: FuncInput) -> Vec<u8> {
    let name = event.name;
    let purchases: Vec<(&String, &f64)> = event.purchases.iter().zip(event.price.iter()).collect();

    // Start writing.
    let mut writer = PdfWriter::new();

    // Define some indirect reference ids we'll use.
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let font_id = Ref::new(4);
    let content_id = Ref::new(5);
    let image_id = Ref::new(6);
    let s_mask_id = Ref::new(7);

    let image_name = Name(b"Im1");
    let font_name = Name(b"F1");

    // Write the document catalog with a reference to the page tree.
    writer.catalog(catalog_id).pages(page_tree_id);

    // Write the page tree with a single child page.
    writer.pages(page_tree_id).kids([page_id]).count(1);

    // Write a page.
    let mut page = writer.page(page_id);

    // Set the size to A4 (measured in points) using `media_box` and set the
    // text object we'll write later as the page's contents.
    let a4 = Rect::new(0.0, 0.0, 595.0, 842.0);
    page.media_box(a4);
    page.parent(page_tree_id);
    page.contents(content_id);

    // We also create the annotations list here that allows us to have things
    // like links or comments on the page.
    //let mut annotations = page.annotations();
    //let mut annotation = annotations.push();

    // Write the type, area, alt-text, and color for our link annotation.
    //annotation.subtype(AnnotationType::Link);
    //annotation.rect(Rect::new(215.0, 730.0, 251.0, 748.0));
    //annotation.contents(TextStr("Link to the Rust project web page"));
    //annotation.color_rgb(0.0, 0.0, 1.0);
    // Write an action for the annotation, telling it where to link to. Actions
    // can be associated with annotations, outline objects, and more and allow
    // creating interactive PDFs (open links, play sounds...).
    //annotation
    //    .action()
    //    .action_type(ActionType::Uri)
    //    .uri(Str(b"https://www.rust-lang.org/"));

    // Set border and style for the link annotation.
    //annotation.border_style().width(2.0).style(BorderType::Underline);

    // We have to finish all the writers that depend on the page here because
    // otherwise they would be mutably borrowed until the end of the block.
    // Finishing is handled through the writer's `Drop` implementations, so that
    // you cannot accidentally forget it. The `finish()` method from the `Finish`
    // trait is just a postfix-style version of dropping.
    //annotation.finish();
    //annotations.finish();

    // We also need to specify which resources the page needs, which in our case
    // is only a font that we name "F1" (the specific name doesn't matter).
    page.resources().fonts().pair(font_name, font_id);
    page.resources().x_objects().pair(image_name, image_id);

    page.finish();

    // Specify the font we want to use. Because Helvetica is one of the 14 base
    // fonts shipped with every PDF reader, we don't have to embed any font
    // data.
    writer.type1_font(font_id).base_font(Name(b"Helvetica"));

    // Write a line of text, with the font specified in the resource list
    // before, at a font size of 14.0, starting at coordinates (108.0, 734.0)
    // measured from the bottom left of the page.
    //
    // Because we haven't specified any encoding when writing the Type 1 font,
    // the standard encoding is used which happens to work with most ASCII
    // characters.
    let mut content = Content::new();

    content.begin_text();
    content.set_font(font_name, 14.0);
    content.next_line(50.0, 800.0);
    content.show(Str(format!("Fake Bill for: {}", name).as_bytes()));
    content.end_text();

    content.begin_text();
    content.set_font(font_name, 14.0);
    content.next_line(50.0, 770.0);
    content.show(Str(b"-------------------------------------------------------------------"));
    content.end_text();
    
    content.begin_text();
    content.set_font(font_name, 14.0);
    content.next_line(50.0, 755.0);
    content.show(Str(b"Purchases"));
    content.end_text();
    let mut idx = 735.0;

    /*
    for (purchase, price) in purchases.iter() {
        content.begin_text();
        content.set_font(font_name, 14.0);
        content.next_line(50.0, idx);
        content.show(Str(format!("{}                                                        ${:.2}", purchase, price).as_bytes()));
        content.end_text();
        idx -= 15.0;
    }
    */

    let dynamic = image::load_from_memory(&EMBED_IMAGE).unwrap();

    //let encoded = &EMBED_IMAGE;

    let level = CompressionLevel::UberCompression as u8;
    let encoded = compress_to_vec_zlib(dynamic.to_rgb8().as_raw(), level);

    // If there's an alpha channel, extract the pixel alpha values.
    let mask = dynamic.color().has_alpha().then(|| {
        let alphas: Vec<_> = dynamic.pixels().map(|p| (p.2).0[3]).collect();
        compress_to_vec_zlib(&alphas, level)
    });
    let filter = Filter::FlateDecode;

    let mut image = writer.image_xobject(image_id, &encoded);
    image.filter(filter);
    image.width(dynamic.width() as i32);
    image.height(dynamic.height() as i32);
    image.color_space().device_rgb();
    image.bits_per_component(8);
    if mask.is_some() {
        image.s_mask(s_mask_id);
    }
    image.finish();

    if let Some(encoded) = &mask {
        let mut s_mask = writer.image_xobject(s_mask_id, &encoded);
        s_mask.filter(filter);
        s_mask.width(dynamic.width() as i32);
        s_mask.height(dynamic.height() as i32);
        s_mask.color_space().device_gray();
        s_mask.bits_per_component(8);
    }

    // Size the image at 1pt per pixel.
    let w = dynamic.width() as f32;
    let h = dynamic.height() as f32;

    // Center the image on the page.
    let x = (a4.x2 - w) / 2.0;
    let y = 50.0; //(a4.y2 - h) / 2.0;

    content.save_state();
    content.transform([w as f32, 0.0, 0.0, h as f32, x, y]);
    content.x_object(image_name);
    content.restore_state();

    // Now add the price data to avoid diverging before image processing
    for (purchase, price) in purchases.iter() {
        content.begin_text();
        content.set_font(font_name, 14.0);
        content.next_line(50.0, idx);
        content.show(Str(format!("{}                                                        ${:.2}", purchase, price).as_bytes()));
        content.end_text();
        idx -= 15.0;
    }

    writer.stream(content_id, &content.finish());

    // Finish writing (this automatically creates the cross-reference table and
    // file trailer) and retrieve the resulting byte buffer.
    return writer.finish();
}

fn batch_genpdf(inputs: BatchInput) -> BatchFuncResponse {
    let mut results = vec![];
    for input in inputs.inputs {
        //makePdf(input);
        results.push(FuncResponse { resp: makePdf(input) });

        unsafe { vectorvisor_barrier() }; 
    }
    return BatchFuncResponse{ resp: results };
}

fn main() {
    let handler = WasmHandler::new(&batch_genpdf);
    handler.run_with_format(1024*512, MsgPack);
}

/*
#[inline(never)]
fn main() {
    let mut buf2 = vec![];
    let now = Instant::now();
    for _idx in 0..1 {
        buf2.extend(makePdf(FuncInput{
            name: "test".to_string(),
            purchases: vec!["test".to_string(), "test1".to_string(), "test2".to_string()],
            price: vec![10.10, 123.32, 100.00],
        }));
    }

    let elapsed = now.elapsed().as_nanos();
    println!("Elapsed: {:.2?}", elapsed);


    let mut file = File::create("test.pdf").unwrap();
    file.write_all(&buf2).unwrap();
        
    //println!("{:?}", buf);
}
*/
