use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::VectorizedVM;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

use crossbeam::channel::Sender;

use std::convert::TryInto;
use std::sync::Arc;

use serde_json::Value;
use std::sync::atomic::Ordering;
use std::thread;
use std::time;

pub struct Serverless {}

impl Serverless {
    pub fn hypercall_serverless_invoke(
        vm_ctx: &mut VectorizedVM,
        hypercall: &mut HyperCall,
        sender: &Sender<HyperCallResult>,
    ) -> () {
        // If other non-invoke calls need to be dispatched, perform a no-op and return.
        // This call will be executed later when ready
        if hypercall.non_serverless_invoke_call_found {
            vm_ctx.no_resp = true;
            sender
            .send({
                HyperCallResult::new(
                    0,
                    hypercall.vm_id,
                    WasiSyscalls::ServerlessInvoke,
                )
            })
            .unwrap();
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
        assert!(vm_idx == hypercall.vm_id);
        sender
            .send({
                HyperCallResult::new(
                    vm_ctx.input_msg_len.try_into().unwrap(),
                    hypercall.vm_id,
                    WasiSyscalls::ServerlessInvoke,
                )
            })
            .unwrap();

        vm_ctx.input_msg_len = 0;
    }

    pub fn hypercall_serverless_response(
        vm_ctx: &mut VectorizedVM,
        hypercall: &mut HyperCall,
        sender: &Sender<HyperCallResult>,
    ) -> () {
        let mut hcall_buf: &'static [u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        let mut overhead_buf: &'static [u64] = unsafe { *hypercall.overhead_tracker.buf.get() };

        let hcall_buf_size: u32 = vm_ctx.hcall_buf_size;
        let vm_idx = vm_ctx.vm_id;
        hcall_buf = &hcall_buf
            [(vm_idx * hcall_buf_size) as usize..((vm_idx + 1) * hcall_buf_size) as usize];
        let msg_len = LittleEndian::read_u32(&hcall_buf[0..4]);
        //dbg!(&msg_len);
        if msg_len > 0 && !vm_ctx.no_resp {
            vm_ctx.no_resp = true;
            let (send_chan1, send_chan2) = (vm_ctx.vm_sender).get(vm_idx as usize).unwrap();
            // the first 4 bytes are the length as a u32, the remainder is the buffer containing the json

            let msg_len = LittleEndian::read_u32(&hcall_buf[0..4]);
            let resp_buf_len: usize = msg_len.try_into().unwrap();

            let mut resp_buf = vec![0u8; resp_buf_len.try_into().unwrap()];
            //let resp_buf = bytes::Bytes::from(&hcall_buf[4..4 + resp_buf_len]);
            // copy the data from the hcall_buffer
            resp_buf[0..resp_buf_len].copy_from_slice(&hcall_buf[4..4+resp_buf_len]);
            let resp_buf = bytes::Bytes::from(resp_buf);

            // calculate on device time and queue submit times
            let on_device_time = hypercall.timestamp_counter - *vm_ctx.timestamp_counter;
            let queue_submit_time = hypercall.queue_submit_delta - *vm_ctx.queue_submit_counter;
            let queue_submit_count = hypercall.num_queue_submits - *vm_ctx.queue_submit_qty;

            // compute set difference for the called fns
            let mut count = 0;
            for _idx in vm_ctx.called_fns_set.intersection(&hypercall.called_fns) {
                count += 1;
            }
            let (uuid, chan_id) = vm_ctx.uuid_queue.pop_front().unwrap();
            if chan_id == 0 {
                send_chan1
                    .lock()
                    .unwrap()
                    .blocking_send((
                        resp_buf,
                        resp_buf_len,
                        on_device_time,
                        queue_submit_time,
                        queue_submit_count,
                        count,
                        overhead_buf[vm_idx as usize],
                        uuid,
                    ))
                    .unwrap();
            } else {
                send_chan2
                    .lock()
                    .unwrap()
                    .blocking_send((
                        resp_buf,
                        resp_buf_len,
                        on_device_time,
                        queue_submit_time,
                        queue_submit_count,
                        count,
                        overhead_buf[vm_idx as usize],
                        uuid,
                    ))
                    .unwrap();
            }
        }

        assert!(vm_idx == hypercall.vm_id);
        sender
            .send({ HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::ServerlessResponse) })
            .unwrap();

        // Perform async replies, no need to block in the critical path
        /*
        sender.send({
            HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::ServerlessResponse)
        }).unwrap();
        */
    }
}
