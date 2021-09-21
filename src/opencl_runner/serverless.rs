use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use crate::opencl_runner::vectorized_vm::VectorizedVM;

use byteorder::LittleEndian;
use byteorder::ByteOrder;

use crossbeam::channel::Sender;

use std::convert::TryInto;
use std::sync::Arc;

use serde_json::Value;
use std::time;
use std::thread;
use std::sync::atomic::Ordering;

pub struct Serverless {}

impl Serverless {
    pub fn hypercall_serverless_invoke(vm_ctx: &mut VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
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

        /*
        let sleep_time = time::Duration::from_millis(10);
        // return the length of the incoming message
        while vm_ctx.ready_for_input.load(Ordering::Relaxed) == true {
            thread::sleep(sleep_time);
        }
        */

        sender.send({
            HyperCallResult::new(vm_ctx.input_msg_len.try_into().unwrap(), hypercall.vm_id, WasiSyscalls::ServerlessInvoke)
        }).unwrap();
    }


    pub fn hypercall_serverless_response(_ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let mut hcall_buf: &[u8] = unsafe { *hypercall.hypercall_buffer.buf.get() };
        let hcall_buf_size: u32 = vm_ctx.hcall_buf_size;
        hcall_buf = &hcall_buf[(hypercall.vm_id * hcall_buf_size) as usize..((hypercall.vm_id+1) * hcall_buf_size) as usize];

        let send_chan = (vm_ctx.vm_sender).get(hypercall.vm_id as usize).unwrap();
        // the first 4 bytes are the length as a u32, the remainder is the buffer containing the json

        let msg_len = LittleEndian::read_u32(&hcall_buf[0..4]);
        let resp_buf_len: usize = msg_len.try_into().unwrap();

        let mut resp_buf = vec![0u8; (resp_buf_len+4).try_into().unwrap()];

        // copy the data from the hcall_buffer
        //println!("buf len: {:?}, vmidx: {:?}", resp_buf_len, hypercall.vm_id);
        resp_buf[0..resp_buf_len].copy_from_slice(&hcall_buf[4..4+resp_buf_len]);

        // calculate on device time and queue submit times
        let on_device_time = hypercall.timestamp_counter - *vm_ctx.timestamp_counter;
        let queue_submit_time = hypercall.queue_submit_delta - *vm_ctx.queue_submit_counter;
        let queue_submit_count = hypercall.num_queue_submits - *vm_ctx.queue_submit_qty;

        // compute set difference for the called fns
        let mut count = 0;
        for _idx in vm_ctx.called_fns_set.intersection(&hypercall.called_fns) {
            count += 1;
        }

        send_chan.lock().unwrap().blocking_send((resp_buf, resp_buf_len, on_device_time, queue_submit_time, queue_submit_count, count)).unwrap();

        // Perform async replies, no need to block in the critical path
        /*
        sender.send({
            HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::ServerlessResponse)
        }).unwrap();
        */
    }

}
