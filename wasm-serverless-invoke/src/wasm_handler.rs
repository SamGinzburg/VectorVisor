use serde_json::{json, Value, to_string};
use serde::Deserialize;
use serde::Serialize;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

enum FuncReturn {
    None,

}

pub struct WasmHandler<T1, T2> {
    function: Box<dyn Fn(T1) -> T2>
}

extern "C" {
    // Our syscall will write directly to a buffer of 16KiB in size
    // which we will cast to a json input
    fn serverless_invoke(input_arr: *const u8, input_arr_len: u32) -> u32;

    // return the json response back to the VMM
    fn serverless_response(input_arr: *const u8, input_arr_len: u32) -> ();
}

impl<'a, T1: Deserialize<'a>, T2: Serialize> WasmHandler<T1, T2> {

    pub fn new(func: &'static (dyn Fn(T1) -> T2)) -> WasmHandler<T1, T2> {
        WasmHandler {
            function: Box::new(func),
        }
    }

    pub fn run(self, hcall_buf_size: usize) -> () {
        // main run loop of the runtime
        // First, allocate a buffer to store json input
        let mut buffer: &mut Vec<u8> = Box::leak(Box::new(vec![0u8; hcall_buf_size]));
        //let mut buffer = vec![0u8; hcall_buf_size];
        let mut func_ret_val: T2;
        // if this is the first invocation, then we skip sending the buffer back
        loop {

            // block until we get a request, which we will populate into the buffer
            let incoming_req_size = unsafe {
                serverless_invoke(buffer.as_ptr(), buffer.len() as u32)
            };

            // now that we have the input in the buffer, parse the json
            match serde_json::from_slice(&buffer[..incoming_req_size as usize]) {
                Ok(json) => {
                    // run the function, get the response
                    func_ret_val = (self.function)(json);

                    // copy the response to the buffer
                    let func_ret_val_as_buffer = to_string(&func_ret_val).unwrap();
                    // return the response
                    unsafe {
                        serverless_response(func_ret_val_as_buffer.as_ptr(), func_ret_val_as_buffer.len() as u32);
                    }
                },
                Err(_) => {
                    // return the response
                    unsafe {
                        serverless_response(buffer.as_ptr(), 0 as u32);
                    }
                },
            };
        }
    }
}
