use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Condvar;
use std::error::Error;
use std::convert::TryInto;

use crossbeam::channel::Sender;
use crossbeam::channel::Receiver;
use crossbeam::channel::bounded;

use wasmtime::*;
use wasi_common::WasiCtx;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime_wasi::Wasi;

pub struct WasmtimeRunner {}

impl WasmtimeRunner {
    // this is run once for each thread/VM
    pub fn run(program: String,
               vm_sender: Arc<Mutex<Sender<(Vec<u8>, usize)>>>,
               vm_recv: Arc<Mutex<Receiver<(Vec<u8>, usize)>>>,
               vm_recv_condvar: Arc<Condvar>) -> Result<(), Box<dyn Error>> {

        let store = Store::default();

        // serverless_invoke
        let serverless_invoke = Func::wrap(&store, move |caller: Caller<'_>, buf_ptr: u32, buf_len: u32| -> u32 {
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => Ok(mem),
                _ => Err(Trap::new("failed to find host memory")),
            };

            println!("test - blocked on input!");

            let chan = vm_recv.clone();
            let (msg, msg_len) = chan.lock().unwrap().recv().unwrap();

            //dbg!(msg);
            dbg!(msg_len);

            // copy the input to the VM
            match mem {
                Ok(memory) => {
                    unsafe {
                        let arr = memory.data_unchecked_mut();
                        let start = buf_ptr as usize;
                        let end = ((buf_ptr as usize)+msg_len);
                        arr[start..end].copy_from_slice(&msg[0..msg_len]);
                    }
                },
                Err(e) => {
                    panic!("Unable to find memory for WASM VM");
                }
            }

            msg_len.try_into().unwrap()
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
                    let chan = vm_sender.clone();
                    unsafe {
                        let arr = memory.data_unchecked_mut();
                        let mut resp_buf = vec![0u8; 16384];
                        let resp_buf_len: usize = buf_len.try_into().unwrap();
                        let main_mem_start = buf_ptr.try_into().unwrap();

                        let resp_buf_as_slice: &mut [u8] = resp_buf.as_mut_slice();
                        resp_buf_as_slice[0..resp_buf_len].copy_from_slice(&arr[main_mem_start..main_mem_start+resp_buf_len]);

                        chan.lock().unwrap().send((resp_buf, resp_buf_len)).unwrap();
                    }
                },
                Err(e) => {
                    panic!("Unable to find memory for WASM VM");
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

        let entry_point = instance.get_func("_start").unwrap().get0::<()>()?;

        // start running the VM
        Ok(entry_point()?)
    }
}