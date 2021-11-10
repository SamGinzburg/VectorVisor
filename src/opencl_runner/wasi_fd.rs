use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;

use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use crate::opencl_runner::vectorized_vm::VectorizedVM;
use crate::opencl_runner::interleave_offsets::Interleave;

use wasi_common::snapshots::preview_1::types::CiovecArray;
use wasi_common::snapshots::preview_1::types::Fd;
use wasi_common::snapshots::preview_1::types::Prestat;
use wasi_common::snapshots::preview_1::types::UserErrorConversion;

//use wasi_common::wasi::types::UserErrorConversion;

use wiggle::GuestPtr;

use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use byteorder::ByteOrder;

use crossbeam::channel::Sender;

use std::convert::TryInto;

pub struct WasiFd {}

impl WasiFd {
    pub fn hypercall_fd_write(ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &[u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        let hcall_buf_size: u32 = (hcall_buf.len() / hypercall.num_total_vms as usize).try_into().unwrap();

        let memory = &vm_ctx.memory;
        let wasm_mem = &vm_ctx.wasm_memory;

        let fd: u32;
        let num_iovecs: u32;
        let mut bytes_to_copy: u32 = 0;
        let raw_mem: &mut [u8] = unsafe { memory.data_unchecked_mut() };
        let vm_idx = vm_ctx.vm_id;

        // copy the hypercall buffer over to the memory object
        if hypercall.is_interleaved_mem > 0 {
            fd = Interleave::read_u32(hcall_buf, 0, hypercall.num_total_vms, vm_idx, hypercall.is_interleaved_mem);
            num_iovecs = Interleave::read_u32(hcall_buf, 8, hypercall.num_total_vms, vm_idx, hypercall.is_interleaved_mem);
            // for each iovec, read the buf_len to determine how many bytes to actually copy over
            for idx in 0..num_iovecs {
                bytes_to_copy += Interleave::read_u32(hcall_buf, 16 + 8 * idx + 4, hypercall.num_total_vms, vm_idx, hypercall.is_interleaved_mem);
            }

            // the amount of bytes to copy is the sum of all buf_lens + size of the iovec_arr
            // we account for the 16 byte header too
            bytes_to_copy += 8 * num_iovecs;
            for idx in 16..(16 + bytes_to_copy) as usize {
                raw_mem[idx - 16] = Interleave::read_u8(hcall_buf,
                                                        idx as u32,
                                                        hypercall.num_total_vms,
                                                        vm_idx,
                                                        hypercall.is_interleaved_mem);
            }
        } else {
            // set the buffer to the scratch space for the appropriate VM
            // we don't have to do this for the interleave
            hcall_buf = &hcall_buf[(vm_idx * hcall_buf_size) as usize..((vm_idx+1) * hcall_buf_size) as usize];
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
            HyperCallResult::new(result.unwrap() as i32, vm_idx, WasiSyscalls::FdWrite)
        }).unwrap();
    }

    pub fn hypercall_fd_prestat_get(ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &mut [u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        let hcall_buf_size: u32 = (hcall_buf.len() / hypercall.num_total_vms as usize).try_into().unwrap();
        let vm_idx = vm_ctx.vm_id;

        //let memory = &vm_ctx.memory;
        //let wasm_mem = &vm_ctx.wasm_memory;

        let fd: u32;

        //let raw_mem: &mut [u8] = unsafe { memory.data_unchecked_mut() };
        if hypercall.is_interleaved_mem > 0 {
            fd = Interleave::read_u32(hcall_buf, 0, hypercall.num_total_vms, vm_idx, hypercall.is_interleaved_mem);
        } else {
            // set the buffer to the scratch space for the appropriate VM
            // we don't have to do this for the interleave
            hcall_buf = &mut hcall_buf[(vm_idx * hcall_buf_size) as usize..((vm_idx+1) * hcall_buf_size) as usize];
            fd = LittleEndian::read_u32(&hcall_buf[0..4]);
        }

        let result = match ctx.fd_prestat_get(Fd::from(fd)) {
            Ok(Prestat::Dir(_prestat_dir)) => 0,
            Err(e) => {
                UserErrorConversion::errno_from_error(ctx, e).unwrap() as u32
            },
        };

        if hypercall.is_interleaved_mem > 0 {
            Interleave::write_u32(hcall_buf, 0, hypercall.num_total_vms, result, vm_idx, hypercall.is_interleaved_mem);
        } else {
            LittleEndian::write_u32(&mut hcall_buf[0..4], result);
        }

        sender.send({
            HyperCallResult::new(result as i32, vm_idx, WasiSyscalls::FdPrestatGet)
        }).unwrap();
    }
    pub fn hypercall_fd_prestat_dir_name(ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &mut [u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        let hcall_buf_size: u32 = (hcall_buf.len() / hypercall.num_total_vms as usize).try_into().unwrap();

        //let memory = &vm_ctx.memory;
        let wasm_mem = &vm_ctx.wasm_memory;
        let vm_idx = vm_ctx.vm_id;

        let fd: u32;
        let str_len: u32;


        //let _raw_mem: &mut [u8] = unsafe { memory.data_unchecked_mut() };
        if hypercall.is_interleaved_mem > 0 {
            fd = Interleave::read_u32(hcall_buf, 0, hypercall.num_total_vms, vm_idx, hypercall.is_interleaved_mem);
            str_len = Interleave::read_u32(hcall_buf, 4, hypercall.num_total_vms, vm_idx, hypercall.is_interleaved_mem);

        } else {
            // set the buffer to the scratch space for the appropriate VM
            // we don't have to do this for the interleave
            hcall_buf = &mut hcall_buf[(vm_idx * hcall_buf_size) as usize..((vm_idx+1) * hcall_buf_size) as usize];
            fd = LittleEndian::read_u32(&hcall_buf[0..4]);
            str_len = LittleEndian::read_u32(&hcall_buf[4..8]);
        }

        let str_ptr = &GuestPtr::new(&wasm_mem, 8);
        let result = match ctx.fd_prestat_dir_name(Fd::from(fd), str_ptr, str_len) {
            Ok(()) => 0,
            Err(e) => {
                UserErrorConversion::errno_from_error(ctx, e).unwrap() as u32
            },
        };

        let mut index = 0;
        for idx in str_ptr.as_array(str_len).iter() {
            let value = idx.unwrap().read().unwrap();
            if hypercall.is_interleaved_mem > 0 {
                Interleave::write_u8(hcall_buf, index + 8, hypercall.num_total_vms, value, vm_idx, hypercall.is_interleaved_mem);
            } else {
                let mut hcall_buf_temp = &mut hcall_buf[(index + 8) as usize..(index + 8 + 1) as usize];
                hcall_buf_temp.write_u8(value).unwrap();
            }
            index += 1;
        }

        sender.send({
            HyperCallResult::new(result as i32, vm_idx, WasiSyscalls::FdPrestatDirName)
        }).unwrap();
    }
}
