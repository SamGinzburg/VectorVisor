#[macro_use]
extern crate rustacuda;
extern crate rustacuda_core;

use rustacuda::context::CurrentContext;
use rustacuda::prelude::*;
use rustacuda::memory::DeviceBox;
use std::error::Error;
use std::ffi::CString;
use std::thread;
use std::net::SocketAddr;
use warp::Filter;
use tokio::sync::Mutex as AsyncMutex;
use std::sync::{Mutex, Arc};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use std::path::Path;
use std::time::Instant;
use base64::encode;
use base64::decode;
use image::load_from_memory_with_format;
use image::ImageFormat;
use rmp_serde::{decode, encode};

use hyper::Body;
use warp::http::{Response, StatusCode};
use std::sync::atomic::{AtomicU64, Ordering};
use image::codecs::bmp::BmpEncoder;
use image::ColorType;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;


#[derive(Debug, Deserialize)]
struct FuncInput<'a> {
    // Image comes in base64 encoded
    image: Cow<'a, str>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    hash: Vec<u8>
}

// send+recv the bytes of the image to/from GPU workers
pub type VmSenderType = bytes::Bytes;
pub type VmRecvType = bytes::Bytes;

fn create_kernel(radius: i32) -> Vec<f32> {
    let width = ((radius * 2) + 1) as usize;
    let mut mtrx = vec![vec![0.0f32; width]; width];
    let sigma: f32 = (radius as f32 / 2.0).max(1.0);
    let e = std::f32::consts::E;
    let pi = std::f32::consts::PI;
    let mut sum = 0.0;

    for x in -radius..radius {
        for y in -radius..radius {
            let exp_nom: f32 = -((x * x) + (y * y)) as f32;
            let exp_den: f32 = 2.0 * sigma * sigma;

            let e_expr: f32 = e.powf(exp_nom / exp_den);
            let e_val: f32 = e_expr / (2.0 * pi * sigma * sigma);

            let i = (x + radius as i32) as usize;
            let j = (y + radius as i32) as usize;

            mtrx[i][j] = e_val;
            sum += mtrx[i][j];
        }
    }

    // Normalize the kernel so that sum = 1
    for x in 0..width {
        for y in 0..width {
            mtrx[x][y] /= sum;
        }
    }

    // Now convert it to the Vec<f32>
    let flatten_mtrx: Vec<f32> = mtrx
        .iter()
        .flat_map(|nested| nested.iter())
        .cloned()
        .collect();

    flatten_mtrx
}

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


async fn response(body: bytes::Bytes,
                  vm_idx: usize,
                  sender: Arc<Vec<Arc<AsyncMutex<Sender<VmRecvType>>>>>,
                  receiver: Arc<Vec<Arc<AsyncMutex<Receiver<VmSenderType>>>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let req_start = std::time::Instant::now();
    let tx: &Arc<AsyncMutex<Sender<VmRecvType>>> = (*sender).get(vm_idx).unwrap();
    let rx: &Arc<AsyncMutex<Receiver<VmSenderType>>> = (*receiver).get(vm_idx).unwrap();

    let req_queue = std::time::Instant::now();

    tx.lock().await.send(body.clone()).await.unwrap();

    let resp = rx.lock().await.recv().await.unwrap();
    let req_end = std::time::Instant::now();
 
    return Ok(create_response(
                    resp,
                    (req_start - req_end).as_nanos().try_into().unwrap(),
                    0,
                    0,
                    0,
                    (req_start - req_queue).as_nanos(),
                    (req_end - req_queue).as_nanos(),
                    0,
                    0,
                ));
} 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the CUDA API
    rustacuda::init(CudaFlags::empty())?;
     
    // Get the first device
    let device = Device::get_device(0)?;

    // Create a context associated to this device
    let context = Context::create_and_push(
        ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

    // create a number of worker threads to run GPU compute on
    // we will communicate with these using channels.
    let num_threads = num_cpus::get();

    // create channels for communication
    let mut vm_sender_vec = vec![];
    let mut vm_recv_vec = vec![];
    let mut server_recv_vec = vec![];
    let mut server_sender_vec = vec![];

    for idx in 0..num_threads {
        let (sender, recv): (
            tokio::sync::mpsc::Sender<VmSenderType>,
            tokio::sync::mpsc::Receiver<VmSenderType>,
        ) = mpsc::channel(16384);
        server_sender_vec.push(Arc::new(AsyncMutex::new(sender)));
        vm_recv_vec.push(Arc::new(Mutex::new(recv)));
    }

    for idx in 0..num_threads {
        let (sender, recv): (
            tokio::sync::mpsc::Sender<VmSenderType>,
            tokio::sync::mpsc::Receiver<VmSenderType>,
        ) = mpsc::channel(16384);
        vm_sender_vec.push(Arc::new(Mutex::new(sender)));
        server_recv_vec.push(Arc::new(AsyncMutex::new(recv)));
    }


    for idx in 0..num_threads {
        let unowned = context.get_unowned();
        let server_recv = vm_recv_vec[idx].clone();
        let server_send = vm_sender_vec[idx].clone();
        thread::spawn(move || {
            CurrentContext::set_current(&unowned).unwrap();
            let stream = Stream::new(StreamFlags::NON_BLOCKING, None).unwrap();
            let module_data = CString::new(include_str!("../kernel/blur.ptx")).unwrap();
            let module = Module::load_from_string(&module_data).unwrap();
            let module_blockhash_data = CString::new(include_str!("../kernel/blockhash.ptx")).unwrap();
            let module_blockhash = Module::load_from_string(&module_blockhash_data).unwrap();

            // Get incoming request from channel
            let mut test_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open(&Path::new("0.jpg")).unwrap().to_rgba8();
            let data_size: usize = test_image.as_raw().len() * 4;
            let block_size: usize = 32; //test_image.as_raw().len() / 256 / 8;

            let mut kernel_matrix_20 = create_kernel(20);
            let mut kernel_matrix_10 = create_kernel(10);

            // Allocate space on the device and copy numbers to it.
            let mut kernel_20: DeviceBuffer<f32> = unsafe { DeviceBuffer::zeroed(kernel_matrix_20.len()).unwrap() };
            let mut kernel_10: DeviceBuffer<f32> = unsafe { DeviceBuffer::zeroed(kernel_matrix_10.len()).unwrap() };
            let mut result: DeviceBuffer<u8> = unsafe { DeviceBuffer::zeroed(data_size).unwrap() };
            let mut result2: DeviceBuffer<u8> = unsafe { DeviceBuffer::zeroed(data_size).unwrap() };
            let mut result_blocks: DeviceBuffer<u32> = unsafe { DeviceBuffer::zeroed(block_size*block_size).unwrap() };

            loop {
                let now_cpu = Instant::now();

                let bytes = server_recv.lock().unwrap().blocking_recv().unwrap();
                let input: FuncInput = decode::from_read(&*bytes).unwrap();
                let mut image = decode(&input.image.as_bytes()).expect(&format!("b64 decode error: {:?}", bytes));
                let mut decoded_image = load_from_memory_with_format(&image, ImageFormat::Bmp).unwrap().to_rgba8();

                let mut x: DeviceBuffer<u8> = unsafe { DeviceBuffer::zeroed(decoded_image.as_raw().as_slice().len()).unwrap() };
                let mut y: DeviceBuffer<u8> = unsafe { DeviceBuffer::zeroed(decoded_image.as_raw().as_slice().len()).unwrap() };
                x.copy_from(decoded_image.as_raw().as_slice()).unwrap();
                y.copy_from(decoded_image.as_raw().as_slice()).unwrap();

                kernel_20.copy_from(&kernel_matrix_20).unwrap();
                kernel_10.copy_from(&kernel_matrix_10).unwrap();
                // First we perform blockhash preprocessing
                // To do this, we blur the same image twice (different params) and subtract the difference
                unsafe {
                    launch!(module.blur_and_sub<<<256, 256, 0, stream>>>(
                        x.as_device_ptr(),
                        y.as_device_ptr(),
                        result.as_device_ptr(),
                        result2.as_device_ptr(),
                        256,
                        256,
                        kernel_20.as_device_ptr(),
                        kernel_10.as_device_ptr(),
                        20,
                        10,
                        1
                    )).unwrap();

                    // Compute the blockhash
                    launch!(module_blockhash.blockhash<<<64, 64, 0, stream>>>(
                        result.as_device_ptr(),
                        result_blocks.as_device_ptr(),
                        8, 8,
                        32,
                        1
                    )).unwrap();

                }

                // Following the preprocessing, we can perform the blockhash itself
                stream.synchronize().unwrap();
                // Copy the result back to the host
                let mut blocks_result: Vec<u32> = vec![];
                blocks_result.resize(block_size*block_size, 0);
                result_blocks.copy_to(&mut blocks_result).unwrap();
                dbg!(&blocks_result);

                // We can now examine the block values to compute the image hash itself

                let final_resp = encode::to_vec(&FuncResponse { hash: vec![]  }).unwrap();
                server_send.lock().unwrap().blocking_send(bytes::Bytes::from(final_resp));
            }

            // Return response...
        });
    }

    let num_vms_u64: u64 = num_threads as u64;
    let vm_idx_counter = Arc::new(AtomicU64::new(0));
    let warp_scheduler = warp::any().map(move || {
                            let current_idx = vm_idx_counter.fetch_add(1, Ordering::SeqCst);
                            (current_idx % num_vms_u64) as usize
                        });
    let server_senders = Arc::new(server_sender_vec);
    let server_recvs = Arc::new(server_recv_vec);
    let warp_senders = warp::any().map(move || Arc::clone(&server_senders));
    let warp_recvs = warp::any().map(move || Arc::clone(&server_recvs));

    let active = warp::path!("is_active").map(|| {
        format!("{}", true)
    });

    let terminate = warp::path!("terminate").map(|| {
        std::process::exit(0);
        format!("terminate")
    });


    let batch_submit = warp::path!("batch_submit")
                        .and(warp::body::bytes())
                        .and(warp_scheduler)
                        .and(warp_senders)
                        .and(warp_recvs)
                        .and_then(response);

    let server_ip = "0.0.0.0";
    let server_port = 8000;
    let socket: SocketAddr = format!("{}:{}", server_ip, server_port).parse().unwrap();

    warp::serve(batch_submit.or(active).or(terminate))
        .run(socket)
        .await;

    Ok(())
}
