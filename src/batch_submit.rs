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
use uuid::Uuid;

use crate::opencl_runner::vectorized_vm::{VmSenderType, VmRecvType};

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

    fn create_response(resp: Vec<u8>, on_dev_time: u64, queue_submit_time: u64, num_queue_submits: u64, num_unique_fns: u64, queue_time: u128, device_time: u128) -> warp::http::Response<Body> {
        let mut final_resp = Response::builder().status(StatusCode::OK);
        {
            let headers = final_resp.headers_mut().unwrap();
            headers.insert("on_device_time", warp::http::HeaderValue::from_str(&on_dev_time.to_string()).unwrap());
            headers.insert("queue_submit_time", warp::http::HeaderValue::from_str(&queue_submit_time.to_string()).unwrap());
            headers.insert("num_queue_submits", warp::http::HeaderValue::from_str(&num_queue_submits.to_string()).unwrap());
            headers.insert("num_unique_fns", warp::http::HeaderValue::from_str(&num_unique_fns.to_string()).unwrap());
            headers.insert("req_queue_time", warp::http::HeaderValue::from_str(&queue_time.to_string()).unwrap());
            headers.insert("device_time", warp::http::HeaderValue::from_str(&queue_time.to_string()).unwrap());
        }

        final_resp.body(Body::from(resp)).unwrap()
    }


    async fn response(body: bytes::Bytes, fast_reply: bool, vm_idx: usize, vm_queue: Arc<VmQueue>, sender: Arc<Vec<Mutex<Sender<(bytes::Bytes, usize, String)>>>>, receiver: Arc<Vec<Mutex<Receiver<VmSenderType>>>>) -> Result<impl warp::Reply, warp::Rejection> {

        //dbg!(&vm_idx);
        // Get an available VM first
        let tx: &Mutex<Sender<VmRecvType>> = (*sender).get(vm_idx).unwrap();
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
        let req_queue = std::time::Instant::now();
        {
            let sender = tx.lock().await;
            let req_id = Uuid::new_v4().to_simple().to_string();
            let req_start = std::time::Instant::now();
            sender.send((body.clone(), body.len(), req_id.clone())).await.unwrap();

            // Wait on response from the VM
            while let Some((resp,
                            len,
                            on_dev_time,
                            queue_submit_time,
                            num_queue_submits,
                            num_unique_fns,
                            uuid)) = rx.lock().await.recv().await {
                if uuid == req_id {
                    let req_end = std::time::Instant::now();
                    return Ok(BatchSubmitServer::create_response(resp, on_dev_time, queue_submit_time, num_queue_submits, num_unique_fns, (req_start-req_queue).as_nanos(), (req_end-req_queue).as_nanos()))
                }
            }
        }

        panic!("This line in batch server should not be reached")
    }

    pub fn start_server(_hcall_buf_size: usize, fast_reply: bool, is_active: Arc<SyncMutex<bool>>, sender: Arc<Vec<Mutex<Sender<(bytes::Bytes, usize, String)>>>>, receiver: Arc<Vec<Mutex<Receiver<VmSenderType>>>>, num_vms: u32, server_ip: String, server_port: String) -> () {
        
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
