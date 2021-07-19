use warp::{Filter, Reply};
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::Mutex;
use std::str::from_utf8;
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Arc;
use std::sync::Mutex as SyncMutex;
use std::net::{SocketAddr};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct FunctionServer {}

#[derive(Debug, Serialize)]
struct BatchReply {
    response: String,
    on_device_execution_time_ns: u64,
    device_queue_overhead_time_ns: u64,
    queue_submit_count: u64,
    num_unique_fns_called: u64,
}

impl FunctionServer {
        async fn response(body: bytes::Bytes, vm_idx: usize, sender: Arc<Vec<Mutex<Sender<(Vec<u8>, usize)>>>>, receiver: Arc<Vec<Mutex<Receiver<(Vec<u8>, usize, u64, u64, u64, u64)>>>>) -> Result<impl warp::Reply, warp::Rejection> {
        // Get an available VM first
        let tx: &Mutex<Sender<(Vec<u8>, usize)>> = (*sender).get(vm_idx).unwrap();
        let rx: &Mutex<Receiver<(Vec<u8>, usize, u64, u64, u64, u64)>> = (*receiver).get(vm_idx).unwrap();

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

    pub fn start_server(sender: Arc<Vec<Mutex<Sender<(Vec<u8>, usize)>>>>, receiver: Arc<Vec<Mutex<Receiver<(Vec<u8>, usize, u64, u64, u64, u64)>>>>, is_active: Arc<SyncMutex<bool>>, num_vms: u32) -> () {
            tokio::runtime::Builder::new_multi_thread()
            .worker_threads(num_cpus::get() * 2)
            .thread_stack_size(1024 * 256) // 256KiB per thread should be enough
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {{
                    let warp_senders = warp::any().map(move || Arc::clone(&sender));
                    let warp_receivers = warp::any().map(move || Arc::clone(&receiver));
                    let vm_idx_counter = Arc::new(AtomicU64::new(0));
                    let num_vms_u64: u64 = num_vms as u64;
                    let warp_scheduler = warp::any().map(move || {
                        let current_idx = vm_idx_counter.fetch_add(1, Ordering::SeqCst);

                        (current_idx % num_vms_u64) as usize
                    });

                    let batch_submit = warp::path!("batch_submit")
                                        .and(warp::body::bytes()).and(warp_scheduler).and(warp_senders).and(warp_receivers).and_then(FunctionServer::response);


                    let is_active_param = warp::any().map(move || Arc::clone(&is_active));
                    let active = warp::path!("is_active").and(is_active_param).map(|is_active: Arc<SyncMutex<bool>>| {
                        let temp = is_active.lock().unwrap();
                        format!("{}", temp)
                    });

                    let terminate = warp::path!("terminate").map(|| {
                        std::process::exit(0);
                        format!("terminate")
                    });

                    let socket: SocketAddr = format!("{}:{}", "0.0.0.0", "8000").parse().unwrap();
                    println!("Listening on: {:?}", &socket);
                    warp::serve(batch_submit.or(active).or(terminate)).run(socket).await;
            }});
    }
}


