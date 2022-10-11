use wasmedge_quickjs::{Context, Runtime, JsString, JsValue};
use std::mem::forget;

static mut RUNTIME: Option<Runtime> = None;
static mut CONTEXT: Option<Context> = None;
static mut FUNC_OBJ: Option<&mut JsValue> = None;

#[export_name = "wizer.initialize"]
pub unsafe extern "C" fn init() {
    let rt = Some(Runtime::new());
    let ctx = Some(Context::new_with_rt((rt.unwrap()).0));
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
    let func = ctx.unwrap().eval_global_str(code.to_string());
    RUNTIME = rt;
    CONTEXT = ctx;
    FUNC_OBJ = Some(Box::leak(Box::new(func)));
    forget(RUNTIME);
    forget(CONTEXT);
    forget(&FUNC_OBJ);
}

fn main() {
        unsafe {
            println!("return value:{:?}", FUNC_OBJ.as_ref().unwrap().invoke("handler", &[JsValue::String(CONTEXT.unwrap().new_string(r#"{"name":"John", "age":30, "city":"New York"}"#))]));
        }

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
