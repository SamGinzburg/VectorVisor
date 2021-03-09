use wasi_common::WasiCtx;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasi_cap_std_sync::file::*;

use std::fmt;
use wasmtime::*;
use wasmtime_wiggle::WasmtimeGuestMemory;
use wasi_cap_std_sync::dir::Dir as WasiDir;
use cap_std::fs::Dir as CapDir;

use crate::opencl_runner::OpenCLBuffers;

use crate::opencl_runner::WasiFd;
use crate::opencl_runner::environment::Environment;
use crate::opencl_runner::serverless::Serverless;

use ocl::core::CommandQueue;

use crossbeam::channel::Sender;
use crossbeam::channel::Receiver;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Condvar;
use std::sync::Barrier;

use std::convert::TryInto;
use std::fs::File;
use std::path::Path;

#[derive(Clone, Copy)]
pub enum WasiSyscalls {
    FdWrite               =  0,
    ProcExit              =  1,
    EnvironSizeGet        =  2,
    EnvironGet            =  3,
    FdPrestatGet          =  4,
    FdPrestatDirName      =  5,
    ServerlessInvoke      =  9999,
    ServerlessResponse    =  10000,
    InvalidHyperCallNum   = -1,
}

impl fmt::Debug for WasiSyscalls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct HyperCall<'a> {
    pub vm_id: u32,
    pub num_total_vms: u32,
    pub sp: u64,
    pub syscall: WasiSyscalls,
    pub is_interleaved_mem: bool,
    pub ocl_buffers: &'a OpenCLBuffers,
    pub hypercall_buffer: Arc<Mutex<&'a mut [u8]>>,
    pub queue: &'a CommandQueue,
}

impl<'a> HyperCall<'a> {
    pub fn new(vm_id: u32,
               num_total_vms: u32,
               sp: u64,
               syscall: WasiSyscalls,
               is_interleaved_mem: bool,
               ocl_buffers: &'a OpenCLBuffers,
               hypercall_buffer: Arc<Mutex<&'a mut [u8]>>,
               queue: &'a CommandQueue) -> HyperCall<'a> {
        HyperCall {
            vm_id: vm_id,
            num_total_vms: num_total_vms,
            sp: sp,
            syscall: syscall,
            is_interleaved_mem: is_interleaved_mem,
            ocl_buffers: ocl_buffers,
            hypercall_buffer: hypercall_buffer,
            queue: queue,
        }
    }
}

impl fmt::Debug for HyperCall<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HyperCall")
        .field("vm_id", &self.vm_id)
        .field("syscall_id", &(self.syscall as u8))
        .finish()
    }
}

pub struct HyperCallResult {
    result_value: i32,
    hypercall_type: WasiSyscalls,
    vm_id: u32,
}

impl HyperCallResult {
    pub fn new(result: i32, vm_id: u32, call_type: WasiSyscalls) -> HyperCallResult {
        HyperCallResult {
            result_value: result,
            hypercall_type: call_type,
            vm_id: vm_id,
        }
    }
    pub fn get_result(&self) -> i32 {
        self.result_value
    }
    pub fn get_type(&self) -> WasiSyscalls {
        self.hypercall_type.clone()
    }

    pub fn get_vm_id(&self) -> u32 {
        self.vm_id
    }
}

pub struct VectorizedVM {
    // each VM has its own WASI state tracking object
    ctx: WasiCtx,
    engine: Engine,
    store: Store,
    pub memory: Memory,
    pub wasm_memory: WasmtimeGuestMemory,
    pub enviroment_size: Option<u32>,
    pub environment_str_size: Option<u32>,
    vm_id: u32,
    pub vm_sender: Arc<Mutex<Sender<u32>>>,
    pub vm_recv:   Arc<Mutex<Receiver<u32>>>,
    pub vm_recv_condvar: Arc<Condvar>,
    pub vm_barrier: Arc<Barrier>
}

impl VectorizedVM {
    pub fn new(vm_id: u32, num_total_vms: u32, vm_sender: Arc<Mutex<Sender<u32>>>, vm_recv: Arc<Mutex<Receiver<u32>>>, vm_recv_condvar: Arc<Condvar>) -> VectorizedVM {
        // default context with no args yet - we can inherit arguments from the CLI if we want
        // or we can pass them in some other config file

        let opendir = unsafe { CapDir::from_std_file(File::open(".").unwrap()) };

        let wasi_ctx = WasiCtxBuilder::new()
                        .inherit_args().unwrap()
                        .inherit_stdio()
                        .inherit_env().unwrap()
                        // preopen whatever the current directory is
                        // TODO: pass this via CLI somehow
                        .preopened_dir(opendir, Path::new(".")).unwrap()
                        .build().unwrap();

        let engine = Engine::default();
        let store = Store::new(&engine);
        let memory_ty = MemoryType::new(Limits::new(1, None));
        let memory = Memory::new(&store, memory_ty);

        let barrier = Arc::new(Barrier::new(num_total_vms.try_into().unwrap()));

        VectorizedVM {
            ctx: wasi_ctx,
            engine: engine,
            store: store,
            /*
             * Memories are internally reference counted so you can clone a Memory. The cloning process only performs a shallow clone, so two cloned Memory instances are equivalent in their functionality.
             * See: https://docs.wasmtime.dev/api/wasmtime/struct.Memory.html
             */
            memory: memory.clone(),
            wasm_memory: WasmtimeGuestMemory::new(memory),
            enviroment_size: None,
            environment_str_size: None,
            vm_id: vm_id,
            vm_sender: vm_sender,
            vm_recv: vm_recv,
            vm_recv_condvar: vm_recv_condvar,
            vm_barrier: barrier
        }
    }

    /*
     * This function dispatched the hypercalls for each VM,
     * If interleaved memory is being used, we can access the raw_mem slices passed in
     * For non interleaved memory, we must perform concurrent reads on the openCL context
     * using the given buffers.
     */
    pub fn dispatch_hypercall(&self,
                              hypercall: &mut HyperCall,
                              sender: &Sender<HyperCallResult>) -> () {
        match hypercall.syscall {
            WasiSyscalls::FdWrite => {
                WasiFd::hypercall_fd_write(&self.ctx, self, hypercall, sender);
            },
            // ProcExit is special cased, since we want to manually mask off those VMs
            WasiSyscalls::ProcExit => {
                sender.send({
                    HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::ProcExit)
                }).unwrap();
            },
            WasiSyscalls::EnvironSizeGet => {
                Environment::hypercall_environ_sizes_get(&self.ctx, hypercall, sender);
            },
            WasiSyscalls::EnvironGet => {
                Environment::hypercall_environ_get(&self.ctx, self, hypercall, sender);
            },
            WasiSyscalls::FdPrestatGet => {
                WasiFd::hypercall_fd_prestat_get(&self.ctx, self, hypercall, sender);
            },
            WasiSyscalls::FdPrestatDirName => {
                WasiFd::hypercall_fd_prestat_dir_name(&self.ctx, self, hypercall, sender);
            },
            WasiSyscalls::ServerlessInvoke => {
                Serverless::hypercall_serverless_invoke(&self.ctx, self, hypercall, sender);
            },
            WasiSyscalls::ServerlessResponse => {
                Serverless::hypercall_serverless_response(&self.ctx, self, hypercall, sender);
            },
            _ => panic!("Unsupported hypercall invoked! {:?}", hypercall),
        }

    }
}

impl fmt::Debug for VectorizedVM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VectorizedVM")
        .field("vm_id", &self.vm_id)
        .finish()
    }
}