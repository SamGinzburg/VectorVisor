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
    fn serverless_invoke(input_arr: *mut u8, input_arr_len: u32) -> u32;

    // return the json response back to the VMM
    fn serverless_response(output_arr: *mut u8, output_arr_len: u32) -> ();
}

impl<'a, T1: Deserialize<'a>, T2: Serialize> WasmHandler<T1, T2> {

    pub fn new(func: &'static (dyn Fn(T1) -> T2)) -> WasmHandler<T1, T2> {
        WasmHandler {
            function: Box::new(func),
        }
    }

    // easy hack to get *mut u8 ref from buffer in our loop, we ensure that this operation is safe
    unsafe fn get_unsafe_mut_ref(buf: &[u8]) -> *mut u8 {
        let const_ptr: *const u8 = buf.as_ptr();
        let mut_ptr: *mut u8 = const_ptr as *mut u8;
        mut_ptr
    }

    pub fn run(self, hcall_buf_size: usize) -> () {
        // main run loop of the runtime
        // First, allocate a buffer to store json input
        let mut buffer: &mut Vec<u8> = Box::leak(Box::new(vec![0u8; hcall_buf_size]));

        let mut func_ret_val: T2;
        // if this is the first invocation, then we skip sending the buffer back
        loop {

            // block until we get a request, which we will populate into the buffer
            let incoming_req_size = unsafe {
                let buf_ptr = WasmHandler::<T1, T2>::get_unsafe_mut_ref(buffer);
                serverless_invoke(buf_ptr, buffer.len() as u32)
            };

            // now that we have the input in the buffer, parse the json
            match serde_json::from_slice(&buffer[..incoming_req_size as usize]) {
                Ok(json) => {
                    // run the function, get the response
                    func_ret_val = (self.function)(json);

                    // copy the response to the buffer
                    let mut func_ret_val_as_buffer = to_string(&func_ret_val).unwrap();
                    // return the response
                    unsafe {
                        serverless_response(func_ret_val_as_buffer.as_mut_ptr(), func_ret_val_as_buffer.len() as u32);
                    }
                },
                Err(_) => {
                    // return an empty response if we were unable to parse the input properly
                    unsafe {
                        let buf_ptr = WasmHandler::<T1, T2>::get_unsafe_mut_ref(buffer);
                        serverless_response(buf_ptr, 0 as u32);
                    }
                },
            };
        }
    }
}
