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
        let mut hcall_buf: &mut [u8] = &mut hypercall.hypercall_buffer.lock().unwrap();

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

        let fd: u32;
        let num_iovecs: u32;
        let mut bytes_to_copy: u32 = 0;
        let raw_mem: &mut [u8] = unsafe { memory.data_unchecked_mut() };

        // copy the hypercall buffer over to the memory object
        // TODO: we can optimize this later, to read minimal amounts of memory
        // Further TODO: we also need to create a generic interleaved read/write function for this
        if hypercall.is_interleaved_mem {
            fd = Interleave::read_u32(hcall_buf, 0, hypercall.num_total_vms, hypercall.vm_id);
            num_iovecs = Interleave::read_u32(hcall_buf, 8, hypercall.num_total_vms, hypercall.vm_id);

            // for each iovec, read the buf_len to determine how many bytes to actually copy over
            for idx in 0..num_iovecs {
                bytes_to_copy += Interleave::read_u32(hcall_buf, 16 + 8 * idx + 4, hypercall.num_total_vms, hypercall.vm_id);
            }

            // the amount of bytes to copy is the sum of all buf_lens + size of the iovec_arr
            // we account for the 16 byte header too
            bytes_to_copy += 8 * num_iovecs;
            for idx in 16..(16 + bytes_to_copy) as usize {
                raw_mem[idx - 16] = Interleave::read_u8(hcall_buf,
                                                        idx as u32,
                                                        hypercall.num_total_vms,
                                                        hypercall.vm_id);
            }
        } else {
            // set the buffer to the scratch space for the appropriate VM
            // we don't have to do this for the interleave
            hcall_buf = &mut hcall_buf[(hypercall.vm_id * 16384) as usize..((hypercall.vm_id+1) * 16384) as usize];
            fd = LittleEndian::read_u32(&hcall_buf[0..4]);
            num_iovecs = LittleEndian::read_u32(&hcall_buf[8..12]);

            // for each iovec, read the buf_len to determine how many bytes to actually copy over
            for idx in 0..num_iovecs {
                let offset: usize = (16 + 8 * idx + 4) as usize;
                bytes_to_copy += LittleEndian::read_u32(&hcall_buf[offset..offset+4]);
            }

            // the amount of bytes to copy is the sum of all buf_lens + size of the iovec_arr
            // we account for the 16 byte header too
            bytes_to_copy += 8 * num_iovecs;

            for idx in 16..(16 + bytes_to_copy) as usize {
                raw_mem[idx - 16] = hcall_buf[idx];
            }
        }

        // after all reads/writes are done, wrap the memory with the GuestMemory trait
        let wasm_mem = WasmtimeGuestMemory::new(memory);

        // we hardcode the ciovec array to start at offset 0
        let ciovec_ptr: &CiovecArray = &GuestPtr::new(&wasm_mem, (0 as u32, 1 as u32));

        let result = ctx.fd_write(Fd::from(fd), &ciovec_ptr);

        sender.send({
            HyperCallResult::new(result.unwrap() as i32, hypercall.vm_id, WasiSyscalls::FdWrite)
        }).unwrap();
    }
}