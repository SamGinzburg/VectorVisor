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
    fn serverless_invoke(input_arr: *mut u8, input_arr_len: u32) -> u32;

    // return the json response back to the VMM
    fn serverless_response(input_arr: *mut u8, input_arr_len: u32) -> ();
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
        loop {
            // block until we get a request, which we will populate into the buffer
            let incoming_req_size = unsafe {
                serverless_invoke(buffer.as_mut_ptr(), buffer.len() as u32)
            };

            println!("simple println! test");
            println!("incoming req size: {:?}", incoming_req_size);
            println!("{:?}", &buffer[..incoming_req_size as usize]);

            // now that we have the input in the buffer, parse the json
            let json: Value = serde_json::from_slice(&buffer[..incoming_req_size as usize]).unwrap();

            // run the function, get the response
            func_ret_val = (self.function)(json);

            // copy the response to the buffer
            let func_ret_val_as_buffer = to_string(&func_ret_val).unwrap();
            buffer[..func_ret_val_as_buffer.len()].clone_from_slice(func_ret_val_as_buffer.as_bytes());

            // return the response
            unsafe {
                serverless_response(buffer.as_mut_ptr(), func_ret_val_as_buffer.len() as u32);
            }
        }
    }
}
