use serde_json::{json, Value, to_string};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

enum FuncReturn {
    None,

}

pub struct WasmHandler {
    function: Box<dyn Fn(Value) -> Value>
}

extern "C" {
    // Our syscall will write directly to a buffer of 16KiB in size
    // which we will cast to a json input
    fn serverless_invoke(input_arr: *mut u8, is_first_inv: bool) -> ();
}

impl WasmHandler {

    pub fn new(func: &'static (dyn Fn(Value) -> Value)) -> WasmHandler {
        WasmHandler {
            function: Box::new(func),
        }
    }

    pub fn run(self) -> () {
        // main run loop of the runtime
        // First, allocate a buffer to store json input
        let mut buffer = [0u8; 1024 * 16];
        let mut func_ret_val: Value;
        // if this is the first invocation, then we skip sending the buffer back
        let mut is_first_inv = true;
        loop {
            // block until we get a request, which we will populate into the buffer
            unsafe {
                serverless_invoke(buffer.as_mut_ptr(), is_first_inv);
                is_first_inv = false;
            }

            // now that we have the input in the buffer, parse the json
            let json: Value = serde_json::from_slice(&buffer).unwrap();

            // run the function, get the response
            func_ret_val = (self.function)(json);

            // copy the response to the buffer
            let func_ret_val_as_buffer = to_string(&func_ret_val).unwrap();
            buffer.clone_from_slice(func_ret_val_as_buffer.as_bytes());
        }
    }
}