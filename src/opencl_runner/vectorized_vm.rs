use wasi_cap_std_sync::WasiCtxBuilder;
use wasi_common::WasiCtx;

use cap_std::fs::Dir as CapDir;
use std::fmt;
use wasmtime::*;
use wiggle::wasmtime::WasmtimeGuestMemory;

use crate::opencl_runner::OpenCLBuffers;

use crate::opencl_runner::environment::Environment;
use crate::opencl_runner::random::Random;
use crate::opencl_runner::wasi_time::Clock;

use crate::opencl_runner::serverless::Serverless;
use crate::opencl_runner::UnsafeCellWrapper;
use crate::opencl_runner::WasiFd;

use ocl::core::CommandQueue;

use crossbeam::channel::Sender as SyncSender;
use tokio::sync::mpsc::{Receiver, Sender};

use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::fs::File;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::convert::TryFrom;
use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(i32)]
pub enum WasiSyscalls {
    FdWrite = 0,
    ProcExit = 1,
    EnvironSizeGet = 2,
    EnvironGet = 3,
    FdPrestatGet = 4,
    FdPrestatDirName = 5,
    RandomGet = 6,
    ClockTimeGet = 7,
    ServerlessInvoke = 9999,
    ServerlessResponse = 10000,
    InvalidHyperCallNum = -1,
}

impl fmt::Debug for WasiSyscalls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct HyperCall<'a> {
    pub vm_id: u32,
    pub num_total_vms: u32,
    pub timestamp_counter: u64,
    pub queue_submit_delta: u64,
    pub num_queue_submits: u64,
    pub called_fns: HashSet<u32>,
    pub syscall: WasiSyscalls,
    pub is_interleaved_mem: u32,
    pub ocl_buffers: &'a OpenCLBuffers,
    pub hypercall_buffer: Arc<UnsafeCellWrapper<u8>>,
    pub queue: &'a CommandQueue,
    pub overhead_tracker: Arc<UnsafeCellWrapper<u64>>,
    pub non_serverless_invoke_call_found: bool,
}

impl<'a> HyperCall<'a> {
    pub fn new(
        vm_id: u32,
        num_total_vms: u32,
        timestamp_counter: u64,
        queue_submit_delta: u64,
        num_queue_submits: u64,
        called_funcs: HashSet<u32>,
        syscall: WasiSyscalls,
        is_interleaved_mem: u32,
        ocl_buffers: &'a OpenCLBuffers,
        hypercall_buffer: Arc<UnsafeCellWrapper<u8>>,
        queue: &'a CommandQueue,
        overhead_tracker: Arc<UnsafeCellWrapper<u64>>,
        non_serverless_invoke_call_found: bool,
    ) -> HyperCall<'a> {
        HyperCall {
            vm_id: vm_id,
            num_total_vms: num_total_vms,
            syscall: syscall,
            is_interleaved_mem: is_interleaved_mem,
            ocl_buffers: ocl_buffers,
            hypercall_buffer: hypercall_buffer,
            timestamp_counter: timestamp_counter,
            queue_submit_delta: queue_submit_delta,
            num_queue_submits: num_queue_submits,
            called_fns: called_funcs,
            queue: queue,
            overhead_tracker: overhead_tracker,
            non_serverless_invoke_call_found: non_serverless_invoke_call_found,
        }
    }
}

impl fmt::Debug for HyperCall<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HyperCall")
            .field("vm_id", &self.vm_id)
            .field("syscall_id", &(self.syscall as u8))
            .field("non_serverless_invoke_call_found", &self.non_serverless_invoke_call_found)
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

pub type VmSenderType = (bytes::Bytes, usize, u64, u64, u64, u64, u64, String);
pub type VmRecvType = (bytes::Bytes, usize, String);

pub struct VectorizedVM {
    // each VM has its own WASI state tracking object
    pub ctx: WasiCtx,
    _engine: Engine,
    pub store: Store<u8>,
    pub memory: Memory,
    pub enviroment_size: Option<u32>,
    pub environment_str_size: Option<u32>,
    pub vm_id: u32,
    pub hcall_buf_size: u32,
    pub timestamp_counter: Arc<u64>,
    pub overhead_counter: Arc<u64>,
    pub queue_submit_counter: Arc<u64>,
    pub queue_submit_qty: Arc<u64>,
    pub called_fns_set: Arc<HashSet<u32>>,
    pub vm_sender: Arc<Vec<Mutex<Sender<VmSenderType>>>>,
    pub vm_recv: Arc<Vec<Mutex<Receiver<VmRecvType>>>>,
    pub ready_for_input: AtomicBool,
    pub input_msg_len: usize,
    pub no_resp: bool,
    pub uuid_queue: VecDeque<String>,
}

impl VectorizedVM {
    pub fn new(
        vm_id: u32,
        hcall_buf_size: u32,
        _num_total_vms: u32,
        vm_sender: Arc<Vec<Mutex<Sender<VmSenderType>>>>,
        vm_recv: Arc<Vec<Mutex<Receiver<VmRecvType>>>>,
    ) -> VectorizedVM {
        // default context with no args yet - we can inherit arguments from the CLI if we want
        // or we can pass them in some other config file

        let opendir = wasi_cap_std_sync::Dir::from_std_file(File::open(".").unwrap());

        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_args()
            .unwrap()
            .inherit_stdio()
            .inherit_env()
            .unwrap()
            // preopen whatever the current directory is
            // TODO: pass this via CLI somehow
            .preopened_dir(opendir, Path::new("."))
            .unwrap()
            .build();

        let engine = Engine::default();
        let mut store = Store::new(&engine, 0u8);

        let num_vm_pages = if hcall_buf_size >= (1024 * 64) {
            hcall_buf_size / (1024 * 64)
        } else {
            1
        };

        let memory_ty = MemoryType::new(num_vm_pages, None);
        let memory = Memory::new(&mut store, memory_ty).unwrap();
        //let raw_mem: &mut [u8] = memory.data_mut(&mut store);

        VectorizedVM {
            ctx: wasi_ctx,
            _engine: engine,
            store: store,
            memory: memory.clone(),
            enviroment_size: None,
            environment_str_size: None,
            vm_id: vm_id,
            hcall_buf_size: hcall_buf_size,
            timestamp_counter: Arc::new(0),
            overhead_counter: Arc::new(0),
            queue_submit_counter: Arc::new(0),
            queue_submit_qty: Arc::new(0),
            called_fns_set: Arc::new(HashSet::new()),
            vm_sender: vm_sender,
            vm_recv: vm_recv,
            ready_for_input: AtomicBool::new(true),
            input_msg_len: 0,
            no_resp: true,
            uuid_queue: VecDeque::new(),
        }
    }

    pub fn is_avail(&mut self) -> bool {
        self.ready_for_input.load(Ordering::Relaxed).clone()
    }

    pub fn queue_request(&mut self, msg: bytes::Bytes, hcall_buf: &mut [u8], uuid: String) -> () {
        let hcall_buf_size: u32 = self.hcall_buf_size;
        let vm_hcall_buf = &mut hcall_buf
            [(self.vm_id * hcall_buf_size) as usize..((self.vm_id + 1) * hcall_buf_size) as usize];
        vm_hcall_buf[0..msg.len()].copy_from_slice(&msg);
        self.ready_for_input.store(false, Ordering::Relaxed);
        self.input_msg_len = msg.len();
        self.uuid_queue.push_back(uuid);
    }

    /*
     * This function dispatched the hypercalls for each VM,
     * If interleaved memory is being used, we can access the raw_mem slices passed in
     * For non interleaved memory, we must perform concurrent reads on the openCL context
     * using the given buffers.
     */
    pub fn dispatch_hypercall(
        &mut self,
        hypercall: &mut HyperCall,
        sender: &SyncSender<HyperCallResult>,
    ) -> () {
        match hypercall.syscall {
            WasiSyscalls::FdWrite => {
                WasiFd::hypercall_fd_write(self, hypercall, sender);
            }
            // ProcExit is special cased, since we want to manually mask off those VMs
            WasiSyscalls::ProcExit => {
                sender
                    .send(HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::ProcExit))
                    .unwrap();
            }
            WasiSyscalls::EnvironSizeGet => {
                Environment::hypercall_environ_sizes_get(self, hypercall, sender);
            }
            WasiSyscalls::EnvironGet => {
                Environment::hypercall_environ_get(self, hypercall, sender);
            }
            WasiSyscalls::FdPrestatGet => {
                WasiFd::hypercall_fd_prestat_get(self, hypercall, sender);
            }
            WasiSyscalls::FdPrestatDirName => {
                WasiFd::hypercall_fd_prestat_dir_name(self, hypercall, sender);
            }
            WasiSyscalls::ServerlessInvoke => {
                Serverless::hypercall_serverless_invoke(self, hypercall, sender);
            }
            WasiSyscalls::ServerlessResponse => {
                Serverless::hypercall_serverless_response(self, hypercall, sender);
                self.ready_for_input.store(true, Ordering::Relaxed);
            }
            WasiSyscalls::RandomGet => {
                Random::hypercall_random_get(self, hypercall, sender);
            }
            WasiSyscalls::ClockTimeGet => {
                Clock::hypercall_clock_time_get(self, hypercall, sender);
            }
            /*
            _ => {
                sender.send({
                    HyperCallResult::new(-1, hypercall.vm_id, WasiSyscalls::InvalidHyperCallNum)
                }).unwrap();
            },
            */
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
