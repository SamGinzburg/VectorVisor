use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use wasi_common::wasi::wasi_snapshot_preview1::WasiSnapshotPreview1;
//use wasi_common::snapshots::wasi_snapshot_preview1::WasiSnapshotPreview1;
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use crate::opencl_runner::interleave_offsets::Interleave;
use crate::opencl_runner::vectorized_vm::VectorizedVM;

use wasi_common::wasi::types::CiovecArray;
use wasi_common::fs::Fd;
use wasi_common::wasi::types::UserErrorConversion;

use wasmtime::*;
use wasmtime_wiggle::WasmtimeGuestMemory;
use wiggle::GuestPtr;

use byteorder::LittleEndian;
use byteorder::ByteOrder;
use crossbeam::channel::Sender;

pub struct Serverless {}

impl Serverless {
    pub fn hypercall_serverless_invoke(ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &mut [u8] = &mut hypercall.hypercall_buffer.lock().unwrap();

        // block until we get an incoming request
        let mut recv_chan = (vm_ctx.vm_recv).clone();

        loop {
            let mut recv_chan_lock = recv_chan.try_lock();
            match recv_chan_lock {
                Ok(mutex) => {
                    // got the lock, quickly get the message
                    //dbg!("waiting on recv...");
                    let res = match mutex.try_recv() {
                        Ok(value) => {
                            // we got an incoming request
                        },
                        Err(e) => {
                            //dbg!("waiting...");
                            continue;
                        }
                    };
                    dbg!(res);
                    break;
                },
                Err(e) => {
                    // couldn't get the lock
                    dbg!("blocking now!");
                }
            }
        }

        dbg!("requests recevied!");
        
        // copy the incoming request into the hcall_buffer
        if hypercall.is_interleaved_mem {

        } else {

        }

        sender.send({
            HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::ServerlessInvoke)
        }).unwrap();
    }


    pub fn hypercall_serverless_response(ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &mut [u8] = &mut hypercall.hypercall_buffer.lock().unwrap();

        (*vm_ctx.vm_sender).lock().unwrap().send(0).unwrap();

        sender.send({
            HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::ServerlessResponse)
        }).unwrap();
    }


}