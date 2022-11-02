use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::VectorizedVM;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

use tokio::sync::mpsc::Sender;

use std::convert::TryInto;
use std::sync::Arc;

use serde_json::Value;
use std::sync::atomic::Ordering;
use std::thread;
use std::time;
use std::collections::VecDeque;
use std::sync::Mutex;
use tokio::sync::Mutex as AsyncMutex;
use crate::VmSenderType;

pub struct Serverless {}

impl Serverless {
    pub async fn hypercall_serverless_invoke<'a>(
        vm_ctx: &'a mut VectorizedVM,
        hypercall: &'a mut HyperCall<'_>,
        sender: &'a Sender<HyperCallResult>,
    ) -> () {
        // If other non-invoke calls need to be dispatched, perform a no-op and return.
        // This call will be executed later when ready
        if hypercall.non_serverless_invoke_call_found {
            return;
        }

        // store this in the vmctx for when we return
        *Arc::make_mut(&mut vm_ctx.timestamp_counter) = hypercall.timestamp_counter;
        *Arc::make_mut(&mut vm_ctx.queue_submit_counter) = hypercall.queue_submit_delta;
        *Arc::make_mut(&mut vm_ctx.queue_submit_qty) = hypercall.num_queue_submits;

        // only on first invoke do we want to update this
        if vm_ctx.called_fns_set.len() == 0 {
            for item in &hypercall.called_fns {
                Arc::make_mut(&mut vm_ctx.called_fns_set).insert(*item);
            }
        }

        if vm_ctx.input_msg_len > 0 {
            vm_ctx.no_resp = false;
        } else {
            vm_ctx.no_resp = true;
        }

        let vm_idx = vm_ctx.vm_id;
        sender
            .send({
                HyperCallResult::new(
                    vm_ctx.input_msg_len.try_into().unwrap(),
                    vm_idx,
                    WasiSyscalls::ServerlessInvoke,
                )
            }).await
            .unwrap();

        // reset msg len for the next request
        vm_ctx.input_msg_len = 0;
    }

    pub async fn hypercall_serverless_response<'a>(
        //vm_ctx: &'a VectorizedVM,
        //hypercall: &mut HyperCall,
        vm_sender: Arc<Vec<AsyncMutex<Sender<VmSenderType>>>>,
        hcall_buf_size: u32,
        vm_idx: u32,
        no_resp: bool,
        uuid_queue: &'a mut VecDeque<String>,
        mut hcall_buf: &'static [u8],
        overhead_buf: &'static [u64],
        sender: &'a Sender<HyperCallResult>,
    ) -> () {
        //let mut hcall_buf: &'static [u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        //let mut overhead_buf: &'static [u64] = unsafe { *hypercall.overhead_tracker.buf.get() };

        //let hcall_buf_size: u32 = vm_ctx.hcall_buf_size;
        //let vm_idx = vm_ctx.vm_id;
        hcall_buf = &hcall_buf
            [(vm_idx * hcall_buf_size) as usize..((vm_idx + 1) * hcall_buf_size) as usize];
        if !no_resp {
            let send_chan = vm_sender.get(vm_idx as usize).unwrap();
            // the first 4 bytes are the length as a u32, the remainder is the buffer containing the json

            let msg_len = LittleEndian::read_u32(&hcall_buf[0..4]);
            let resp_buf_len: usize = msg_len.try_into().unwrap();

            // Making a zero-copy reference to the hcall_buffer is safe here because:
            // 1) We ensure that the next hcall after serverless_response is *always*
            //    serverless_invoke.
            // 2) Inputs are buffered separately from responses so we won't overwrite
            // 3) The only edge case is if we finish our response, accept a new request,
            //    and start performing a different hcall before our server replies.
            //    **However**, this can't happen by design, since our server won't accept
            //    a new request for a VM until the previous request is completed (lock released).
            let resp_buf = bytes::Bytes::from_static(&hcall_buf[4..4 + resp_buf_len]);

            // calculate on device time and queue submit times
            /*
            let on_device_time = hypercall.timestamp_counter - *vm_ctx.timestamp_counter;
            let queue_submit_time = hypercall.queue_submit_delta - *vm_ctx.queue_submit_counter;
            let queue_submit_count = hypercall.num_queue_submits - *vm_ctx.queue_submit_qty;
    
            // compute set difference for the called fns
            let mut count = 0;
            for _idx in vm_ctx.called_fns_set.intersection(&hypercall.called_fns) {
                count += 1;
            }
            */
            let on_device_time = 0;
            let queue_submit_time = 0;
            let queue_submit_count = 0;
            let mut count = 1;

            send_chan
                .lock().await
                .send((
                    resp_buf,
                    resp_buf_len,
                    on_device_time,
                    queue_submit_time,
                    queue_submit_count,
                    count,
                    overhead_buf[vm_idx as usize],
                    uuid_queue.pop_front().unwrap(),
                )).
                await.unwrap();
        }

        sender
            .send({ HyperCallResult::new(0, vm_idx, WasiSyscalls::ServerlessResponse) }).await
            .unwrap();
    }
}
