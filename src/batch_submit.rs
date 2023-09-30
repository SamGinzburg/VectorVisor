use std::collections::HashMap;
use std::convert::TryInto;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::sync::Mutex as SyncMutex;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

use serde::Deserialize;
use serde::Serialize;

use hyper::Body;
use uuid::Uuid;
use warp::http::{Response, StatusCode};
use warp::Filter;

use crate::opencl_runner::vectorized_vm::{VmRecvType, VmSenderType};

pub struct BatchSubmitServer {}

#[derive(Debug, Deserialize)]
struct FunctionInput {
    req: String,
    req_id: u32,
}

#[derive(Debug, Deserialize)]
struct BatchInput {
    requests: Vec<FunctionInput>,
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
    requests: HashMap<u32, BatchReply<'a>>,
}

type VmQueue = deadqueue::limited::Queue<usize>;

#[derive(Debug)]
struct NoVmAvailable;

impl warp::reject::Reject for NoVmAvailable {}

#[derive(Debug)]
struct LostRequest;

impl warp::reject::Reject for LostRequest {}

impl BatchSubmitServer {
    fn create_response(
        resp: bytes::Bytes,
        on_dev_time: u64,
        queue_submit_time: u64,
        num_queue_submits: u64,
        num_unique_fns: u64,
        queue_time: u128,
        device_time: u128,
        overhead_time: u64,
        compile_time: u128,
    ) -> warp::http::Response<Body> {
        let mut final_resp = Response::builder().status(StatusCode::OK);
        {
            let headers = final_resp.headers_mut().unwrap();
            headers.insert(
                "on_device_time",
                warp::http::HeaderValue::from_str(&on_dev_time.to_string()).unwrap(),
            );
            headers.insert(
                "queue_submit_time",
                warp::http::HeaderValue::from_str(&queue_submit_time.to_string()).unwrap(),
            );
            headers.insert(
                "num_queue_submits",
                warp::http::HeaderValue::from_str(&num_queue_submits.to_string()).unwrap(),
            );
            headers.insert(
                "num_unique_fns",
                warp::http::HeaderValue::from_str(&num_unique_fns.to_string()).unwrap(),
            );
            headers.insert(
                "req_queue_time",
                warp::http::HeaderValue::from_str(&queue_time.to_string()).unwrap(),
            );
            headers.insert(
                "device_time",
                warp::http::HeaderValue::from_str(&device_time.to_string()).unwrap(),
            );
            headers.insert(
                "overhead_time_ns",
                warp::http::HeaderValue::from_str(&overhead_time.to_string()).unwrap(),
            );
            headers.insert(
                "compile_time_ns",
                warp::http::HeaderValue::from_str(&compile_time.to_string()).unwrap(),
            );
        }

        final_resp.body(Body::from(resp)).unwrap()
    }

    async fn response(
        body: bytes::Bytes,
        fast_reply: bool,
        vm_idx: usize,
        vm_queue: Arc<VmQueue>,
        sender: Arc<
            Vec<(
                Mutex<Sender<(bytes::Bytes, usize, String)>>,
                Mutex<Sender<(bytes::Bytes, usize, String)>>,
            )>,
        >,
        receiver: Arc<Vec<(Mutex<Receiver<VmSenderType>>, Mutex<Receiver<VmSenderType>>)>>,
        vm_chan_ctr: Arc<Vec<AtomicU64>>,
        compile_time: u128,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        //dbg!(&vm_idx);
        // Get an available VM first
        let chan_id = (*vm_chan_ctr)
            .get(vm_idx)
            .unwrap()
            .fetch_add(1, Ordering::SeqCst);
        let (tx, tx2) = (*sender).get(vm_idx).unwrap();
        let (rx, rx2) = (*receiver).get(vm_idx).unwrap();
        let actual_recv = if chan_id % 2 == 0 { rx } else { rx2 };

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
        //
        // Tokio locks are FIFO, and tokio is cooperatively scheduled
        // The first person to acquire sender is also the first to acquire recv
        let req_queue = std::time::Instant::now();
        let req_id = Uuid::new_v4().to_simple().to_string();
        let req_start;
        // Acquire both locks...
        let sender = if chan_id % 2 == 0 { tx } else { tx2 };
        let sender = match sender.try_lock() {
            Ok(lock) => lock,
            Err(_) => return Err(warp::reject::custom(NoVmAvailable)),
        };

        let mut recv = match actual_recv.try_lock() {
            Ok(lock) => lock,
            Err(_) => return Err(warp::reject::custom(NoVmAvailable)),
        };

        // clear all previous requests...
        while true {
            match recv.try_recv() {
                Ok(_) => {
                    // no-op
                }
                _ => {
                    // The queue is clear, we can continue
                    break;
                }
            }
        }

        req_start = std::time::Instant::now();
        sender
            .send((body.clone(), body.len(), req_id.clone()))
            .await
            .unwrap();

        match recv.recv().await {
            Some((
                resp,
                len,
                on_dev_time,
                queue_submit_time,
                num_queue_submits,
                num_unique_fns,
                overhead_time_ns,
                uuid,
            )) => {
                if uuid == req_id {
                    let req_end = std::time::Instant::now();
                    return Ok(BatchSubmitServer::create_response(
                        resp,
                        on_dev_time,
                        queue_submit_time,
                        num_queue_submits,
                        num_unique_fns,
                        (req_start - req_queue).as_nanos(),
                        (req_end - req_queue).as_nanos(),
                        overhead_time_ns,
                        compile_time,
                    ));
                }
            }
            _ => (),
        }

        //  If we got the wrong response
        Err(warp::reject::custom(LostRequest))
    }

    pub fn start_server(
        _hcall_buf_size: usize,
        fast_reply: bool,
        is_active: Arc<SyncMutex<bool>>,
        sender: Arc<
            Vec<(
                Mutex<Sender<(bytes::Bytes, usize, String)>>,
                Mutex<Sender<(bytes::Bytes, usize, String)>>,
            )>,
        >,
        receiver: Arc<Vec<(Mutex<Receiver<VmSenderType>>, Mutex<Receiver<VmSenderType>>)>>,
        num_vms: u32,
        server_ip: String,
        server_port: String,
        compile_time: u128,
    ) -> () {
        tokio::runtime::Builder::new_multi_thread()
            //.worker_threads(4)
            .worker_threads(num_cpus::get())
            .thread_stack_size(1024 * 256) // 256KiB per thread should be enough
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                {
                    // Set up queue of available VMs (each VM has a unique communication channel)
                    let queue = Arc::new(VmQueue::new(num_vms.try_into().unwrap()));
                    for i in 0..num_vms {
                        queue.push(i.try_into().unwrap()).await;
                    }

                    let fast_reply_bool = warp::any().map(move || fast_reply);
                    let warp_queue = warp::any().map(move || Arc::clone(&queue));
                    let warp_senders = warp::any().map(move || Arc::clone(&sender));
                    let warp_receivers = warp::any().map(move || Arc::clone(&receiver));
                    let compile_time = warp::any().map(move || compile_time.clone());

                    let vm_idx_counter = Arc::new(AtomicU64::new(0));

                    let num_vms_u64: u64 = num_vms as u64;
                    let warp_scheduler = warp::any().map(move || {
                        let current_idx = vm_idx_counter.fetch_add(1, Ordering::SeqCst);

                        (current_idx % num_vms_u64) as usize
                    });

                    // Channel counter
                    let mut chan_ctr: Vec<AtomicU64> = vec![];
                    for i in 0..num_vms {
                        chan_ctr.push(AtomicU64::new(0));
                    }
                    let vm_chan_ctr = Arc::new(chan_ctr);
                    let chan_ctr_moved = warp::any().map(move || Arc::clone(&vm_chan_ctr));

                    let batch_submit = warp::path!("batch_submit")
                        .and(warp::body::bytes())
                        .and(fast_reply_bool)
                        .and(warp_scheduler)
                        .and(warp_queue)
                        .and(warp_senders)
                        .and(warp_receivers)
                        .and(chan_ctr_moved)
                        .and(compile_time)
                        .and_then(BatchSubmitServer::response);

                    let is_active_param = warp::any().map(move || Arc::clone(&is_active));
                    let active = warp::path!("is_active").and(is_active_param).map(
                        |is_active: Arc<SyncMutex<bool>>| {
                            let temp = is_active.lock().unwrap();
                            format!("{}", temp)
                        },
                    );

                    let terminate = warp::path!("terminate").map(|| {
                        std::process::exit(0);
                        format!("terminate")
                    });

                    let socket: SocketAddr =
                        format!("{}:{}", server_ip, server_port).parse().unwrap();
                    warp::serve(batch_submit.or(active).or(terminate))
                        .run(socket)
                        .await;
                }
            });
    }
}
