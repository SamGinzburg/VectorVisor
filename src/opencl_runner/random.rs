use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use crate::opencl_runner::interleave_offsets::Interleave;
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::VectorizedVM;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;
use wiggle::wasmtime::WasmtimeGuestMemory;
use wiggle::GuestPtr;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

use crossbeam::channel::Sender;

use std::convert::TryInto;

pub struct Random {}

impl Random {
    #[tokio::main]
    pub async fn hypercall_random_get(
        vm_ctx: &mut VectorizedVM,
        hypercall: &mut HyperCall,
        sender: &Sender<HyperCallResult>,
    ) -> () {
        let mut hcall_buf: &mut [u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        let hcall_buf_size: u32 = vm_ctx.hcall_buf_size;

        let vm_idx = vm_ctx.vm_id;
        let raw_mem: &mut [u8] = vm_ctx.memory.data_mut(&mut vm_ctx.store);

        // If the VM is masked off, don't try to run the syscall
        hcall_buf = &mut hcall_buf
            [(vm_idx * hcall_buf_size) as usize..((vm_idx + 1) * hcall_buf_size) as usize];
        let random_len = LittleEndian::read_u32(&hcall_buf[0..4]);

        let wasm_mem = WasmtimeGuestMemory::new(raw_mem);
        let buf = &GuestPtr::new(&wasm_mem, 0);
        let _result = vm_ctx.ctx.random_get(buf, random_len).await.unwrap();

        // now copy the random data back to the hcall_buffer
        hcall_buf[0..(random_len as usize)].clone_from_slice(&raw_mem[0..(random_len as usize)]);

        sender
            .send(HyperCallResult::new(0, vm_idx, WasiSyscalls::RandomGet))
            .unwrap();
    }
}
