use wasi_common::WasiCtx;
use std::fmt;

use crate::opencl_runner::OpenCLBuffers;

use crate::opencl_runner::WasiFd;
use ocl::core::CommandQueue;

use crossbeam::channel::Sender;

use std::sync::Arc;
use std::cell::RefCell;

#[derive(Clone, Copy)]
pub enum WasiSyscalls {
    FdWrite,
    ProcExit,
    InvalidHyperCallNum
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
    pub hypercall_buffer: Arc<&'a mut [u8]>,
    pub queue: &'a CommandQueue,
}

impl<'a> HyperCall<'a> {
    pub fn new(vm_id: u32,
               num_total_vms: u32,
               sp: u64,
               syscall: WasiSyscalls,
               is_interleaved_mem: bool,
               ocl_buffers: &'a OpenCLBuffers,
               hypercall_buffer: Arc<&'a mut [u8]>,
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
    vm_id: u32,
}

impl VectorizedVM {
    pub fn new(vm_id: u32) -> VectorizedVM {
        // default context with no args yet - we can inherit arguments from the CLI if we want
        // or we can pass them in some other config file
        let wasi_ctx = WasiCtx::new(&[""]).unwrap();

        VectorizedVM {
            ctx: wasi_ctx,
            vm_id: vm_id,
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
                WasiFd::hypercall_fd_write(&self.ctx, hypercall, sender);
            },
            // ProcExit is special cased, since we want to manually mask off those VMs
            WasiSyscalls::ProcExit => {
                sender.send({
                    HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::ProcExit)
                }).unwrap();
            }
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