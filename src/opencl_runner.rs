mod vectorized_vm;

use vectorized_vm::VectorizedVM;
use std::ffi::CString;

use ocl::core::Event;
use ocl::core::ContextProperties;
use ocl::core::ArgVal;

pub enum VMMRuntimeStatus {
    STATUS_OKAY,
    STATUS_UNKNOWN_ERROR,
}

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
}

pub struct OpenCLRunner<'a> {
    num_vms: u32,
    vms: Vec<VectorizedVM<'a>>,
    program_source: String,
    is_gpu_backend: bool,
    is_memory_interleaved: bool,
    entry_point: u32,
    buffers: Option<OpenCLBuffers>,
}

impl<'a> OpenCLRunner<'a> {
    pub fn new(num_vms: u32, mem_interleave: bool, running_on_gpu: bool, entry_point: u32, program: String) -> OpenCLRunner<'a> {
        OpenCLRunner {
            num_vms: num_vms,
            vms: vec!(),
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
                          context: ocl::core::Context) -> (OpenCLRunner<'a>, ocl::core::Context) {
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
    pub fn run_vector_vms(&self, per_vm_stack_frames_size: u32, program: ocl::core::Program, context: ocl::core::Context, device_id: ocl::core::DeviceId) -> (VMMRuntimeStatus) {
        // we have the compiled program & context, we now can set up the kernels...
        let data_kernel = ocl::core::create_kernel(&program, "data_init").unwrap();
        let start_kernel = ocl::core::create_kernel(&program, "wasm_entry").unwrap();
        let queue = ocl::core::create_command_queue(&context, &device_id, None).unwrap();
        let mut stack_pointer_temp = vec![0u64; self.num_vms as usize];
        let mut entry_point_temp = vec![0u32; self.num_vms as usize];
        let mut hypercall_num_temp = vec![0u64; self.num_vms as usize];
        // this is for debugging only...
        let mut check_results_debug = vec![0u8; 100 as usize];
        let mut sp_exit_flag;
        let mut entry_point_exit_flag;

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
                println!("sp: {}", sp);
                sp_exit_flag = (*sp == (0 as u64)) & sp_exit_flag;
            }

            // if all (sp) == 0, exit
            if sp_exit_flag {
                break;
            }

            // if all entry_point == -1, also exit
            entry_point_exit_flag = true;
            for e in &entry_point_temp {
                println!("entry: {}", *e as i32);
                entry_point_exit_flag = (*e as i32 == (-1)) & entry_point_exit_flag;
            }

            if entry_point_exit_flag {
                break;
            }

            // now it is time to dispatch hypercalls
            for hc in &hypercall_num_temp {
                println!("hypercall_num: {}", hc);
            }

            // check again for threads that may be done - this is because
            // proc_exit(...) can actually block off additional threads
            // we don't have to read again, we can have proc_exit write directly to entry_point_temp
            entry_point_exit_flag = true;
            for e in &entry_point_temp {
                println!("entry: {}", *e as i32);
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

            unsafe {
                ocl::core::enqueue_write_buffer(&queue, &buffers.entry, true, 0, &mut entry_point_temp, None::<Event>, None::<&mut Event>).unwrap();
            }
        }

        unsafe {
            ocl::core::enqueue_read_buffer(&queue, &buffers.stack_buffer, true, 0, &mut check_results_debug, None::<Event>, None::<&mut Event>).unwrap();
        }

        for item in check_results_debug {
            println!("{}", item);
        }


        return VMMRuntimeStatus::STATUS_OKAY;
    }

}