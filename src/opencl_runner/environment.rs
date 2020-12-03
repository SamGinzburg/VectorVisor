use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use wasi_common::wasi::wasi_snapshot_preview1::WasiSnapshotPreview1;
//use wasi_common::snapshots::wasi_snapshot_preview1::WasiSnapshotPreview1;
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use crate::opencl_runner::interleave_offsets::Interleave;

use wasi_common::wasi::types::CiovecArray;
use wasi_common::fs::Fd;

use wasmtime::*;
use wasmtime_wiggle::WasmtimeGuestMemory;
use wiggle::GuestPtr;

use byteorder::LittleEndian;
use byteorder::ByteOrder;
use crossbeam::channel::Sender;

pub struct Environment {}

impl Environment {
    pub fn hypercall_environ_sizes_get(ctx: &WasiCtx, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &mut [u8] = &mut hypercall.hypercall_buffer.lock().unwrap();
        
        let engine = Engine::default();
        let store = Store::new(&engine);
        let memory_ty = MemoryType::new(Limits::new(1, None));
        let _memory = Memory::new(&store, memory_ty);

        match ctx.environ_sizes_get() {
            Ok(tuple) => {
                // now that we have retreived the sizes
                // now we copy the result to the hcall buf
                if hypercall.is_interleaved_mem {
                    Interleave::write_u32(hcall_buf, 0, hypercall.num_total_vms, tuple.0, hypercall.vm_id);
                    Interleave::write_u32(hcall_buf, 4, hypercall.num_total_vms, tuple.1, hypercall.vm_id);
                } else {
                    hcall_buf = &mut hcall_buf[(hypercall.vm_id * 16384) as usize..((hypercall.vm_id+1) * 16384) as usize];
                    LittleEndian::write_u32(&mut hcall_buf[0..4], tuple.0);
                    LittleEndian::write_u32(&mut hcall_buf[4..8], tuple.1);
                }
            },
            Err(e) => panic!("Unable to execute WASI function environ_sizes_get: {:?}", e),
        }

        sender.send({
            HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::EnvironSizeGet)
        }).unwrap();
    }
}