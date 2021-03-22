use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use crate::opencl_runner::interleave_offsets::Interleave;
use crate::opencl_runner::vectorized_vm::VectorizedVM;

use wasi_common::snapshots::preview_1::types::UserErrorConversion;

use wasmtime::*;
use wasmtime_wiggle::WasmtimeGuestMemory;
use wiggle::GuestPtr;

use byteorder::LittleEndian;
use byteorder::ByteOrder;
use crossbeam::channel::Sender;

use std::convert::TryInto;

pub struct Random {}

impl Random {
    pub fn hypercall_random_get(ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &mut [u8] = &mut hypercall.hypercall_buffer.lock().unwrap();
        
        let memory = &vm_ctx.memory;
        let wasm_mem = &vm_ctx.wasm_memory;
        let raw_mem: &mut [u8] = unsafe { memory.data_unchecked_mut() };

        let random_len = if hypercall.is_interleaved_mem {
            Interleave::read_u32(hcall_buf, 0, hypercall.num_total_vms, hypercall.vm_id)
        } else {
            LittleEndian::read_u32(&hcall_buf[0..4])
        };

        let buf = &GuestPtr::new(&wasm_mem, 0);
        let result = ctx.random_get(buf, random_len).unwrap();

        // now copy the random data back to the hcall_buffer
        if hypercall.is_interleaved_mem {
            for offset in 0..(random_len as usize) {
                Interleave::write_u8(hcall_buf, offset.try_into().unwrap(), hypercall.num_total_vms, raw_mem[offset], hypercall.vm_id);
            }
        } else {
            hcall_buf[0..(random_len as usize)].clone_from_slice(&raw_mem[0..(random_len as usize)]);
        }

        sender.send({
            HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::EnvironGet)
        }).unwrap();
    }
}