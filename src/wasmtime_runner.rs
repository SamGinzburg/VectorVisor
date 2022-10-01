use crate::{VmRecvType, VmSenderType};
use chrono::prelude::*;
use serde_json::Value;
use std::convert::TryInto;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc::{Receiver, Sender};
use wasi_cap_std_sync::WasiCtxBuilder;
use wasi_common::WasiCtx;
use wasmtime::*;

pub struct WasmtimeRunner {
    vm_idx: usize,
    vm_sender: Arc<Vec<Mutex<Sender<VmSenderType>>>>,
    vm_recv: Arc<Vec<Mutex<Receiver<VmRecvType>>>>,
}

impl WasmtimeRunner {
    pub fn new(
        vm_idx: usize,
        vm_sender: Arc<Vec<Mutex<Sender<VmSenderType>>>>,
        vm_recv: Arc<Vec<Mutex<Receiver<VmRecvType>>>>,
    ) -> WasmtimeRunner {
        WasmtimeRunner {
            vm_idx: vm_idx,
            vm_sender: vm_sender,
            vm_recv: vm_recv,
        }
    }
    // this is run once for each thread/VM
    pub fn run(
        &'static self,
        program: String,
        hcall_buf_size: usize,
        heap_size: u32,
    ) -> Result<(), Box<dyn Error>> {
        let curr_time = Arc::new(Mutex::<i64>::new(0));

        let mut config = Config::new();
        config.wasm_simd(true).wasm_bulk_memory(true);

        let engine = Engine::new(&config).expect("Could not init wasmtime engine");

        let wasi = WasiCtxBuilder::new().inherit_stdio().build();
        let mut store = Store::new(&engine, wasi);

        let curr_time_invoke = curr_time.clone();
        let curr_time_response = curr_time.clone();

        let current_uuid = Arc::new(Mutex::new(String::from("")));
        let curr_uuid_invoke = current_uuid.clone();
        let curr_uuid_response = current_uuid.clone();

        // serverless_invoke
        let serverless_invoke = Func::wrap(
            &mut store,
            move |mut caller: Caller<'_, WasiCtx>, buf_ptr: u32, _buf_len: u32| -> u32 {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => Ok(mem),
                    _ => Err(Trap::new("failed to find host memory")),
                };
                let chan = self.vm_recv.get(self.vm_idx).unwrap();
                let (msg, _, uuid) = chan.lock().unwrap().blocking_recv().unwrap();
                *curr_uuid_invoke.lock().unwrap() = uuid;

                /*
                // Parse JSON
                let incoming_json_obj: Value = serde_json::from_slice(&msg).unwrap();
                // Serialize parsed json
                let serialized_json = serde_cbor::ser::to_vec_packed(&incoming_json_obj).unwrap();
                */

                // copy the input to the VM
                match mem {
                    Ok(memory) => {
                        let arr = memory.data_mut(&mut caller);
                        let start = buf_ptr as usize;
                        let end = (buf_ptr as usize) + msg.len();
                        arr[start..end].copy_from_slice(&msg);
                    }
                    Err(e) => {
                        panic!("Unable to find memory for WASM VM: {}", e);
                    }
                }

                let tsc = curr_time_invoke.clone();
                *tsc.lock().unwrap() = Utc::now().timestamp_nanos();

                msg.len().try_into().unwrap()
            },
        );

        // serverless_invoke
        let serverless_response = Func::wrap(
            &mut store,
            move |mut caller: Caller<'_, _>, buf_ptr: u32, buf_len: u32| -> () {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => Ok(mem),
                    _ => Err(Trap::new("failed to find host memory")),
                };

                // copy the output json
                match mem {
                    Ok(memory) => {
                        let chan = self.vm_sender.get(self.vm_idx).unwrap();
                        let arr = memory.data(&caller);

                        // Debug memory usage of functions
                        //println!("wasmtime vm memory len: {:?}", &arr.len());

                        let resp_buf_len: usize = buf_len.try_into().unwrap();
                        let mut resp_buf = vec![0u8; resp_buf_len];
                        let main_mem_start = buf_ptr.try_into().unwrap();

                        let resp_buf_as_slice: &mut [u8] = resp_buf.as_mut_slice();
                        resp_buf_as_slice[0..resp_buf_len]
                            .copy_from_slice(&arr[main_mem_start..main_mem_start + resp_buf_len]);

                        let tsc = curr_time_response.clone();
                        let device_execution_time =
                            Utc::now().timestamp_nanos() - *tsc.lock().unwrap();
                        let resp_uuid: String = curr_uuid_response.lock().unwrap().to_string();

                        chan.lock()
                            .unwrap()
                            .blocking_send((
                                bytes::Bytes::from(resp_buf),
                                resp_buf_len,
                                device_execution_time.try_into().unwrap(),
                                0,
                                0,
                                0,
                                0,
                                resp_uuid,
                            ))
                            .unwrap();
                    }
                    Err(e) => {
                        panic!("Unable to find memory for WASM VM: {}", e);
                    }
                }
            },
        );

        let mut linker = Linker::new(&engine);
        linker.define("env", "serverless_invoke", serverless_invoke)?;
        linker.define("env", "serverless_response", serverless_response)?;

        wasmtime_wasi::sync::add_to_linker(&mut linker, |s| s)?;

        let module = Module::new(store.engine(), program)?;
        let instance = linker.instantiate(&mut store, &module)?;

        let memory = instance.get_memory(&mut store, "memory").unwrap();
        let current_mem_size = memory.size(&store);
        if current_mem_size < heap_size.into() {
            memory.grow(&mut store, (heap_size as u64) - current_mem_size)?;
        }
        //dbg!(&memory.size());

        let entry_point = instance
            .get_func(&mut store, "_start")
            .expect("Could not find _start function in WASM binary");

        // start running the VM
        let result = Ok(entry_point.call(&mut store, &[], &mut []).unwrap());

        dbg!(&memory.size(store));
        result
    }
}
