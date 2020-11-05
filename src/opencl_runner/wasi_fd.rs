use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use wasi_common::wasi::wasi_snapshot_preview1::WasiSnapshotPreview1;
//use wasi_common::snapshots::wasi_snapshot_preview1::WasiSnapshotPreview1;
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use crate::opencl_runner::interleave_offsets::Interleave;

use ocl::core::Event;

use wasi_common::wasi::types::CiovecArray;
use wasi_common::wasi::types::Ciovec;
use wasi_common::fs::Fd;

use wasmtime::*;
use wasmtime_wiggle::WasmtimeGuestMemory;
use wiggle::GuestPtr;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use byteorder::ByteOrder;
use std::convert::TryInto;
use crossbeam::channel::Sender;

use std::sync::Arc;

pub struct WasiFd {}

impl WasiFd {
    pub fn hypercall_fd_write(ctx: &WasiCtx, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        /*
            pub vm_id: u32,
            syscall: WasiSyscalls,
            is_interleaved_mem: bool,
            ocl_buffers: &'a OpenCLBuffers,
            raw_mem_stack: Option<&'a [u8]>,
            raw_mem_heap: Option<&'a [u8]>
        */

        /*
         * Our VMM supports two distinct memory models, standard and interleaved. 
         *  In the interleaved memory model all offsets of the same value are adjacent to each other,
         *  so we can coalesce reads/writes to memory for maximum efficiency. 
         * 
         * For the standard memory model we will do all reads/writes to GPU memory right here
         */
        if hypercall.is_interleaved_mem {
            // we need 4*4*hypercall.number_vms because of the interleaved memory model
            let mut stack_bytes: &mut [u8] = &mut vec![0; 4*4*hypercall.num_total_vms as usize].into_boxed_slice();
            // read the last 4 32 bit values off of the stack
            let num_vms = hypercall.num_total_vms as u64;
            let stack_read_offset = hypercall.sp*num_vms-(4*num_vms);
            println!("{}", stack_read_offset);
            unsafe {
                ocl::core::enqueue_read_buffer(&hypercall.queue, &hypercall.ocl_buffers.stack_buffer, true, stack_read_offset as usize, &mut stack_bytes, None::<Event>, None::<&mut Event>).unwrap();
            }

            println!("{:?}", stack_bytes);

            let fd = stack_bytes[0];
            let num_iovecs = stack_bytes[2] as usize;
            let _nbytes = stack_bytes[3];
            let heap_offset = stack_bytes[1];


            sender.send({
                HyperCallResult::new(0 as i32, hypercall.vm_id, WasiSyscalls::FdWrite)
            }).unwrap();

        } else {

            let hcall_buf: &mut [u8] = unsafe { *Arc::get_mut_unchecked(&mut hypercall.hypercall_buffer) };

            /*
             * It may seem inefficient to recreate this for every hypercall, but it is actually far more efficient!
             * This is because we only have N active threads at a time, so we only have N 64KiB pages allocated at a time!
             * 
             * Preallocating these structures adds *massive* overhead - 64KiB * (1024 * 16) = 1GiB!!!
             * 
             * The compute tradeoff is definately worth it, as we are far more memory-bound than anything else in this system
             * 
             */
            let engine = Engine::default();
            let store = Store::new(&engine);
            let memory_ty = MemoryType::new(Limits::new(1, None));
            let memory = Memory::new(&store, memory_ty);

            let raw_mem: &mut [u8] = unsafe { memory.data_unchecked_mut() };

            // copy the hypercall buffer over to the memory object
            // TODO: we can optimize this later, to read minimal amounts of memory
            // Further TODO: we also need to create a generic interleaved read/write function for this
            for idx in 16..16384 {
                raw_mem[idx - 16] = hcall_buf[idx];
            }

            // after all reads/writes are done, wrap the memory with the GuestMemory trait
            let wasm_mem = WasmtimeGuestMemory::new(memory);

            // we hardcode the ciovec array to start at offset 0
            let ciovec_ptr: &CiovecArray = &GuestPtr::new(&wasm_mem, (0 as u32, 1 as u32));

            let result = ctx.fd_write(Fd::from(1), &ciovec_ptr);

            sender.send({
                HyperCallResult::new(result.unwrap() as i32, hypercall.vm_id, WasiSyscalls::FdWrite)
            }).unwrap();

        }
    }
}