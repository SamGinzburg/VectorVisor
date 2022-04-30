use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use crate::opencl_runner::interleave_offsets::Interleave;
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::VectorizedVM;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;

use wiggle::GuestPtr;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

use crossbeam::channel::Sender;

use std::convert::TryInto;

pub struct Random {}

impl Random {
    pub fn hypercall_random_get(
        ctx: &WasiCtx,
        vm_ctx: &VectorizedVM,
        hypercall: &mut HyperCall,
        sender: &Sender<HyperCallResult>,
    ) -> () {
        let mut hcall_buf: &mut [u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        let hcall_buf_size: u32 = vm_ctx.hcall_buf_size;

        let memory = &vm_ctx.memory;
        let wasm_mem = &vm_ctx.wasm_memory;
        let vm_idx = vm_ctx.vm_id;
        let raw_mem: &mut [u8] = unsafe { memory.data_unchecked_mut() };

        // If the VM is masked off, don't try to run the syscall
        if !vm_ctx.no_resp {
            let random_len = if hypercall.is_interleaved_mem > 0 {
                Interleave::read_u32(
                    hcall_buf,
                    0,
                    hypercall.num_total_vms,
                    vm_idx,
                    hypercall.is_interleaved_mem,
                )
            } else {
                LittleEndian::read_u32(&hcall_buf[0..4])
            };

            let buf = &GuestPtr::new(&wasm_mem, 0);
            let _result = ctx.random_get(buf, random_len).unwrap();

            // now copy the random data back to the hcall_buffer
            if hypercall.is_interleaved_mem > 0 {
                for offset in 0..(random_len as usize) {
                    Interleave::write_u8(
                        hcall_buf,
                        offset.try_into().unwrap(),
                        hypercall.num_total_vms,
                        raw_mem[offset],
                        vm_idx,
                        hypercall.is_interleaved_mem,
                    );
                }
            } else {
                hcall_buf = &mut hcall_buf
                    [(vm_idx * hcall_buf_size) as usize..((vm_idx + 1) * hcall_buf_size) as usize];
                hcall_buf[0..(random_len as usize)]
                    .clone_from_slice(&raw_mem[0..(random_len as usize)]);
            }
        }

        sender
            .send({ HyperCallResult::new(0, vm_idx, WasiSyscalls::RandomGet) })
            .unwrap();
    }
}
