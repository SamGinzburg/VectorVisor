use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::MsgPack;
use wasmedge_quickjs::{Context, Runtime, JsString, JsValue};
use std::mem::forget;
use serde::*;

static mut RUNTIME: Option<Runtime> = None;
static mut CONTEXT: Option<Context> = None;
static mut FUNC_OBJ: Option<&mut JsValue> = None;

#[derive(Debug, Deserialize)]
struct FuncInput {
    json_inputs: Vec<String>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    json_resp: Vec<String>
}


#[export_name = "wizer.initialize"]
pub unsafe extern "C" fn init() {
    let rt = Some(Runtime::new());
    let ctx = Some(Context::new_with_rt((rt.unwrap()).0));
let code = r#"
exports = {}

exports.handler = function handler(func_input) {
    try {
        var obj = JSON.parse(func_input);
        var result = 0;
        for (var count = 0; count < obj['values'].length; count++) {
            result += obj['values'][count];
        }
        return JSON.stringify(result / obj['values'].length); 
    } catch (error) {
        return JSON.stringify(0); 
    }
}

exports
"#;
    let func = ctx.unwrap().eval_global_str(code.to_string());
    RUNTIME = rt;
    CONTEXT = ctx;
    FUNC_OBJ = Some(Box::leak(Box::new(func)));
    forget(RUNTIME);
    forget(CONTEXT);
    forget(&FUNC_OBJ);
}

#[inline(never)]
fn average_json(event: FuncInput) -> FuncResponse {
    let mut resp: Vec<String> = vec![];
    for input in event.json_inputs { 
        let ret_val = unsafe { FUNC_OBJ.as_ref().unwrap().invoke("handler", &[JsValue::String(CONTEXT.unwrap().new_string(&input))]).unwrap() };
        resp.push(ret_val.to_string().unwrap().to_string());
    }
    FuncResponse { json_resp: resp }
}


fn main() {
        let handler = WasmHandler::new(&average_json);
        handler.run_with_format(1024*512, MsgPack);

        /*
        unsafe {
            println!("return value:{:?}", FUNC_OBJ.as_ref().unwrap().invoke("handler", &[JsValue::String(CONTEXT.unwrap().new_string(r#"{"name":"John", "age":30, "city":"New York"}"#))]));
        }
        */

        /*
        let mut rt = Runtime::new();
        rt.run_with_context(|ctx| {
            let code = r#"
exports = {}

exports.handler = function handler(func_input) {
    try {
        var obj = JSON.parse(func_input);
        console.log(obj.name)
    } catch (error) {
        console.log(error);
    }
}

exports
"#;
            let func = ctx.eval_global_str(code.to_string());
            println!("ret value: {:?}", func.invoke("handler", &[JsValue::String(ctx.new_string(r#"{"name":"John", "age":30, "city":"New York"}"#))]));
        });
        */
}
