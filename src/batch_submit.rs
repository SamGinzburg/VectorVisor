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
struct BatchReply {
    response: String,
    on_device_execution_time_ns: u64,
    device_queue_overhead_time_ns: u64,
    queue_submit_count: u64,
    num_unique_fns_called: u64,
}

#[derive(Debug, Serialize)]
struct BatchResponse {
    requests: HashMap<u32, BatchReply>
}

type VmQueue = deadqueue::limited::Queue<usize>;

#[derive(Debug)]
struct NoVmAvailable;

impl warp::reject::Reject for NoVmAvailable {}

impl BatchSubmitServer {

    async fn response(body: bytes::Bytes, vm_idx: usize, vm_queue: Arc<VmQueue>, sender: Arc<Vec<Mutex<Sender<(Vec<u8>, usize)>>>>, receiver: Arc<Vec<Mutex<Receiver<(Vec<u8>, usize, u64, u64, u64, u64)>>>>) -> Result<impl warp::Reply, warp::Rejection> {

        /*
        dbg!(&vm_idx);
        // Get an available VM first
        let tx: &Mutex<Sender<(Vec<u8>, usize)>> = (*sender).get(vm_idx).unwrap();
        let rx: &Mutex<Receiver<(Vec<u8>, usize, u64, u64, u64, u64)>> = (*receiver).get(vm_idx).unwrap();
        */

        let (tx, rx, vm_idx) = match vm_queue.try_pop() {
            Some(idx) => {
                ((*sender).get(idx).unwrap(), (*receiver).get(idx).unwrap(), idx)
            },
            // TODO, if we have no available GPU workers, try using backup CPU resources
            None => return Err(warp::reject::custom(NoVmAvailable)),
        };

        // Send the request body to the selected VM
        // We can't await on the send because we have the mutex acquired here
        let sender = tx.lock().await;
        
        sender.send((body.to_vec(), body.len())).await.unwrap();

        // Wait on response from the VM
        let (resp, len, on_dev_time, queue_submit_time, num_queue_submits, num_unique_fns) = match rx.lock().await.recv().await {
            Some(val) => val,
            None => panic!("A VM died while processing a request, vm_idx: {}", vm_idx),
        };

        let final_response = BatchReply {
            response: from_utf8(&resp[0..len]).unwrap().to_string(),
            on_device_execution_time_ns: on_dev_time,
            device_queue_overhead_time_ns: queue_submit_time,
            queue_submit_count: num_queue_submits,
            num_unique_fns_called: num_unique_fns,
        };

        Ok(warp::reply::json(&final_response).into_response())
    }

    pub fn start_server(_hcall_buf_size: usize, is_active: Arc<SyncMutex<bool>>, sender: Arc<Vec<Mutex<Sender<(Vec<u8>, usize)>>>>, receiver: Arc<Vec<Mutex<Receiver<(Vec<u8>, usize, u64, u64, u64, u64)>>>>, num_vms: u32, server_ip: String, server_port: String) -> () {
        
        tokio::runtime::Builder::new_multi_thread()
            //.worker_threads(4)
            .worker_threads(num_cpus::get() * 2)
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
                                        .and(warp::body::bytes()).and(warp_scheduler).and(warp_queue).and(warp_senders).and(warp_receivers).and_then(BatchSubmitServer::response);


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
