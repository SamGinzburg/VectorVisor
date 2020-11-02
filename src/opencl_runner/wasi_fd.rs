use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use wasi_common::wasi::wasi_snapshot_preview1::WasiSnapshotPreview1;
//use wasi_common::snapshots::wasi_snapshot_preview1::WasiSnapshotPreview1;
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;

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

use crossbeam::channel::Sender;

pub struct WasiFd {}

impl WasiFd {
    pub fn hypercall_fd_write(ctx: &WasiCtx, hypercall: &HyperCall, sender: &Sender<HyperCallResult>) -> () {
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
            //ctx.fd_write()
        } else {
            // to store the calling parameters
            let mut stack_bytes = [0u32; 4];

            // read the last 4 32 bit values off of the stack

            unsafe {
                ocl::core::enqueue_read_buffer(&hypercall.queue, &hypercall.ocl_buffers.stack_buffer, true, (hypercall.sp-4) as usize, &mut stack_bytes, None::<Event>, None::<&mut Event>).unwrap();
            }

            /*
            [src/opencl_runner/wasi_fd.rs:42] stack_bytes[0] = 1
            [src/opencl_runner/wasi_fd.rs:43] stack_bytes[1] = 8
            [src/opencl_runner/wasi_fd.rs:44] stack_bytes[2] = 1
            [src/opencl_runner/wasi_fd.rs:45] stack_bytes[3] = 12
            * (i32.const 1)   ;; fd 1 (stdout)
            * (i32.const 8)   ;; (iovec*)8
            * (i32.const 1)   ;; 1 iovec
            * (i32.const 12)) ;; write the number of written bytes back to iovec.buf_len
            */

            let fd = stack_bytes[0];
            let num_iovecs = stack_bytes[2] as usize;
            let _nbytes = stack_bytes[3];
            let heap_offset = stack_bytes[1];

            /*
            dbg!(fd);
            dbg!(num_iovecs);
            dbg!(nbytes);
            dbg!(heap_offset);
            */

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

            // read each iovec from the guest VM
            let mut next_buffer_ptr: u32 = 0;
            let mut iovec_buf_lens = vec![];
            let mut iovec_buf_ptrs = vec![];

            for idx in 0..num_iovecs {
                let mut iovec = [0u8; 8];
                unsafe {
                    ocl::core::enqueue_read_buffer(&hypercall.queue, &hypercall.ocl_buffers.heap_buffer, true, (heap_offset) as usize, &mut iovec, None::<Event>, None::<&mut Event>).unwrap();
                }

                iovec_buf_ptrs.push((&iovec[0..4]).read_u32::<LittleEndian>().unwrap());
                iovec_buf_lens.push((&iovec[4..8]).read_u32::<LittleEndian>().unwrap());

                // we allocate the top of the raw mem for the ciovec array
                for byte_idx in 0..iovec.len() {
                    raw_mem[byte_idx + (idx*8)] = iovec[byte_idx];
                    next_buffer_ptr = ((idx+1)*8) as u32;
                }
            }

            // now that we have allocated all of the ciovec array, put the buffers immediately after it
            // we also need to update the buf_ptrs to point at the new buffer locations
            for idx in 0..num_iovecs {
                let buf_len_u32 = iovec_buf_lens[idx];
                // copy the buffer into the guest memory object
                let mut buffer_write = &mut raw_mem[(next_buffer_ptr as u32) as usize..(next_buffer_ptr as u32 + buf_len_u32) as usize];
                unsafe {
                    ocl::core::enqueue_read_buffer(&hypercall.queue, &hypercall.ocl_buffers.heap_buffer, true, (iovec_buf_ptrs[idx]) as usize, &mut buffer_write, None::<Event>, None::<&mut Event>).unwrap();
                }

                // write back the *new* buf ptr
                LittleEndian::write_u32(&mut raw_mem[(idx*8)..(idx*8)+4], next_buffer_ptr);

                next_buffer_ptr += buf_len_u32;
            }

            // after all reads/writes are done, wrap the memory with the GuestMemory trait
            let wasm_mem = WasmtimeGuestMemory::new(memory);

            // we hardcode the ciovec array to start at offset 0
            let ciovec_ptr: &CiovecArray = &GuestPtr::new(&wasm_mem, (0 as u32, num_iovecs as u32));

            let result = ctx.fd_write(Fd::from(fd), &ciovec_ptr);

            sender.send({
                HyperCallResult::new(result.unwrap() as i32, hypercall.vm_id, WasiSyscalls::FdWrite)
            }).unwrap();

        }
    }
}