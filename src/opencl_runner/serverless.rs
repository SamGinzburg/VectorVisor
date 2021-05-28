use wasi_common::WasiCtx;
// this provides the needed traits for the WASI calls
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;
use crate::opencl_runner::vectorized_vm::HyperCall;
use crate::opencl_runner::vectorized_vm::HyperCallResult;
use crate::opencl_runner::vectorized_vm::WasiSyscalls;
use crate::opencl_runner::interleave_offsets::Interleave;
use crate::opencl_runner::vectorized_vm::VectorizedVM;

use byteorder::LittleEndian;
use byteorder::ByteOrder;
use crossbeam::channel::Sender;

use std::convert::TryInto;
use std::sync::Arc;

pub struct Serverless {}

impl Serverless {
    pub fn hypercall_serverless_invoke(vm_ctx: &mut VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let hcall_buf: &mut [u8] = &mut hypercall.hypercall_buffer.lock().unwrap();

        // block until we get an incoming request
        let recv_chan = (vm_ctx.vm_recv).clone();

        let (msg, msg_len) = recv_chan.lock().unwrap().recv().unwrap();

        // copy the incoming request into the hcall_buffer
        if hypercall.is_interleaved_mem {
            for offset in 0..msg_len {
                Interleave::write_u8(hcall_buf, offset.try_into().unwrap(), hypercall.num_total_vms, msg[offset], hypercall.vm_id);
            }
        } else {
            hcall_buf[0..msg_len].copy_from_slice(&msg[0..msg_len]);
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

        // return msg_len
        sender.send({
            HyperCallResult::new(msg_len.try_into().unwrap(), hypercall.vm_id, WasiSyscalls::ServerlessInvoke)
        }).unwrap();
    }


    pub fn hypercall_serverless_response(_ctx: &WasiCtx, vm_ctx: &VectorizedVM, hypercall: &mut HyperCall, sender: &Sender<HyperCallResult>) -> () {
        let hcall_buf: &mut [u8] = &mut hypercall.hypercall_buffer.lock().unwrap();

        let mut resp_buf = vec![0u8; vm_ctx.hcall_buf_size.try_into().unwrap()];
        // the first 4 bytes are the length as a u32, the remainder is the buffer containing the json
        let msg_len = if hypercall.is_interleaved_mem {
            Interleave::read_u32(hcall_buf, 0, hypercall.num_total_vms, hypercall.vm_id)
        } else {
            LittleEndian::read_u32(&hcall_buf[0..4])
        };

        let resp_buf_len: usize = msg_len.try_into().unwrap();

        // copy the data from the hcall_buffer
        if hypercall.is_interleaved_mem {
            for offset in 0..resp_buf_len {
                resp_buf[offset] = Interleave::read_u8(hcall_buf, (4 + offset).try_into().unwrap(), hypercall.num_total_vms, hypercall.vm_id);
            }
        } else {
            resp_buf[0..resp_buf_len].copy_from_slice(&hcall_buf[4..4+resp_buf_len]);
        }

        // calculate on device time and queue submit times
        let on_device_time = hypercall.timestamp_counter - *vm_ctx.timestamp_counter;
        let queue_submit_time = hypercall.queue_submit_delta - *vm_ctx.queue_submit_counter;
        let queue_submit_count = hypercall.num_queue_submits - *vm_ctx.queue_submit_qty;

        // compute set difference for the called fns
        let mut count = 0;
        for _idx in vm_ctx.called_fns_set.intersection(&hypercall.called_fns) {
            count += 1;
        }

        (*vm_ctx.vm_sender).lock().unwrap().send((resp_buf.to_vec(), resp_buf_len, on_device_time, queue_submit_time, queue_submit_count, count)).unwrap();

        sender.send({
            HyperCallResult::new(0, hypercall.vm_id, WasiSyscalls::ServerlessResponse)
        }).unwrap();
    }

}
