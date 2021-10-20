use std::str::from_utf8;
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Arc;
use std::sync::Mutex as SyncMutex;
use std::net::{SocketAddr};
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::Mutex;

use serde::Deserialize;
use serde::Serialize;

use warp::{Filter, Reply};
use warp::http::{Response, StatusCode};
use hyper::Body;

use crate::opencl_runner::vectorized_vm::VmSenderType;

pub struct BatchSubmitServer {}

#[derive(Debug, Deserialize)]
struct FunctionInput {
    req: String,
    req_id: u32
}

#[derive(Debug, Deserialize)]
struct BatchInput {
    requests: Vec<FunctionInput>
}

#[derive(Debug, Serialize)]
struct BatchReply<'a> {
    response: &'a [u8],
    on_device_execution_time_ns: u64,
    device_queue_overhead_time_ns: u64,
    queue_submit_count: u64,
    num_unique_fns_called: u64,
}

#[derive(Debug, Serialize)]
struct BatchResponse<'a> {
    requests: HashMap<u32, BatchReply<'a>>
}

type VmQueue = deadqueue::limited::Queue<usize>;

#[derive(Debug)]
struct NoVmAvailable;

impl warp::reject::Reject for NoVmAvailable {}

impl BatchSubmitServer {

    fn create_response(resp: Vec<u8>, on_dev_time: u64, queue_submit_time: u64, num_queue_submits: u64, num_unique_fns: u64) -> warp::http::Response<Body> {
        let mut final_resp = Response::builder().status(StatusCode::OK);
        {
            let headers = final_resp.headers_mut().unwrap();
            headers.insert("on_device_time", warp::http::HeaderValue::from_str(&on_dev_time.to_string()).unwrap());
            headers.insert("queue_submit_time", warp::http::HeaderValue::from_str(&queue_submit_time.to_string()).unwrap());
            headers.insert("num_queue_submits", warp::http::HeaderValue::from_str(&num_queue_submits.to_string()).unwrap());
            headers.insert("num_unique_fns", warp::http::HeaderValue::from_str(&num_unique_fns.to_string()).unwrap());
        }

        final_resp.body(Body::from(resp)).unwrap()
    }


    async fn response(body: bytes::Bytes, fast_reply: bool, vm_idx: usize, vm_queue: Arc<VmQueue>, sender: Arc<Vec<Mutex<Sender<(bytes::Bytes, usize)>>>>, receiver: Arc<Vec<Mutex<Receiver<VmSenderType>>>>) -> Result<impl warp::Reply, warp::Rejection> {

        //dbg!(&vm_idx);
        // Get an available VM first
        let tx: &Mutex<Sender<(bytes::Bytes, usize)>> = (*sender).get(vm_idx).unwrap();
        let rx: &Mutex<Receiver<VmSenderType>> = (*receiver).get(vm_idx).unwrap();

        /*
        let (tx, rx, vm_idx) = match vm_queue.try_pop() {
            Some(idx) => {
                ((*sender).get(idx).unwrap(), (*receiver).get(idx).unwrap(), idx)
            },
            // TODO, if we have no available GPU workers, try using backup CPU resources
            None => return Err(warp::reject::custom(NoVmAvailable)),
        };
        */

        // Send the request body to the selected VM
        let req_start = std::time::Instant::now();
        {
            let sender = tx.lock().await;
            sender.send((body.clone(), body.len())).await.unwrap();
        }

        /*
         * Tokio locks operate in a FIFO order, so the first request to send will also
         * subsequenly be the first to receive.
         * see: https://docs.rs/tokio/1.12.0/tokio/sync/struct.Mutex.html
         *
         * This means that the first person to acquire the previous lock on 'sender' will be the
         * first to acquire the lock on recv before yielding to the tokio scheduler. So reqs will
         * line up properly. This also allows multiple requests to queue up on VMs.
         */

        // Wait on response from the VM
        let (resp, len, on_dev_time, queue_submit_time, num_queue_submits, num_unique_fns) = match rx.lock().await.recv().await {
            Some(val) => val,
            None => panic!("A VM died while processing a request, vm_idx: {}", vm_idx),
        };
        let req_end = std::time::Instant::now();
        //println!("req time: {:?}, vm_idx: {:?}", (req_end - req_start).as_nanos(), vm_idx);

        /*
        if fast_reply {
            Ok(BatchSubmitServer::create_response(resp, on_dev_time, queue_submit_time, num_queue_submits, num_unique_fns))
        } else {
            let final_response = BatchReply {
                response: &resp,
                on_device_execution_time_ns: on_dev_time,
                device_queue_overhead_time_ns: queue_submit_time,
                queue_submit_count: num_queue_submits,
                num_unique_fns_called: num_unique_fns,
            };

            Ok(warp::reply::json(&final_response).into_response())
        }
        */


        Ok(BatchSubmitServer::create_response(resp, on_dev_time, queue_submit_time, num_queue_submits, num_unique_fns))
    }

    pub fn start_server(_hcall_buf_size: usize, fast_reply: bool, is_active: Arc<SyncMutex<bool>>, sender: Arc<Vec<Mutex<Sender<(bytes::Bytes, usize)>>>>, receiver: Arc<Vec<Mutex<Receiver<VmSenderType>>>>, num_vms: u32, server_ip: String, server_port: String) -> () {
        
        tokio::runtime::Builder::new_multi_thread()
            //.worker_threads(4)
            .worker_threads(num_cpus::get())
            .thread_stack_size(1024 * 256) // 256KiB per thread should be enough
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {{
                    // Set up queue of available VMs (each VM has a unique communication channel)
                    let queue = Arc::new(VmQueue::new(num_vms.try_into().unwrap()));
                    for i in 0..num_vms {
                        queue.push(i.try_into().unwrap()).await;
                    }

                    let fast_reply_bool = warp::any().map(move || fast_reply);
                    let warp_queue = warp::any().map(move || Arc::clone(&queue));
                    let warp_senders = warp::any().map(move || Arc::clone(&sender));
                    let warp_receivers = warp::any().map(move || Arc::clone(&receiver));

                    let vm_idx_counter = Arc::new(AtomicU64::new(0));

                    let num_vms_u64: u64 = num_vms as u64;
                    let warp_scheduler = warp::any().map(move || {
                        let current_idx = vm_idx_counter.fetch_add(1, Ordering::SeqCst);

                        (current_idx % num_vms_u64) as usize
                    });

                    let batch_submit = warp::path!("batch_submit")
                                        .and(warp::body::bytes()).and(fast_reply_bool).and(warp_scheduler).and(warp_queue).and(warp_senders).and(warp_receivers).and_then(BatchSubmitServer::response);


                    let is_active_param = warp::any().map(move || Arc::clone(&is_active));
                    let active = warp::path!("is_active").and(is_active_param).map(|is_active: Arc<SyncMutex<bool>>| {
                        let temp = is_active.lock().unwrap();
                        format!("{}", temp)
                    });

                    let terminate = warp::path!("terminate").map(|| {
                        std::process::exit(0);
                        format!("terminate")
                    });

                    let socket: SocketAddr = format!("{}:{}", server_ip, server_port).parse().unwrap();
                    warp::serve(batch_submit.or(active).or(terminate)).run(socket).await;
            }});
    }
}
