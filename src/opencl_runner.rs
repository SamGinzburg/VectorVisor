mod vectorized_vm;
mod wasi_fd;

use wasi_fd::WasiFd;
use vectorized_vm::VectorizedVM;
use vectorized_vm::HyperCall;
use vectorized_vm::HyperCallResult;

use vectorized_vm::WasiSyscalls;

use std::ffi::CString;

use ocl::core::Event;
use ocl::core::ContextProperties;
use ocl::core::ArgVal;

use std::thread;
use crossbeam::channel::unbounded;
use crossbeam::channel::bounded;

use rayon::prelude::*;
use ocl::core::CommandQueue;

use crossbeam::channel::Sender;
use crossbeam::channel::Receiver;


pub enum VMMRuntimeStatus {
    StatusOkay,
    StatusUnknownError,
}

#[derive(Clone)]
pub struct OpenCLBuffers {
    stack_buffer: ocl::core::Mem,
    heap_buffer: ocl::core::Mem,
    stack_frames: ocl::core::Mem,
    sp: ocl::core::Mem,
    sfp: ocl::core::Mem,
    call_stack: ocl::core::Mem,
    branch_value_stack_state: ocl::core::Mem,
    loop_value_stack_state: ocl::core::Mem,
    hypercall_num: ocl::core::Mem,
    hypercall_continuation: ocl::core::Mem,
    entry: ocl::core::Mem,
}

impl OpenCLBuffers {
    pub fn new(stack_buffer: ocl::core::Mem,
               heap_buffer: ocl::core::Mem,
               stack_frames: ocl::core::Mem,
               sp: ocl::core::Mem,
               sfp: ocl::core::Mem,
               call_stack: ocl::core::Mem,
               branch_value_stack_state: ocl::core::Mem,
               loop_value_stack_state: ocl::core::Mem,
               hypercall_num: ocl::core::Mem,
               hypercall_continuation: ocl::core::Mem,
               entry: ocl::core::Mem) -> OpenCLBuffers {
                OpenCLBuffers {
                    stack_buffer: stack_buffer,
                    heap_buffer: heap_buffer,
                    stack_frames: stack_frames,
                    sp: sp,
                    sfp: sfp,
                    call_stack: call_stack,
                    branch_value_stack_state: branch_value_stack_state,
                    loop_value_stack_state: loop_value_stack_state,
                    hypercall_num: hypercall_num,
                    hypercall_continuation: hypercall_continuation,
                    entry: entry,
                }
    }

    pub fn copy(&self) -> OpenCLBuffers {
        OpenCLBuffers {
            stack_buffer: self.stack_buffer.clone(),
            heap_buffer: self.heap_buffer.clone(),
            stack_frames: self.stack_frames.clone(),
            sp: self.sp.clone(),
            sfp: self.sfp.clone(),
            call_stack: self.call_stack.clone(),
            branch_value_stack_state: self.branch_value_stack_state.clone(),
            loop_value_stack_state: self.loop_value_stack_state.clone(),
            hypercall_num: self.hypercall_num.clone(),
            hypercall_continuation: self.hypercall_continuation.clone(),
            entry: self.entry.clone(),
        }

    }
}



pub struct OpenCLRunner {
    num_vms: u32,
    program_source: String,
    is_gpu_backend: bool,
    is_memory_interleaved: bool,
    entry_point: u32,
    buffers: Option<OpenCLBuffers>
}

impl OpenCLRunner {
    pub fn new(num_vms: u32, mem_interleave: bool, running_on_gpu: bool, entry_point: u32, program: String) -> OpenCLRunner {
        OpenCLRunner {
            num_vms: num_vms,
            program_source: program,
            is_gpu_backend: running_on_gpu,
            is_memory_interleaved: mem_interleave,
            entry_point: entry_point,
            buffers: None,
        }
    }

    pub fn setup_vms(&self) -> () {
        // TODO, initialize WASI stuff here
    }

    // All of the size parameters are *per-VM* sizes, not total
    pub fn create_buffers(mut self,
                          stack_size: u32,
                          heap_size: u32,
                          stack_frame_size: u32,
                          stack_frame_ptr_size: u32,
                          call_stack_size: u32,
                          predictor_size: u32,
                          context: ocl::core::Context) -> (OpenCLRunner, ocl::core::Context) {
        let stack_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (stack_size * self.num_vms) as usize,
                                              None).unwrap()
        };

        let heap_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (heap_size * self.num_vms) as usize,
                                              None).unwrap()
        };

        /*
         * TODO: find proper sizes for the non-heap/stack buffers
         */
        let stack_frames = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (stack_frame_size * self.num_vms) as usize,
                                              None).unwrap()
        };

        // TODO: sp is currently 8 bytes? very unecessary - 4 bytes is probably enough
        let sp = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (8 * self.num_vms) as usize,
                                              None).unwrap()
        };

        // way, way too big
        let sfp = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (stack_frame_ptr_size * self.num_vms) as usize,
                                              None).unwrap()
        };

        // 1KB call stack should be way more than enough
        let call_stack = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (call_stack_size * self.num_vms) as usize,
                                              None).unwrap()
        };

        let branch_value_stack_state = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (predictor_size * self.num_vms) as usize,
                                              None).unwrap()
        };

        let loop_value_stack_state = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (predictor_size * self.num_vms) as usize,
                                              None).unwrap()
        };


        let hypercall_num = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (8 * self.num_vms) as usize,
                                              None).unwrap()
        };

        let hypercall_continuation = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (8 * self.num_vms) as usize,
                                              None).unwrap()
        };


        let entry = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (8 * self.num_vms) as usize,
                                              None).unwrap()
        };

        self.buffers = Some(OpenCLBuffers::new(stack_buffer,
                                               heap_buffer,
                                               stack_frames,
                                               sp,
                                               sfp,
                                               call_stack,
                                               branch_value_stack_state,
                                               loop_value_stack_state,
                                               hypercall_num,
                                               hypercall_continuation,
                                               entry));
        (self, context)
    }

    /*
     * This function starts up a new thread to start running vectorized VMs
     * 
     * It returns a sending channel for the HTTP Endpoint to send requests to be processed with.
     * 
     */
    pub fn setup_kernel(&self) -> (ocl::core::Program, ocl::core::Context, ocl::core::DeviceId) {
        let program = self.program_source.clone();

        // (1) Define which platform and device(s) to use. Create a context,
        // queue, and program then define some dims..
        let platform_id = ocl::core::default_platform().unwrap();
        let device_type = if true {
            Some(ocl::core::DEVICE_TYPE_GPU)
        } else {
            Some(ocl::core::DEVICE_TYPE_GPU)
        };

        let device_ids = ocl::core::get_device_ids(&platform_id, device_type, None).unwrap();
        let device_id = device_ids[0];
        println!("{:?}", platform_id);
        println!("{:?}", device_ids);
        println!("{:?}", device_id);
        // set up the device context
        let context_properties = ContextProperties::new().platform(platform_id);
        let context = ocl::core::create_context(Some(&context_properties), &[device_id], None, None).unwrap();        

        // compile the GPU kernel(s)
        let src_cstring = CString::new(program.clone()).unwrap();
        let compiled_program = ocl::core::create_program_with_source(&context, &[src_cstring]).unwrap();
        let compile_result = ocl::core::build_program(&compiled_program, None::<&[()]>, &CString::new(format!("-DNUM_THREADS={}", self.num_vms)).unwrap(), None, None);
        match compile_result {
            Err(e) => {
                println!("Compilation error:\n{}", e);
                println!("\n\nWriting source to output file test.cl\n");
                std::fs::write("test.cl", program).expect("Unable to write file");
                panic!("Unable to compile OpenCL kernel - see errors above");
            },
            Ok(_) => (),
        }

        return (compiled_program, context, device_id)
    }

    /*
     * This function actually runs the vectorized VMs, it spins off a thread that
     * sits inside of a while loop, waiting for input to be sent to it on a channel.
     * 
     */
    pub fn run_vector_vms(self: &'static OpenCLRunner, per_vm_stack_frames_size: u32, program: ocl::core::Program, context: ocl::core::Context, device_id: ocl::core::DeviceId, queue: &'static CommandQueue) -> (VMMRuntimeStatus) {
        // we have the compiled program & context, we now can set up the kernels...
        let data_kernel = ocl::core::create_kernel(&program, "data_init").unwrap();
        let start_kernel = ocl::core::create_kernel(&program, "wasm_entry").unwrap();

        let mut stack_pointer_temp = vec![0u64; self.num_vms as usize];
        let mut entry_point_temp = vec![0u32; self.num_vms as usize];
        let mut hypercall_num_temp = vec![0i32; self.num_vms as usize];
        // this is for debugging only...
        let mut check_results_debug = vec![0u8; 100 as usize];
        let mut sp_exit_flag;
        let mut entry_point_exit_flag;
        let vm_slice: Vec<u32> = std::ops::Range { start: 0, end: (self.num_vms) }.collect();
        let mut hypercall_sender = vec![];

        /* 
         * Start up N threads to serve WASI hypercalls
         * 
         * The WASI contexts are not thread safe, so we partition the VMs evenly across each
         * thread, to ensure an even workload. Thread 0 = (0...N/4) VMs, where N = total number of VMs
         * 
         * The indexing is as follows:
         * 
         * vm_id % N => This gets the index in hypercall_sender to send to
         * 
         * Then, inside of each thread, the vm_id % (N/4) gets the WASI context
         * 
         */
        let number_vms = self.num_vms.clone();
        let num_threads = 4;
        let (result_sender, result_receiver): (Sender<HyperCallResult>, Receiver<HyperCallResult>) = bounded(0);
        for _idx in 0..num_threads {
            let (sender, recv): (Sender<HyperCall>, Receiver<HyperCall>) = unbounded();
            let sender_copy = result_sender.clone();
            hypercall_sender.push(sender.clone());
            thread::spawn(move || {
                let receiver = recv.clone();
                // create the WASI contexts for this thread
                let mut wasi_ctxs = vec![];
                for vm in 0..number_vms/num_threads {
                    wasi_ctxs.push(VectorizedVM::new(vm));
                }

                loop {
                    // in the primary loop, we will block until someone sends us a hypercall
                    // to dispatch...
                    let incoming_msg = match receiver.recv() {
                        Ok(m) => m,
                        _ => {
                            // if the main sending thread is closed, we will get an error
                            // we are handling that error elsewhere, so we can just exit the thread in that case
                            break;
                        },
                    };
                    // get the WASI context
                    let wasi_context = &wasi_ctxs[(incoming_msg.vm_id % (num_threads/4)) as usize];

                    wasi_context.dispatch_hypercall(&incoming_msg, &sender_copy.clone());
                }
            });    
        }

        let buffers = match &self.buffers {
            Some(b) => b,
            _ => panic!("run_vector_vms called before allocating buffers for kernels..."),
        };

        println!("{:?}", buffers.stack_buffer);

        let mut default_sp: [u8; 8] = unsafe { std::mem::transmute((0 as u64).to_be()) };
        let mut default_hypercall_num: [u8; 4] = unsafe { std::mem::transmute((-2 as i32).to_be()) };
        // points to _start
        let mut default_entry_point: [u8; 4] = unsafe { std::mem::transmute((self.entry_point as i32).to_be()) };
        // Important!! std::mem::transmute puts the bytes in the reverse order, we have to change it back!
        default_entry_point.reverse();
        default_sp.reverse();
        default_hypercall_num.reverse();

        println!("{:?}", default_entry_point);
        // first, set up the default values for the VMs
        unsafe {
            for idx in 0..self.num_vms {
                println!("setting up VM: {}", idx);
                // sizeof(ulong) * 8 - NOTE: if we update sp to be 4 bytes, we have to change this too
                let sp_result = ocl::core::enqueue_write_buffer(&queue, &buffers.sp, true, (idx * 8) as usize, &default_sp, None::<Event>, None::<&mut Event>);
            
                match sp_result {
                    Err(e) => panic!("sp_result, Error: {}", e),
                    _ => (),
                }

                // set the stack frame: stack_frames[sfp - 1] = sp
                let stack_frame_result = ocl::core::enqueue_write_buffer(&queue, &buffers.stack_frames, true, (idx * per_vm_stack_frames_size) as usize, &default_sp, None::<Event>, None::<&mut Event>);

                match stack_frame_result {
                    Err(e) => panic!("stack_frame_result, Error: {}", e),
                    _ => (),
                }

                // set the entry point!
                let entry_point_result = ocl::core::enqueue_write_buffer(&queue, &buffers.entry, true, (idx * 4) as usize, &default_entry_point, None::<Event>, None::<&mut Event>);


                match entry_point_result {
                    Err(e) => panic!("entry_point_result, Error: {}", e),
                    _ => (),
                }

                // set the default hypercall number to -2
                let hypercall_num_result = ocl::core::enqueue_write_buffer(&queue, &buffers.hypercall_num, true, (idx * 4) as usize, &default_hypercall_num, None::<Event>, None::<&mut Event>);
            
                match hypercall_num_result {
                    Err(e) => panic!("hypercall_num_result, Error: {}", e),
                    _ => (),
                }
            }
        }

        // run the data kernel to init the memory
        ocl::core::set_kernel_arg(&data_kernel, 0, ArgVal::mem(&buffers.heap_buffer)).unwrap();
        unsafe {
            ocl::core::enqueue_kernel(&queue, &data_kernel, 1, None, &[self.num_vms as usize, 1, 1], None, None::<Event>, None::<&mut Event>).unwrap();
            match ocl::core::finish(&queue) {
                Err(e) => {
                    panic!("Unable to finish waiting on queue for data_kernel\n\n{}", e);
                },
                Ok(_) => (),
            }
        }

        // set up the clArgs for the wasm_entry kernel
        ocl::core::set_kernel_arg(&start_kernel, 0, ArgVal::mem(&buffers.stack_buffer)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 1, ArgVal::mem(&buffers.stack_buffer)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 2, ArgVal::mem(&buffers.heap_buffer)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 3, ArgVal::mem(&buffers.heap_buffer)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 4, ArgVal::mem(&buffers.stack_frames)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 5, ArgVal::mem(&buffers.sp)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 6, ArgVal::mem(&buffers.sfp)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 7, ArgVal::mem(&buffers.call_stack)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 8, ArgVal::mem(&buffers.branch_value_stack_state)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 9, ArgVal::mem(&buffers.loop_value_stack_state)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 10, ArgVal::mem(&buffers.hypercall_num)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 11, ArgVal::mem(&buffers.hypercall_continuation)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 12, ArgVal::mem(&buffers.entry)).unwrap();

        // now the data in the program has been initialized, we can run the main loop
        loop {
            // run the kernel!
            // warning - bugged kernels can cause GPU driver hangs! Will result in the driver restarting...
            // Hangs are frequently a sign of a segmentation fault from inside of the GPU kernel
            // Unfortunately the OpenCL API doesn't give us a good way to identify what happened - the OS logs (dmesg) do have a record of this though
            unsafe {
                ocl::core::enqueue_kernel(&queue, &start_kernel, 1, None, &[self.num_vms as usize, 1, 1], None, None::<Event>, None::<&mut Event>).unwrap();
                match ocl::core::finish(&queue) {
                    Err(e) => {
                        panic!("Unable to finish waiting on queue for data_kernel\n\n{}", e);
                    },
                    Ok(_) => (),
                }
            }

            // upon exiting we check the stack pointer for each VM
            unsafe {
                ocl::core::enqueue_read_buffer(&queue, &buffers.sp, true, 0, &mut stack_pointer_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_read_buffer(&queue, &buffers.entry, true, 0, &mut entry_point_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_read_buffer(&queue, &buffers.hypercall_num, true, 0, &mut hypercall_num_temp, None::<Event>, None::<&mut Event>).unwrap();
            }

            sp_exit_flag = true;
            for sp in &stack_pointer_temp {
                sp_exit_flag = (*sp == (0 as u64)) & sp_exit_flag;
            }

            // if all (sp) == 0, exit
            if sp_exit_flag {
                break;
            }

            // if all entry_point == -1, also exit
            entry_point_exit_flag = true;
            for e in &entry_point_temp {
                entry_point_exit_flag = (*e as i32 == (-1)) & entry_point_exit_flag;
            }

            if entry_point_exit_flag {
                break;
            }

            // now it is time to dispatch hypercalls
            vm_slice.as_slice().par_iter().for_each(|vm_id| {
                let hypercall_id = match hypercall_num_temp[*vm_id as usize] as i64 {
                    0 => WasiSyscalls::FdWrite,
                    1 => WasiSyscalls::ProcExit,
                    _ => WasiSyscalls::InvalidHyperCallNum,
                };

                hypercall_sender[(vm_id % 4) as usize].send(
                    HyperCall::new(*vm_id,
                                   stack_pointer_temp[*vm_id as usize],
                                   hypercall_id,
                                   self.is_memory_interleaved.clone(),
                                   &buffers,
                                   None,
                                   None,
                                   queue)
                ).unwrap();
            });

            // now block until all of the hypercalls have been successfully dispatched
            for _ in 0..self.num_vms {
                let result = result_receiver.recv().unwrap();
                // we want to special case proc_exit to exit the VM
                match result.get_type() {
                    WasiSyscalls::ProcExit => entry_point_temp[result.get_vm_id() as usize] = ((-1) as i32) as u32,
                    _ => (),
                }
            }

            // after all of the hypercalls are finished, we should update all of the stack pointers
            // also we need to set the hypercall num to -1 to indicate we are resuming the continuation
            for idx in 0..self.num_vms {
                stack_pointer_temp[idx as usize] += 1;
                hypercall_num_temp[idx as usize] = -1;
            }

            // check again for threads that may be done - this is because
            // proc_exit(...) can actually block off additional threads
            // we don't have to read again, we can have proc_exit write directly to entry_point_temp
            entry_point_exit_flag = true;
            for e in &entry_point_temp {
                entry_point_exit_flag = (*e as i32 == (-1)) & entry_point_exit_flag;
            }

            if entry_point_exit_flag {
                break;
            }

            // now set the entry_point of exited procs to -1 if sp == 0
            for (idx, sp) in stack_pointer_temp.iter().enumerate() {
                if *sp == 0 as u64 {
                    // this cast is hacky, but it does the C equivalent of (uint)(-1)
                    entry_point_temp[idx] = ((-1) as i32) as u32;
                }
            }

            // update the entry point to resume execution
            // update all of the stack pointers
            // update the hypercall numbers to -1 to indicate that we are now returning from the hypercall

            unsafe {
                ocl::core::enqueue_write_buffer(&queue, &buffers.entry, false, 0, &mut entry_point_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_write_buffer(&queue, &buffers.sp, false, 0, &mut stack_pointer_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_write_buffer(&queue, &buffers.hypercall_num, false, 0, &mut hypercall_num_temp, None::<Event>, None::<&mut Event>).unwrap();
                match ocl::core::finish(&queue) {
                    Err(e) => {
                        panic!("Unable to finish waiting on queue for write_buffer cmds\n\n{}", e);
                    },
                    Ok(_) => (),
                }
            }

        }

        /*
        // To get final results back from the stack if we want for debugging stuff
        unsafe {
            ocl::core::enqueue_read_buffer(&queue, &buffers.stack_buffer, true, 0, &mut check_results_debug, None::<Event>, None::<&mut Event>).unwrap();
        }

        for item in check_results_debug {
            println!("END LOOP: {}", item);
        }
        */


        return VMMRuntimeStatus::StatusOkay;
    }

}