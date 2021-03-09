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
        let mut linker = Linker::new(&store);
    
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