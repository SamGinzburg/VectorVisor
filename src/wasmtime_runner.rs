use std::sync::Arc;
use std::sync::Mutex;
use std::error::Error;
use std::convert::TryInto;
use serde_json::Value;
use tokio::sync::mpsc::{Sender, Receiver};
use wasmtime::*;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime_wasi::Wasi;
use chrono::prelude::*;

pub struct WasmtimeRunner {
    vm_idx: usize,
    vm_sender: Arc<Vec<Mutex<Sender<(Vec<u8>, usize, u64, u64, u64, u64)>>>>,
    vm_recv: Arc<Vec<Mutex<Receiver<(bytes::Bytes, usize)>>>>
}

impl WasmtimeRunner {
    pub fn new(vm_idx: usize, vm_sender: Arc<Vec<Mutex<Sender<(Vec<u8>, usize, u64, u64, u64, u64)>>>>,
               vm_recv: Arc<Vec<Mutex<Receiver<(bytes::Bytes, usize)>>>>) -> WasmtimeRunner {
            WasmtimeRunner {
                vm_idx: vm_idx,
                vm_sender: vm_sender,
                vm_recv: vm_recv
            }
    }
    // this is run once for each thread/VM
    pub fn run(&'static self, program: String, hcall_buf_size: usize, heap_size: u32) -> Result<(), Box<dyn Error>> {

        let curr_time = Arc::new(Mutex::<i64>::new(0));

        let store = Store::default();

        let curr_time_invoke = curr_time.clone();
        let curr_time_response = curr_time.clone();

        // serverless_invoke
        let serverless_invoke = Func::wrap(&store, move |caller: Caller<'_>, buf_ptr: u32, _buf_len: u32| -> u32 {
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => Ok(mem),
                _ => Err(Trap::new("failed to find host memory")),
            };

            let chan = self.vm_recv.get(self.vm_idx).unwrap();
            let (msg, _) = chan.lock().unwrap().blocking_recv().unwrap();

            /*
            // Parse JSON
            let incoming_json_obj: Value = serde_json::from_slice(&msg).unwrap();
            // Serialize parsed json
            let serialized_json = serde_cbor::ser::to_vec_packed(&incoming_json_obj).unwrap();
            */

            // copy the input to the VM
            match mem {
                Ok(memory) => {
                    unsafe {
                        let arr = memory.data_unchecked_mut();
                        let start = buf_ptr as usize;
                        let end = (buf_ptr as usize)+msg.len();
                        arr[start..end].copy_from_slice(&msg);
                    }
                },
                Err(e) => {
                    panic!("Unable to find memory for WASM VM: {}", e);
                }
            }

            let tsc = curr_time_invoke.clone();
            *tsc.lock().unwrap() = Utc::now().timestamp_nanos();

            msg.len().try_into().unwrap()
        });

        // serverless_invoke
        let serverless_response = Func::wrap(&store, move |caller: Caller<'_>, buf_ptr: u32, buf_len: u32| -> () {
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => Ok(mem),
                _ => Err(Trap::new("failed to find host memory")),
            };

            // copy the output json 
            match mem {
                Ok(memory) => {
                    // Debug memory usage of functions
                    dbg!(memory.size());

                    let chan = self.vm_sender.get(self.vm_idx).unwrap();
                    unsafe {
                        let arr = memory.data_unchecked_mut();
                        let resp_buf_len: usize = buf_len.try_into().unwrap();
                        let mut resp_buf = vec![0u8; resp_buf_len];
                        let main_mem_start = buf_ptr.try_into().unwrap();

                        let resp_buf_as_slice: &mut [u8] = resp_buf.as_mut_slice();
                        resp_buf_as_slice[0..resp_buf_len].copy_from_slice(&arr[main_mem_start..main_mem_start+resp_buf_len]);

                        let tsc = curr_time_response.clone();
                        let device_execution_time = Utc::now().timestamp_nanos() - *tsc.lock().unwrap();

                        chan.lock().unwrap().blocking_send((resp_buf, resp_buf_len, device_execution_time.try_into().unwrap(), 0, 0, 0)).unwrap();
                    }
                },
                Err(e) => {
                    panic!("Unable to find memory for WASM VM: {}", e);
                }
            }
        });

        let mut linker = Linker::new(&store);
        linker.define("env", "serverless_invoke", serverless_invoke)?;
        linker.define("env", "serverless_response", serverless_response)?;

        let wasi = Wasi::new(
            &store,
            WasiCtxBuilder::new()
                .inherit_stdio()
                .build()?,
        );

        wasi.add_to_linker(&mut linker)?;

        let module = Module::new(store.engine(), program)?;
        let instance = linker.instantiate(&module)?;

        let memory = instance.get_memory("memory").unwrap();
        let current_mem_size = memory.size();
        if current_mem_size < heap_size {
            memory.grow(heap_size - current_mem_size)?;
        }
        //dbg!(&memory.size());

        let entry_point = instance.get_func("_start").unwrap().get0::<()>()?;

        // start running the VM
        Ok(entry_point()?)
    }
}
