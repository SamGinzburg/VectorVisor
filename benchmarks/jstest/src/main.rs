use wasmedge_quickjs::{Context, Runtime};

fn main() {
    //println!("init");
    let mut rt = Runtime::new();
    rt.run_with_context(|ctx| {
        //println!("\n<----run_simple_js---->");
        let code = r#"const obj = JSON.parse('{"name":"John", "age":30, "city":"New York"}'); console.log(obj.name);"#;
        let r = ctx.eval_global_str(code.to_string());
        //println!("return value:{:?}", r);
        ctx.js_loop().unwrap();
    });

}
