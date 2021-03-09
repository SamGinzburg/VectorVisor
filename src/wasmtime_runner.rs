use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Condvar;
use std::error::Error;

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
               vm_sender: Arc<Mutex<Sender<u32>>>,
               vm_recv: Arc<Mutex<Receiver<u32>>>,
               vm_recv_condvar: Arc<Condvar>) -> Result<(), Box<dyn Error>> {

        let store = Store::default();

        // serverless_invoke
        let serverless_invoke = Func::wrap(&store, move |buf_ptr: i32, buf_len: i32| -> () {
            //dbg!(buf_ptr);
            //dbg!(buf_len);

            let chan = vm_recv.clone();
            let msg = chan.lock().unwrap().recv().unwrap();
            dbg!(msg);
        });

        // serverless_invoke
        let serverless_response = Func::wrap(&store, move |buf_ptr: i32, buf_len: i32| -> () {
            //dbg!(buf_ptr);
            //dbg!(buf_len);

            let chan = vm_sender.clone();
            chan.lock().unwrap().send(0).unwrap();
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