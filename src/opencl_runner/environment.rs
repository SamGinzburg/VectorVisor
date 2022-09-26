// this provides the needed traits for the WASI calls
use crate::opencl_runner::interleave_offsets::Interleave;
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::VectorizedVM;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;
use wasi_common::snapshots::preview_1::types::UserErrorConversion;
use wiggle::wasmtime::WasmtimeGuestMemory;

use wiggle::GuestPtr;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

use crossbeam::channel::Sender;

use std::convert::TryInto;

pub struct Environment {}

impl Environment {
    #[tokio::main]
    pub async fn hypercall_environ_sizes_get(
        vm_ctx: &mut VectorizedVM,
        hypercall: &mut HyperCall,
        sender: &Sender<HyperCallResult>,
    ) -> () {
        let mut hcall_buf: &mut [u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        let vm_idx = vm_ctx.vm_id;
        let result = match vm_ctx.ctx.environ_sizes_get().await {
            Ok(tuple) => {
                // now that we have retreived the sizes
                // now we copy the result to the hcall buf
                let hcall_buf_size: u32 = (hcall_buf.len() / hypercall.num_total_vms as usize)
                    .try_into()
                    .unwrap();
                hcall_buf = &mut hcall_buf[(vm_idx * hcall_buf_size) as usize
                    ..((vm_idx + 1) * hcall_buf_size) as usize];
                LittleEndian::write_u32(&mut hcall_buf[0..4], tuple.0);
                LittleEndian::write_u32(&mut hcall_buf[4..8], tuple.1);
                0
            }
            Err(e) => vm_ctx.ctx.errno_from_error(e).unwrap() as i32,
        };

        sender
            .send(HyperCallResult::new(result, vm_idx, WasiSyscalls::EnvironSizeGet))
            .unwrap();
    }

    #[tokio::main]
    pub async fn hypercall_environ_get(
        vm_ctx: &mut VectorizedVM,
        hypercall: &mut HyperCall,
        sender: &Sender<HyperCallResult>,
    ) -> () {
        let mut hcall_buf: &mut [u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        let hcall_buf_size: u32 = vm_ctx.hcall_buf_size;
        let vm_idx = vm_ctx.vm_id;

        let memory = &vm_ctx.memory;
        let raw_mem: &mut [u8] = memory.data_mut(&mut vm_ctx.store);

        // environ_get is likely to always be called *after* environ_sizes_get, so we can cache the results from that call in the VM object
        let (num_env_vars, env_str_size) =
            match (vm_ctx.enviroment_size, vm_ctx.environment_str_size) {
                (Some(env_size), Some(env_str_size)) => (env_size, env_str_size),
                (_, _) => {
                    // if we haven't cached the values yet, we have to get them
                    vm_ctx.ctx.environ_sizes_get().await.unwrap()
                }
            };

        let wasm_mem = WasmtimeGuestMemory::new(raw_mem);
        let ciovec_ptr = &GuestPtr::new(&wasm_mem, 8);
        let env_str_ptr = &GuestPtr::new(&wasm_mem, 8 + num_env_vars * 4);
        vm_ctx.ctx.environ_get(ciovec_ptr, env_str_ptr).await.unwrap();

        //let arr = &raw_mem[(8 + num_env_vars * 4) as usize..(8 + num_env_vars * 4 + env_str_size) as usize];
        //println!("{}", String::from_utf8(arr.to_vec()).unwrap());
        //println!("{:?}", &arr);

        // copy the results back to the hcall_buf

        hcall_buf = &mut hcall_buf
            [(vm_idx * hcall_buf_size) as usize..((vm_idx + 1) * hcall_buf_size) as usize];
        LittleEndian::write_u32(&mut hcall_buf[0..4], num_env_vars);
        LittleEndian::write_u32(&mut hcall_buf[4..8], env_str_size);
        for idx in 8..(num_env_vars * 4 + env_str_size) {
            hcall_buf[idx as usize] = raw_mem[idx as usize];
        }

        //dbg!(&mut hcall_buf[0..16]);

        sender
            .send(HyperCallResult::new(0, vm_idx, WasiSyscalls::EnvironGet))
            .unwrap();
    }
}
