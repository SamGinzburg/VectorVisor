use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use wasi_common::wasi::wasi_snapshot_preview1::WasiSnapshotPreview1;

use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use crate::opencl_runner::vectorized_vm::VectorizedVM;
use crate::opencl_runner::interleave_offsets::Interleave;

use wasi_common::wasi::types::CiovecArray;
use wasi_common::fs::Fd;
use wasi_common::wasi::types::PrestatDir;
use wasi_common::wasi::types::Prestat;
use wasi_common::wasi::types::UserErrorConversion;

use wasmtime::*;
use wasmtime_wiggle::WasmtimeGuestMemory;
use wiggle::GuestPtr;

use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use byteorder::ByteOrder;
use crossbeam::channel::Sender;

pub struct WasiFd {}

impl WasiFd {
    pub fn hypercall_fd_write(ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &mut [u8] = &mut hypercall.hypercall_buffer.lock().unwrap();

        let memory = &vm_ctx.memory;
        let wasm_mem = &vm_ctx.wasm_memory;

        let fd: u32;
        let num_iovecs: u32;
        let mut bytes_to_copy: u32 = 0;
        let raw_mem: &mut [u8] = unsafe { memory.data_unchecked_mut() };

        // copy the hypercall buffer over to the memory object
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

        //dbg!(&raw_mem[0..64]);

        // we hardcode the ciovec array to start at offset 0
        let ciovec_ptr: &CiovecArray = &GuestPtr::new(&wasm_mem, (0 as u32, num_iovecs as u32));
        let result = ctx.fd_write(Fd::from(fd), &ciovec_ptr);

        sender.send({
            HyperCallResult::new(result.unwrap() as i32, hypercall.vm_id, WasiSyscalls::FdWrite)
        }).unwrap();
    }

    pub fn hypercall_fd_prestat_get(ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &mut [u8] = &mut hypercall.hypercall_buffer.lock().unwrap();

        let memory = &vm_ctx.memory;
        let wasm_mem = &vm_ctx.wasm_memory;

        let fd: u32;

        let raw_mem: &mut [u8] = unsafe { memory.data_unchecked_mut() };
        if hypercall.is_interleaved_mem {
            fd = Interleave::read_u32(hcall_buf, 0, hypercall.num_total_vms, hypercall.vm_id);
        } else {
            // set the buffer to the scratch space for the appropriate VM
            // we don't have to do this for the interleave
            hcall_buf = &mut hcall_buf[(hypercall.vm_id * 16384) as usize..((hypercall.vm_id+1) * 16384) as usize];
            fd = LittleEndian::read_u32(&hcall_buf[0..4]);
        }

        let result = match ctx.fd_prestat_get(Fd::from(fd)) {
            Ok(Prestat::Dir(prestat_dir)) => 0,
            Err(e) => {
                UserErrorConversion::errno_from_error(ctx, e).unwrap() as u32
            },
        };

        if hypercall.is_interleaved_mem {
            Interleave::write_u32(hcall_buf, 0, hypercall.num_total_vms, result, hypercall.vm_id);
        } else {
            LittleEndian::write_u32(&mut hcall_buf[0..4], result);
        }

        sender.send({
            HyperCallResult::new(result as i32, hypercall.vm_id, WasiSyscalls::FdPrestatGet)
        }).unwrap();
    }
    pub fn hypercall_fd_prestat_dir_name(ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &mut [u8] = &mut hypercall.hypercall_buffer.lock().unwrap();

        let memory = &vm_ctx.memory;
        let wasm_mem = &vm_ctx.wasm_memory;

        let fd: u32;
        let str_len: u32;


        let raw_mem: &mut [u8] = unsafe { memory.data_unchecked_mut() };
        if hypercall.is_interleaved_mem {
            fd = Interleave::read_u32(hcall_buf, 0, hypercall.num_total_vms, hypercall.vm_id);
            str_len = Interleave::read_u32(hcall_buf, 4, hypercall.num_total_vms, hypercall.vm_id);

        } else {
            // set the buffer to the scratch space for the appropriate VM
            // we don't have to do this for the interleave
            hcall_buf = &mut hcall_buf[(hypercall.vm_id * 16384) as usize..((hypercall.vm_id+1) * 16384) as usize];
            fd = LittleEndian::read_u32(&hcall_buf[0..4]);
            str_len = LittleEndian::read_u32(&hcall_buf[4..8]);
        }

        let mut str_ptr = &GuestPtr::new(&wasm_mem, 8);
        let result = match ctx.fd_prestat_dir_name(Fd::from(fd), str_ptr, str_len) {
            Ok(()) => 0,
            Err(e) => {
                UserErrorConversion::errno_from_error(ctx, e).unwrap() as u32
            },
        };

        let mut index = 0;
        for idx in str_ptr.as_array(str_len).iter() {
            let value = idx.unwrap().read().unwrap();
            if hypercall.is_interleaved_mem {
                Interleave::write_u8(hcall_buf, index + 8, hypercall.num_total_vms, value, hypercall.vm_id);
            } else {
                let mut hcall_buf_temp = &mut hcall_buf[(index + 8) as usize..(index + 8 + 1) as usize];
                hcall_buf_temp.write_u8(value);
            }
            index += 1;
        }

        sender.send({
            HyperCallResult::new(result as i32, hypercall.vm_id, WasiSyscalls::FdPrestatDirName)
        }).unwrap();
    }
}