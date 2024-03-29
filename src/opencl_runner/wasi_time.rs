use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use crate::opencl_runner::interleave_offsets::Interleave;
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::VectorizedVM;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use wasi_common::snapshots::preview_1::types::Clockid;
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;

use wiggle::GuestPtr;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

use crossbeam::channel::Sender;

use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Clock {}

impl Clock {
    #[tokio::main]
    pub async fn hypercall_clock_time_get(
        vm_ctx: &mut VectorizedVM,
        hypercall: &mut HyperCall,
        sender: &Sender<HyperCallResult>,
    ) -> () {
        let mut hcall_buf: &mut [u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        let hcall_buf_size: u32 = vm_ctx.hcall_buf_size;

        let vm_idx = vm_ctx.vm_id;
        //let raw_mem: &mut [u8] = vm_ctx.memory.data_mut(&mut vm_ctx.store);

        // If the VM is masked off, don't try to run the syscall
        hcall_buf = &mut hcall_buf
            [(vm_idx * hcall_buf_size) as usize..((vm_idx + 1) * hcall_buf_size) as usize];

        let clock_id = LittleEndian::read_u32(&hcall_buf[0..4]);
        let precision = LittleEndian::read_u64(&hcall_buf[4..12]);

        //dbg!(&clock_id);
        //dbg!(&precision);

        let timestamp = vm_ctx
            .ctx
            .clock_time_get(Clockid::try_from(clock_id as i32).unwrap(), precision)
            .await
            .unwrap();
        // now copy the random data back to the hcall_buffer
        hcall_buf[0..8].clone_from_slice(&timestamp.to_le_bytes());

        sender
            .send(HyperCallResult::new(0, vm_idx, WasiSyscalls::ClockTimeGet))
            .unwrap();
    }
}
