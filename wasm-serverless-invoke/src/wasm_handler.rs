use serde_json::{json, Value, to_vec};
use rmp_serde::{decode, encode};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub mod server;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use server::FunctionServer;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use std::sync::{Mutex, Arc};
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use tokio::sync::Mutex as AsyncMutex;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use tokio::sync::mpsc;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use std::thread;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use chrono::prelude::*;

use std::convert::TryInto;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

enum FuncReturn {
    None,
}

#[derive(Clone, Copy)]
pub enum SerializationFormat {
    Json,
    MsgPack,
}

pub struct WasmHandler<T1: 'static, T2: 'static> {
    //function: Box<(dyn Fn(T1) -> T2)>
    function: &'static (dyn Fn(T1) -> T2 + Send + Sync),
}

extern "C" {
    // Our syscall will write directly to a buffer of 16KiB in size
    // which we will cast to a json input
    fn serverless_invoke(input_arr: *mut u8, input_arr_len: u32) -> u32;

    // return the json response back to the VMM
    fn serverless_response(output_arr: *mut u8, output_arr_len: u32) -> ();

    // Custom syscalls for acquiring/releasing locks on VMs
    fn lock_vm(vm_id: usize) -> ();
    fn unlock_vm(vm_id: usize) -> ();
    fn num_total_vms() -> usize;

    // Copy data between VMs (from VM 1, to VM 2)
    fn inter_vm_copy(buf: &[u8], len: usize, vm1: usize, vm2: usize) -> ();

    // Switch read/write mode to another VM
    fn get_warp_idx() -> usize;
    fn set_warp_idx(new_warp_idx: usize) -> ();
}


// VectorVisor API for "distributed" applications
pub struct VMMailbox<T> {
    copy_buffer: [u8; 1024*16], // 16KiB copy buffer
    mailbox: Vec<T>,
    // used for locking
    atomic_val: usize,
    current_vm_id: usize,
}

impl<T: DeserializeOwned + Serialize + Clone + Copy> VMMailbox<T> {
    pub fn send(&mut self, data: T, vm_id: usize) -> ()
    where
        T: Clone + Copy {
        // 1) Acquire lock on remote VM's mailbox
        unsafe { lock_vm(vm_id) };
        // 2) Copy data into remote VM's copy_buffer
        //    which will be alloc'd at the same offset
        let encoded_data = encode::to_vec(&data).unwrap();
        let encoded_data_len = encoded_data.len();

        self.copy_buffer.clone_from_slice(&encoded_data);

        // 2.1) Perform inter-VM copy (VM 1 to VM 2)
        unsafe { inter_vm_copy(&self.copy_buffer, encoded_data_len, self.current_vm_id, vm_id) };

        // 2.1) Unmarshal data in VM 2
        // old_warp_idx is saved on the stack
        let old_warp_idx = unsafe { get_warp_idx() };
        unsafe { set_warp_idx(vm_id) };
        let recieved_msg: T = decode::from_slice(&self.copy_buffer).unwrap();
        self.mailbox.push(recieved_msg);
        unsafe { set_warp_idx(old_warp_idx) };

        // 3) Release lock
        unsafe { unlock_vm(vm_id) };
    }

    pub fn broadcast(&mut self, data: T) -> ()
    where
        T: Clone + Copy {
            // send(data) for each VM in system
            let current_vm_id = self.current_vm_id.clone();
            let total_vm_count = unsafe { num_total_vms() };
            // Start with the next VM and continue sending
            // This lets us keep some parallelism w.r.t writes and avoid lock contention
            for vm_idx in current_vm_id..(current_vm_id+total_vm_count) {
                let curr_vm = vm_idx % total_vm_count;
                if curr_vm != current_vm_id {
                    self.send(data, curr_vm);
                }
            }
    }

    pub fn recv(&mut self) -> Option<T>
    where
        T: Clone + Copy {
        // 1) Acquire lock on local mailbox
        unsafe { lock_vm(self.current_vm_id) };

        // 2) Pop val
        let val = self.mailbox.pop();

        // 3) Release lock on local mailbox
        unsafe { unlock_vm(self.current_vm_id) };

        return val;
    }
}


impl<'a, T1: DeserializeOwned, T2: Serialize> WasmHandler<T1, T2> {

    pub fn new(func: &'static (dyn Fn(T1) -> T2 + Send + Sync)) -> WasmHandler<T1, T2> {
        WasmHandler {
            //function: Box::new(func),
            function: func,
        }
    }

    // easy hack to get *mut u8 ref from buffer in our loop, we ensure that this operation is safe
    unsafe fn get_unsafe_mut_ref(buf: &[u8]) -> *mut u8 {
        let const_ptr: *const u8 = buf.as_ptr();
        let mut_ptr: *mut u8 = const_ptr as *mut u8;
        mut_ptr
    }


    // The default serialization format is JSON
    pub fn run(self, hcall_buf_size: usize) -> () {
        self.run_with_format(hcall_buf_size, SerializationFormat::Json);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run_with_format(self, hcall_buf_size: usize, serializiation_format: SerializationFormat) -> () {
        // main run loop of the runtime
        // First, allocate a buffer to store json input
        let buf_size = (hcall_buf_size + 8) / 8;
        let mut u64_vec = vec![0u64; buf_size];
        // We want to ensure an 8-byte alignment, so we alloc with 8-byte types, then transmute
        let ptr = u64_vec.as_mut_ptr();
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts
        // This API is pretty bad, but we manage the unsafety here:
        // - the ptr is guaranteed to be safe here, as it was alloc'd via vec![]
        // - length == capacity
        // - We don't have to worry about proper deallocation, since these buffers live for the
        //   duration of the application.
        let mut buffer: &mut Vec<u8> = &mut unsafe {
            let ptr = ptr as *mut u8;
            Vec::from_raw_parts(ptr, buf_size*8, buf_size*8) 
        };

        std::mem::forget(u64_vec);

        let mut func_ret_val: T2;
        // if this is the first invocation, then we skip sending the buffer back
        loop {

            // block until we get a request, which we will populate into the buffer
            let incoming_req_size = unsafe {
                let buf_ptr = WasmHandler::<T1, T2>::get_unsafe_mut_ref(buffer);
                serverless_invoke(buf_ptr, buffer.len() as u32)
            };
            
            if incoming_req_size > 0 {
                let parsed_func_input: T1 = {
                    match serializiation_format {
                        SerializationFormat::Json => {
                            serde_json::from_slice(&buffer[..incoming_req_size as usize]).unwrap()
                        },
                        SerializationFormat::MsgPack => {
                            decode::from_read(&buffer[..incoming_req_size as usize]).unwrap()
                        }
                    }
                };

                func_ret_val = (self.function)(parsed_func_input);
                let mut func_ret_val_as_buffer = match serializiation_format { 
                    SerializationFormat::Json => {
                        to_vec(&func_ret_val).unwrap()
                    },
                    SerializationFormat::MsgPack => {
                        encode::to_vec(&func_ret_val).unwrap()
                    },
                };

                unsafe {
                    serverless_response(func_ret_val_as_buffer.as_mut_ptr(), func_ret_val_as_buffer.len() as u32);
                }
            } else {
                unsafe {
                    serverless_response(0 as *mut u8, 0 as u32);
                }
            }

            /*
            match parsed_func_input {
                Ok(json) => {
                    // run the function, get the response
                    func_ret_val = (self.function)(json);

                    // copy the response to the buffer
                    let mut func_ret_val_as_buffer = to_string(&func_ret_val).unwrap();
                    // return the response
                    unsafe {
                        serverless_response(func_ret_val_as_buffer.as_mut_ptr(), func_ret_val_as_buffer.len() as u32);
                    }
                },
                Err(_) => {
                    // return an empty response if we were unable to parse the input properly
                    unsafe {
                        let buf_ptr = WasmHandler::<T1, T2>::get_unsafe_mut_ref(buffer);
                        serverless_response(buf_ptr, 0 as u32);
                    }
                },
            };
            */
        }
    }
    
    // Compile a modified version of the run function for running x86 versions of benchmarks
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub fn run_with_format(self, hcall_buf_size: usize, serializiation_format: SerializationFormat) -> () {
        // Start webserver
        let num_cpu_threads = num_cpus::get();
        // create channels for threads, copied from main VMM
        let mut server_sender_vec = vec![];
        let mut vm_recv_vec = vec![];
        for _ in 0..num_cpu_threads {
            let (sender, recv): (tokio::sync::mpsc::Sender<(Vec<u8>, usize, String)>,
                                 tokio::sync::mpsc::Receiver<(Vec<u8>, usize, String)>) = mpsc::channel(16384);
            server_sender_vec.push(AsyncMutex::new(sender));
            vm_recv_vec.push(Mutex::new(recv));
        }

        let server_sender_vec_arc = Arc::new(server_sender_vec);
        let vm_recv_vec_arc = Arc::new(vm_recv_vec);

        let mut vm_sender_vec = vec![];
        let mut server_recv_vec = vec![];
        for _ in 0..num_cpu_threads {
            let (sender, recv): (tokio::sync::mpsc::Sender<(Vec<u8>, usize, u64, u64, u64, u64, String, u32)>,
                                 tokio::sync::mpsc::Receiver<(Vec<u8>, usize, u64, u64, u64, u64, String, u32)>) = mpsc::channel(16384);
            vm_sender_vec.push(Mutex::new(sender));
            server_recv_vec.push(AsyncMutex::new(recv));
        }

        let vm_sender_vec_arc = Arc::new(vm_sender_vec);
        let server_recv_vec_arc = Arc::new(server_recv_vec);
        let is_active = Arc::new(Mutex::new(true));

        // start server
        thread::spawn(move || {
            FunctionServer::start_server(server_sender_vec_arc,
                                         server_recv_vec_arc,
                                         is_active,
                                         num_cpu_threads.try_into().unwrap());
        });

        let mut handles = vec![];
        for thread_idx in 0..num_cpu_threads {
            let vm_sender_mutex_clone = vm_sender_vec_arc.clone();
            let vm_recv_mutex_clone = vm_recv_vec_arc.clone();
            let func_ptr = self.function.clone();
            let handle = thread::spawn(move || {
                let curr_time = Arc::new(Mutex::<i64>::new(0));
                let vm_idx = thread_idx.clone();
                let vm_sender_mutex = vm_sender_mutex_clone.clone();
                let vm_recv_mutex = vm_recv_mutex_clone.clone();
        		let curr_time_invoke = curr_time.clone();
                let curr_time_response = curr_time.clone();
                // main function loop
                loop {
                    // Get input from server
                    let chan = vm_recv_mutex.get(vm_idx).unwrap();
                    let (msg, msg_len, req_id) = chan.lock().unwrap().blocking_recv().unwrap();
                    // Deserialize inputs...
            		let tsc = curr_time_invoke.clone();
                    *tsc.lock().unwrap() = Utc::now().timestamp_nanos();

                    // bypass weird borrow checker stuff
                    let msg_ref = unsafe { WasmHandler::<T1, T2>::get_unsafe_mut_ref(&msg) };
                    let final_msg = unsafe { std::slice::from_raw_parts(msg_ref, msg_len) };

                    let parsed_func_input = {
                        match serializiation_format {
                            SerializationFormat::Json => {
                                serde_json::from_slice(&final_msg[..msg_len as usize]).unwrap()
                            },
                            SerializationFormat::MsgPack => {
                                decode::from_read(&final_msg[..msg_len as usize]).unwrap()
                            }
                        }
                    };
                    
                    let func_ret_val = (func_ptr)(parsed_func_input);
                    let response = match serializiation_format { 
                        SerializationFormat::Json => {
                            to_vec(&func_ret_val).unwrap()
                        },
                        SerializationFormat::MsgPack => {
                            encode::to_vec(&func_ret_val).unwrap()
                        },
                    };

                    /*
                    let response = match parsed_func_input {
                        Ok(input) => {
                            let func_ret_val = (func_ptr)(input);
                            to_string(&func_ret_val).unwrap().into_bytes()
                        },
                        Err(e) => {
                            String::from("Error occured during parsing of input").into_bytes()
                        }
                    };
                    */

                    // Respond
                    let tsc = curr_time_response.clone();
                    let device_execution_time = Utc::now().timestamp_nanos() - *tsc.lock().unwrap();
                    let chan = vm_sender_mutex.get(vm_idx).unwrap();
                    let resp_len = response.len();

                    chan.lock().unwrap().blocking_send((response,
                                                        resp_len,
                                                        device_execution_time.try_into().unwrap(), 0, 0, 0, req_id, 0)).unwrap();
                }
            });
            handles.push(handle);
        }

        for h in handles {
            h.join();
        }
    }
}
