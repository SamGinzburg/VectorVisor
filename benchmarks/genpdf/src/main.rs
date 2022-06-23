#[macro_use]
extern crate lazy_static;

use lopdf::dictionary;
use lopdf::{Document, Object, Stream};
use lopdf::content::{Content, Operation};
use lopdf::xobject;
use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::MsgPack;
use serde::Deserialize;
use serde::Serialize;
use lopdf::Dictionary;
use lopdf::Object::Name;

lazy_static! {
    static ref EMBED_IMAGE: &'static [u8] = include_bytes!("test.jpg");
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
fn genpdf(event: FuncInput) -> FuncResponse {
    let name = event.name;
    let purchases: Vec<(&String, &f64)> = event.purchases.iter().zip(event.price.iter()).collect();

    let mut result: Vec<u8> = vec![];
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Courier",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        },
    });

    let mut pdf_ops = vec![
        Operation::new("BT", vec![]),
        Operation::new("Tf", vec!["F1".into(), 24.into()]),
        Operation::new("Td", vec![50.into(), 800.into()]),
        Operation::new("Tj", vec![Object::string_literal(format!("Fake Bill for: {}", name))]),
        Operation::new("ET", vec![]),
        Operation::new("BT", vec![]),
        Operation::new("Tf", vec!["F1".into(), 12.into()]),
        Operation::new("Td", vec![50.into(), 720.into()]),
        Operation::new("Tj", vec![Object::string_literal("-------------------------------------------------------------------")]),
        Operation::new("ET", vec![]),
        Operation::new("BT", vec![]),
        Operation::new("Tf", vec!["F1".into(), 12.into()]),
        Operation::new("Td", vec![50.into(), 700.into()]),
        Operation::new("Tj", vec![Object::string_literal("Purchases:")]),
        Operation::new("ET", vec![]),
    ];
    let mut purchase_ops: Vec<Operation> = vec![];
    let mut idx = 700 - 12;
    for (purchase, price) in purchases.iter() {
        purchase_ops.push(Operation::new("BT", vec![]));
        purchase_ops.push(Operation::new("Tf", vec!["F1".into(), 12.into()]));
        purchase_ops.push(Operation::new("Td", vec![50.into(), idx.into()]));
        purchase_ops.push(Operation::new("Tj", vec![Object::string_literal(format!("{}                                                        ${:.2}", purchase, price))]));
        purchase_ops.push(Operation::new("ET", vec![]),);
        idx -= 12;
    }
    pdf_ops.extend(purchase_ops);

    let mut image_ops: Vec<Operation> = vec![];

    // Add an image
    // The default "insert_image" API results in massive code bloat, adding extra compression routines for no reason

    let mut dict = Dictionary::new();
    dict.set("Type", Object::Name(b"XObject".to_vec()));
    dict.set("Subtype", Object::Name(b"Image".to_vec()));
    dict.set("Width", 814);
    dict.set("Height", 613);
    dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
    dict.set("BitsPerComponent", 8);
    // For JPG files
    dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));

    let img_stream = Stream::new(dict, EMBED_IMAGE.to_vec());

    let img_position = (100.0, 210.0);
    let img_size = (100.0+(814.0/3.0), 210.0+(613.0/3.0));
    let img_id = doc.add_object(img_stream);
    let img_name = format!("X{}", img_id.0);
    image_ops.push(Operation::new("q", vec![]));
    image_ops.push(Operation::new(
        "cm",
        vec![
            img_size.0.into(),
            0.into(),
            0.into(),
            img_size.1.into(),
            img_position.0.into(),
            img_position.1.into(),
        ],
    ));
    image_ops.push(Operation::new("Do", vec![Name(img_name.as_bytes().to_vec())]));
    image_ops.push(Operation::new("Q", vec![]));
    image_ops.push(Operation::new("Q", vec![]));

    pdf_ops.extend(image_ops);

    let content = Content {
        operations: pdf_ops,
    };

    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });

    doc.add_xobject(page_id, img_name.as_bytes(), img_id).unwrap();

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc.save_to(&mut result).unwrap();
    //doc.save("test.pdf").unwrap();
    //println!("{:?}", result);

    FuncResponse{ resp: result }
}

fn batch_genpdf(inputs: BatchInput) -> BatchFuncResponse {
    let mut results = vec![];
    for input in inputs.inputs {
        results.push(genpdf(input));
    }
    return BatchFuncResponse{ resp: results };
}

fn main() {

    let handler = WasmHandler::new(&batch_genpdf);
    handler.run_with_format(1024*512, MsgPack);

    /*
    genpdf(FuncInput{
        name: "test".to_string(),
        purchases: vec!["test".to_string()],
        price: vec![10.10],
    });
    */
}
