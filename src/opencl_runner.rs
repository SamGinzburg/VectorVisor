mod vectorized_vm;
mod wasi_fd;
mod interleave_offsets;
mod environment;

use wasi_fd::WasiFd;
use vectorized_vm::VectorizedVM;
use vectorized_vm::HyperCall;
use vectorized_vm::HyperCallResult;
use vectorized_vm::WasiSyscalls;

use interleave_offsets::Interleave;

use std::ffi::CString;

use ocl::core::Event;
use ocl::core::ContextProperties;
use ocl::core::ArgVal;

use std::thread;
use std::collections::HashMap;
use crossbeam::channel::unbounded;
use crossbeam::channel::bounded;

use rayon::prelude::*;
use ocl::core::CommandQueue;
use ocl::core::CommandQueueProperties;

use crossbeam::channel::Sender;
use crossbeam::channel::Receiver;

use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;
use serde::{Serialize, Deserialize};
use bincode;

use byteorder::LittleEndian;
use byteorder::ByteOrder;

use chrono::{Datelike, Timelike, Utc};


pub enum VMMRuntimeStatus {
    StatusOkay,
    StatusUnknownError,
}

#[derive(Clone)]
pub struct OpenCLBuffers {
    stack_buffer: ocl::core::Mem,
    heap_buffer: ocl::core::Mem,
    stack_frames: ocl::core::Mem,
    globals_buffer: ocl::core::Mem,
    sp: ocl::core::Mem,
    sfp: ocl::core::Mem,
    call_stack: ocl::core::Mem,
    call_return_stack: ocl::core::Mem,
    branch_value_stack_state: ocl::core::Mem,
    loop_value_stack_state: ocl::core::Mem,
    hypercall_num: ocl::core::Mem,
    hypercall_continuation: ocl::core::Mem,
    current_mem: ocl::core::Mem,
    max_mem: ocl::core::Mem,
    is_calling: ocl::core::Mem,
    entry: ocl::core::Mem,
}

impl OpenCLBuffers {
    pub fn new(stack_buffer: ocl::core::Mem,
               heap_buffer: ocl::core::Mem,
               stack_frames: ocl::core::Mem,
               globals_buffer: ocl::core::Mem,
               sp: ocl::core::Mem,
               sfp: ocl::core::Mem,
               call_stack: ocl::core::Mem,
               call_return_stack: ocl::core::Mem,
               branch_value_stack_state: ocl::core::Mem,
               loop_value_stack_state: ocl::core::Mem,
               hypercall_num: ocl::core::Mem,
               hypercall_continuation: ocl::core::Mem,
               current_mem: ocl::core::Mem,
               max_mem: ocl::core::Mem,
               is_calling: ocl::core::Mem,
               entry: ocl::core::Mem) -> OpenCLBuffers {
                OpenCLBuffers {
                    stack_buffer: stack_buffer,
                    heap_buffer: heap_buffer,
                    stack_frames: stack_frames,
                    globals_buffer: globals_buffer,
                    sp: sp,
                    sfp: sfp,
                    call_stack: call_stack,
                    call_return_stack: call_return_stack,
                    branch_value_stack_state: branch_value_stack_state,
                    loop_value_stack_state: loop_value_stack_state,
                    hypercall_num: hypercall_num,
                    hypercall_continuation: hypercall_continuation,
                    current_mem: current_mem,
                    max_mem: max_mem,
                    is_calling: is_calling,
                    entry: entry,
                }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SeralizedProgram {
    pub program_data: Vec<u8>,
    pub entry_point: u32,
    pub num_compiled_funcs: u32,
    pub globals_buffer_size: u32,
    pub interleaved: bool,
}

#[derive(Clone)]
pub enum InputProgram {
    binary(Vec<u8>),
    text(String),
    partitioned(HashMap<u32, String>),
}

#[derive(Clone)]
pub enum ProgramType {
    Standard(ocl::core::Program),
    Partitioned(HashMap<u32, ocl::core::Program>),
}

pub struct OpenCLRunner {
    num_vms: u32,
    input_program: InputProgram,
    is_gpu_backend: bool,
    is_memory_interleaved: bool,
    entry_point: u32,
    buffers: Option<OpenCLBuffers>
}

impl OpenCLRunner {
    pub fn new(num_vms: u32, mem_interleave: bool, running_on_gpu: bool, entry_point: u32, program: InputProgram) -> OpenCLRunner {
        OpenCLRunner {
            num_vms: num_vms,
            input_program: program,
            is_gpu_backend: running_on_gpu,
            is_memory_interleaved: mem_interleave,
            entry_point: entry_point,
            buffers: None,
        }
    }

    pub fn run(self,
               input_filename: &str,
               stack_size: u32,
               heap_size: u32,
               call_stack_size: u32,
               stack_frames_size: u32,
               sfp_size: u32,
               // needed for the size of the loop/branch data structures
               num_compiled_funcs: u32,
               globals_buffer_size: u32,
               compile_flags: String,
               link_flags: String,
               print_return: bool) -> JoinHandle<()> {
        let num_vms = self.num_vms.clone();
        let (program, context, device_id) = self.setup_kernel(input_filename, num_compiled_funcs, globals_buffer_size, compile_flags, link_flags);

        // create the buffers
        let (new_runner, context) = self.create_buffers(stack_size,
                                                        heap_size, 
                                                        call_stack_size, 
                                                        stack_frames_size, 
                                                        sfp_size, 
                                                        num_compiled_funcs,
                                                        globals_buffer_size,
                                                        context);

        let handler = std::thread::spawn(move || {
            // this function returns the channel that we will use to send it HTTP requests later


            // each vector VMM group gets its own command queue - in the future we may have 1 queue per [Large N] number of VMs
            let properties = CommandQueueProperties::new().profiling();
            let command_queue = ocl::core::create_command_queue(&context, &device_id, Some(properties)).unwrap();

            // We purposefully leak the runner into a static object to deal with the lifetimes of the
            // hypercall dispatch thread pools, we will clean up the new_runner object if needed
            // These values really do last for the entire program, so it is fine to make them static
            let final_runner = Box::leak(Box::new(new_runner));
            let leaked_command_queue: &'static CommandQueue = Box::leak(Box::new(command_queue));
            let hypercall_buffer_read_buffer: &'static mut [u8] = Box::leak(vec![0u8; 16 * 1024 * num_vms as usize].into_boxed_slice());

            // decide which vector runner to use based off the compiled program enum...
            let status = match program {
                ProgramType::Standard(program) => {
                    final_runner.run_vector_vms(stack_frames_size, program, &leaked_command_queue, hypercall_buffer_read_buffer, 1024*16, context, print_return)
                },
                ProgramType::Partitioned(program_map) => {
                    final_runner.run_partitioned_vector_vms(stack_frames_size, program_map, &leaked_command_queue, hypercall_buffer_read_buffer, 1024*16, context, print_return)
                }
            };

            // this line being reached means either:
            // 1) The VMM has exited normally
            // 2) The VMM has exited prematurely due to a crash
            match status {
                VMMRuntimeStatus::StatusUnknownError => panic!("Vector VMM has crashed!!!"),
                VMMRuntimeStatus::StatusOkay => (),
            }

            // In the future if we want to make this dynamic, we need to cleanup the leaked objects

        });
        handler
    }

    // All of the size parameters are *per-VM* sizes, not total
    pub fn create_buffers(mut self,
                          stack_size: u32,
                          heap_size: u32,
                          stack_frame_size: u32,
                          stack_frame_ptr_size: u32,
                          call_stack_size: u32,
                          // needed for loop/branch structures
                          num_compiled_funcs: u32,
                          global_buffers_size: u32,
                          context: ocl::core::Context) -> (OpenCLRunner, ocl::core::Context) {
        let mut size_tracker: u64 = 0;
        
        let stack_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              //ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                                              ocl::core::MEM_READ_WRITE,
                                              (stack_size as u64 * self.num_vms as u64) as usize,
                                              None).unwrap()
        };
        size_tracker += (stack_size as u64 * self.num_vms as u64) as u64;

        let heap_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              //ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                                              ocl::core::MEM_READ_WRITE,
                                              (heap_size as u64 * self.num_vms as u64) as usize,
                                              None).unwrap()
        };
        size_tracker += (heap_size as u64 * self.num_vms as u64) as u64;


        let globals_buffer = unsafe {
            if global_buffers_size > 0 {
                size_tracker += (global_buffers_size * 4 * self.num_vms) as u64;
                ocl::core::create_buffer::<_, u8>(&context,
                    ocl::core::MEM_READ_WRITE,
                    // global_buffers_size is in increments of 4 bytes
                    (global_buffers_size * 4 * self.num_vms) as usize,
                    None).unwrap()
            } else {
                size_tracker += (1) as u64;
                // just to get by, create a buffer of size 1 that we will never use
                ocl::core::create_buffer::<_, u8>(&context,
                    ocl::core::MEM_READ_WRITE,
                    // global_buffers_size is in increments of 4 bytes
                    1,
                    None).unwrap()
            }
        };

        /*
         * TODO: find proper sizes for the non-heap/stack buffers
         */
        let stack_frames = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (stack_frame_size as u64 * 4 * self.num_vms as u64) as usize,
                                              None).unwrap()
        };
        size_tracker += (stack_frame_size as u64 * 4 * self.num_vms as u64) as u64;

        // TODO: sp is currently 8 bytes? very unecessary - 4 bytes is probably enough
        let sp = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (8 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (8 * self.num_vms) as u64;

        // way, way too big
        let sfp = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (stack_frame_ptr_size * 8 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (stack_frame_ptr_size * 8 * self.num_vms) as u64;

        // 1KB call stack should be way more than enough
        let call_stack = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (call_stack_size * 8 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (call_stack_size * 8 * self.num_vms) as u64;

        let call_return_stack = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (call_stack_size * 8 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (call_stack_size * 8 * self.num_vms) as u64;

        // max supported call stack depth of 256 calls
        // TODO: make max call stack depth configurable
        // we can store up to 128 loops and 128 branches within a func
        let branch_value_stack_state = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (64 * 512 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (64 * 512 * self.num_vms) as u64;

        let loop_value_stack_state = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (64 * 512 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (64 * 512 * self.num_vms) as u64;

        let hypercall_num = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (4 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (4 * self.num_vms) as u64;

        let hypercall_continuation = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (4 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (4 * self.num_vms) as u64;

        let current_mem = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (4 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (4 * self.num_vms) as u64;

        let max_mem = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (4 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (4 * self.num_vms) as u64;


        let entry = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (4 * self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (4 * self.num_vms) as u64;

        let is_calling = unsafe {
            ocl::core::create_buffer::<_, u8>(&context,
                                              ocl::core::MEM_READ_WRITE,
                                              (self.num_vms) as usize,
                                              None).unwrap()
        };
        size_tracker += (self.num_vms) as u64;

        println!("Allocated: {:.2} MB in OpenCL Buffers", size_tracker as f64 / 1024.0 / 1024.0);

        self.buffers = Some(OpenCLBuffers::new(stack_buffer,
                                               heap_buffer,
                                               stack_frames,
                                               globals_buffer,
                                               sp,
                                               sfp,
                                               call_stack,
                                               call_return_stack,
                                               branch_value_stack_state,
                                               loop_value_stack_state,
                                               hypercall_num,
                                               hypercall_continuation,
                                               current_mem,
                                               max_mem,
                                               is_calling,
                                               entry));
        (self, context)
    }

    /*
     * This function starts up a new thread to start running vectorized VMs
     * 
     * It returns a sending channel for the HTTP Endpoint to send requests to be processed with.
     * 
     */
    pub fn setup_kernel(&self, input_filename: &str, num_compiled_funcs: u32, globals_buffer_size: u32, compile_flags: String, link_flags: String) -> (ProgramType, ocl::core::Context, ocl::core::DeviceId) {
        let platform_id = ocl::core::default_platform().unwrap();
        let device_type = if self.is_gpu_backend {
            Some(ocl::core::DEVICE_TYPE_GPU)
        } else {
            Some(ocl::core::DEVICE_TYPE_CPU)
        };

        let device_ids = ocl::core::get_device_ids(&platform_id, device_type, None).unwrap();
        let device_id = device_ids[0];
        println!("{:?}", platform_id);
        println!("{:?}", device_ids);
        println!("{:?}", device_id);
        // set up the device context
        let context_properties = ContextProperties::new().platform(platform_id);
        let context = ocl::core::create_context(Some(&context_properties), &[device_id], None, None).unwrap();        

        let dev_type = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::Type);
        let dev_name = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::Name);
        let vendor = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::Vendor);
        let ocl_version = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::Version);
        let ocl_c_version = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::OpenclCVersion);
        let compute_units = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::MaxComputeUnits);
        let max_param_size = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::MaxParameterSize);
        let max_global_mem_size = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::GlobalMemSize);
        let max_constant_buffer_size = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::MaxConstantBufferSize);
        let linker_available = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::LinkerAvailable);
        let extensions = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::Extensions);

        println!("Device type: {:?}", dev_type);
        println!("Device name: {:?}", dev_name);
        println!("Vendor: {:?}", vendor);
        println!("OpenCL Version: {:?}", ocl_version);
        println!("OpenCL C Version: {:?}", ocl_c_version);
        println!("Num. compute units: {:?}", compute_units);
        println!("Max param size: {:?}", max_param_size);
        println!("Max global mem size: {:?}", max_global_mem_size);
        println!("Max constant buffer size: {:?}", max_constant_buffer_size);
        println!("Linker available: {:?}", linker_available);
        println!("OpenCL Extensions: {:?}", extensions);

        // compile the GPU kernel(s)
        let program_to_run = match &self.input_program {
            InputProgram::text(program) => {
                let src_cstring = CString::new(program.clone()).unwrap();

                println!("Sucessfully compiled kernel to OpenCL C: saving to: {}", format!("{}.cl", input_filename));
                let mut file = File::create(format!("{}.cl", input_filename)).unwrap();
                file.write_all(&program.clone().into_bytes()).unwrap();
                println!("Starting kernel compilation...");
                let compile_start = std::time::Instant::now();

                let compiled_program = ocl::core::create_program_with_source(&context, &[src_cstring.clone()]).unwrap();
                let build_result = ocl::core::compile_program(&compiled_program, Some(&[device_ids[0]]), &CString::new(format!("{} -DNUM_THREADS={}", compile_flags, self.num_vms)).unwrap(), &[], &[], None, None, None);
                match build_result {
                    Err(e) => {
                        println!("Build error:\n{}", e);
                        println!("\n\nWriting source to output file test.cl\n");
                        std::fs::write("test.cl", program).expect("Unable to write file");
                        panic!("Build failure: {:?}", e);
                    }
                    Ok(_) => {
                    },
                };
                let buildinfo = ocl::core::get_program_build_info(&compiled_program, &device_ids[0], ocl::core::ProgramBuildInfo::BuildLog).unwrap();
                dbg!(buildinfo);
                let compile_end = std::time::Instant::now();
                println!("Compile time for kernel: {:?}", compile_end-compile_start);

                println!("Now linking program...");

                let link_start = std::time::Instant::now();
                let final_program = ocl::core::link_program(&context, Some(&[device_ids[0]]), &CString::new(format!("{}", link_flags)).unwrap(), &[&compiled_program], None, None, None);
                let link_end = std::time::Instant::now();
                println!("Link time for kernel: {:?}", link_end-link_start);

                match final_program {
                    Err(e) => {
                        println!("Link error:\n{}", e);
                        println!("\n\nWriting source to output file test.cl\n");
                        std::fs::write("test.cl", program).expect("Unable to write file");
                        panic!("Unable to compile OpenCL kernel - see errors above");
                    },
                    Ok(_) => println!("Finished kernel compilation!"),
                }

                // if we are going to save the program, we save the binary here
                let program_to_save = final_program.unwrap();
                let saved_binary = ocl::core::get_program_info(&program_to_save, ocl::core::ProgramInfo::Binaries);
                let binary = match saved_binary.unwrap() {
                    ocl::core::types::enums::ProgramInfoResult::Binaries(binary_vec) => binary_vec.get(0).unwrap().clone(),
                    _ => panic!("Incorrect result from get_program_info"),
                };
                /*
                 * When packaging the binary, we also have to save program metadata such as:
                 * 1) the program entry_point
                 * 2) global buffer size
                 * 3) number of total compiled functions
                 * 
                 * Note: The binary built here is not necessarily portable across GPUs
                 */
                let program_to_serialize = SeralizedProgram {
                    program_data: binary,
                    globals_buffer_size: globals_buffer_size,
                    entry_point: self.entry_point,
                    num_compiled_funcs: num_compiled_funcs,
                    interleaved: self.is_memory_interleaved,
                };

                let serialized_program = bincode::serialize(&program_to_serialize).unwrap();
                let mut file = File::create(format!("{}.bin", input_filename)).unwrap();
                file.write_all(&serialized_program).unwrap();

                // while we are at it, might as well save the input .cl file as well
                let mut file = File::create(format!("{}.cl", input_filename)).unwrap();
                file.write_all(&program.clone().into_bytes()).unwrap();

                ProgramType::Standard(program_to_save)
            },
            InputProgram::binary(b) => {
                let binary_start = std::time::Instant::now();

                let program_to_run = match ocl::core::create_program_with_binary(&context, &[device_ids[0]], &[&b]) {
                    Ok(binary) => binary,
                    Err(e) => panic!("Unable to create program from given binary: {:?}", e),
                };
                ocl::core::build_program(&program_to_run, Some(&[device_ids[0]]), &CString::new(format!("{} -DNUM_THREADS={}", compile_flags, self.num_vms)).unwrap(), None, None).unwrap();
                let knames = ocl::core::get_program_info(&program_to_run, ocl::core::ProgramInfo::KernelNames);
                println!("Loaded kernels: {}", knames.unwrap());
                let binary_prep_end = std::time::Instant::now();
                println!("Time to load program from binary: {:?}", binary_prep_end-binary_start);
                ProgramType::Standard(program_to_run)
            },
            InputProgram::partitioned(map) => {
                let mut final_hashmap: HashMap<u32, ocl::core::Program> = HashMap::new();
                let kernel_compile = std::time::Instant::now();

                // Spin up N threads (n = ncpus)
                // Evenly divide workload between threads
                let num_threads = num_cpus::get();
                let num_vms = self.num_vms.clone();
                let device_id = device_ids[0];
                
                let (finished_sender, finished_receiver): (Sender<(u32, ocl::core::Program)>, Receiver<(u32, ocl::core::Program)>) = unbounded();

                let mut submit_compile_job = vec![];

                for _idx in 0..num_threads {
                    let (compile_sender, compile_receiver): (Sender<(u32, ocl::core::Program)>, Receiver<(u32, ocl::core::Program)>) = unbounded();
                    let cflags = compile_flags.clone();
                    let sender = finished_sender.clone();
                    submit_compile_job.push(compile_sender);

                    thread::spawn(move || {
                        let receiver = compile_receiver.clone();
                        loop {
                            // receive the function to compile
                            let (key, program_to_build) = match receiver.recv() {
                                Ok(m) => m,
                                _ => {
                                    // if the main sending thread is closed, we will get an error
                                    // we are handling that error elsewhere, so we can just exit the thread in that case
                                    break;
                                },
                            };
                            ocl::core::build_program(&program_to_build, Some(&[device_id]), &CString::new(format!("{} -DNUM_THREADS={}", cflags.clone(), num_vms)).unwrap(), None, None).unwrap();
                            sender.send((key, program_to_build)).unwrap();
                        }
                    });
                }

                // for each function, submit it to be compiled
                let mut counter = 0;
                for (key, value) in map.iter() {
                    let src_cstring = CString::new(value.clone()).unwrap();    
                    let compiled_program = ocl::core::create_program_with_source(&context, &[src_cstring.clone()]).unwrap();

                    submit_compile_job[counter % submit_compile_job.len() as usize].send((*key, compiled_program)).unwrap();
                    counter += 1;
                }

                for _idx in 0..map.len() {
                    let (key, compiled_program) = finished_receiver.recv().unwrap();
                    final_hashmap.insert(key, compiled_program);
                }

                let kernel_compile_end = std::time::Instant::now();

                println!("Time to compile all functions: {:?}", kernel_compile_end-kernel_compile);


                ProgramType::Partitioned(final_hashmap)
            }
        };

        return (program_to_run, context, device_id)
    }

    /*
     * This function actually runs the vectorized VMs, it spins off a thread that
     * sits inside of a while loop, waiting for input to be sent to it on a channel.
     * 
     */
    pub fn run_vector_vms(self: &'static OpenCLRunner,
                         per_vm_stack_frames_size: u32,
                         program: ocl::core::Program,
                         queue: &'static CommandQueue,
                         hypercall_buffer_read_buffer: &'static mut [u8],
                         hypercall_buffer_size: u32,
                         ctx: ocl::core::Context,
                         print_return: bool) -> VMMRuntimeStatus {
        // we have the compiled program & context, we now can set up the kernels...
        let data_kernel = ocl::core::create_kernel(&program, "data_init").unwrap();
        let start_kernel = ocl::core::create_kernel(&program, "wasm_entry").unwrap();

        let mut stack_pointer_temp = vec![0u64; self.num_vms as usize];
        let mut entry_point_temp = vec![0u32; self.num_vms as usize];
        let mut hypercall_num_temp = vec![0i32; self.num_vms as usize];
        let mut hypercall_retval_temp = vec![0i32; self.num_vms as usize];
        let mut sp_exit_flag;
        let mut entry_point_exit_flag;
        let vm_slice: Vec<u32> = std::ops::Range { start: 0, end: (self.num_vms) }.collect();
        let mut hypercall_sender = vec![];
        let hcall_read_buffer: Arc<Mutex<&mut [u8]>> = Arc::new(Mutex::new(hypercall_buffer_read_buffer));
        let mut total_gpu_execution_time: u64 = 0;
        let mut queue_submit_delta: u64 = 0;

        let mut hcall_execution_time: u128 = 0;
        let mut vmm_overhead: u128 = 0;

        /*
         * Allocate the hypercall_buffer at the last minute, 16KiB per VM
         *
         */
        let hypercall_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(&ctx,
                                              ocl::core::MEM_READ_WRITE,
                                              (hypercall_buffer_size * self.num_vms) as usize,
                                              None).unwrap()
        };

        /*
         * Allocate the buffer to return values
         */
        let hcall_retval_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(&ctx,
                                              ocl::core::MEM_READ_WRITE,
                                              (4 * self.num_vms) as usize,
                                              None).unwrap()
        };

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
        let num_threads = num_cpus::get() as u32;
        let (result_sender, result_receiver): (Sender<HyperCallResult>, Receiver<HyperCallResult>) = bounded(0);
        for _idx in 0..num_threads {
            let (sender, recv): (Sender<HyperCall>, Receiver<HyperCall>) = unbounded();
            let sender_copy = result_sender.clone();
            hypercall_sender.push(sender.clone());
            thread::spawn(move || {
                let receiver = recv.clone();
                // create the WASI contexts for this thread
                let mut wasi_ctxs = vec![];
                // we divide up the number of VMs per thread evenly
                for vm in 0..(number_vms/num_threads) {
                    wasi_ctxs.push(VectorizedVM::new(vm));
                }

                loop {
                    // in the primary loop, we will block until someone sends us a hypercall
                    // to dispatch...
                    let mut incoming_msg = match receiver.recv() {
                        Ok(m) => m,
                        _ => {
                            // if the main sending thread is closed, we will get an error
                            // we are handling that error elsewhere, so we can just exit the thread in that case
                            break;
                        },
                    };
                    // get the WASI context
                    let wasi_context = &wasi_ctxs[(incoming_msg.vm_id % (number_vms/num_threads)) as usize];

                    wasi_context.dispatch_hypercall(&mut incoming_msg, &sender_copy.clone());
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

        // start counting only when all VM init is finished
        let e2e_time_start = std::time::Instant::now();

        // run the data kernel to init the memory
        ocl::core::set_kernel_arg(&data_kernel, 0, ArgVal::mem(&buffers.heap_buffer)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 1, ArgVal::mem(&buffers.globals_buffer)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 2, ArgVal::mem(&buffers.current_mem)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 3, ArgVal::mem(&buffers.max_mem)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 4, ArgVal::mem(&buffers.is_calling)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 5, ArgVal::mem(&buffers.sfp)).unwrap();

        let mut profiling_event = ocl::Event::empty();
        unsafe {
            ocl::core::enqueue_kernel(&queue, &data_kernel, 1, None, &[self.num_vms as usize, 1, 1], None, None::<Event>, Some(&mut profiling_event)).unwrap();
        }

        ocl::core::wait_for_event(&profiling_event).unwrap();
        let start_data_kernel = profiling_event.profiling_info(ocl::enums::ProfilingInfo::Submit).unwrap().time().unwrap();
        let end_data_kernel = profiling_event.profiling_info(ocl::enums::ProfilingInfo::End).unwrap().time().unwrap();
        total_gpu_execution_time += end_data_kernel-start_data_kernel;
        println!("Finished data_init kernel");

        // set up the clArgs for the wasm_entry kernel
        ocl::core::set_kernel_arg(&start_kernel, 0, ArgVal::mem(&buffers.stack_buffer)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 1, ArgVal::mem(&buffers.stack_buffer)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 2, ArgVal::mem(&buffers.heap_buffer)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 3, ArgVal::mem(&buffers.heap_buffer)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 4, ArgVal::mem(&hypercall_buffer)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 5, ArgVal::mem(&buffers.globals_buffer)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 6, ArgVal::mem(&buffers.stack_frames)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 7, ArgVal::mem(&buffers.sp)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 8, ArgVal::mem(&buffers.sfp)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 9, ArgVal::mem(&buffers.call_stack)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 10, ArgVal::mem(&buffers.call_return_stack)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 11, ArgVal::mem(&buffers.branch_value_stack_state)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 12, ArgVal::mem(&buffers.loop_value_stack_state)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 13, ArgVal::mem(&buffers.hypercall_num)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 14, ArgVal::mem(&buffers.hypercall_continuation)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 15, ArgVal::mem(&buffers.current_mem)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 16, ArgVal::mem(&buffers.max_mem)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 17, ArgVal::mem(&buffers.is_calling)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 18, ArgVal::mem(&buffers.entry)).unwrap();
        ocl::core::set_kernel_arg(&start_kernel, 19, ArgVal::mem(&hcall_retval_buffer)).unwrap();

        // now the data in the program has been initialized, we can run the main loop
        println!("start: {}", Utc::now().timestamp());
        loop {
            // run the kernel!
            // warning - bugged kernels can cause GPU driver hangs! Will result in the driver restarting...
            // Hangs are frequently a sign of a segmentation faults from inside of the GPU kernel
            // Unfortunately the OpenCL API doesn't give us a good way to identify what happened - the OS logs (dmesg) do have a record of this though
            profiling_event = ocl::Event::empty();
            unsafe {
                ocl::core::enqueue_kernel(&queue, &start_kernel, 1, None, &[self.num_vms as usize, 1, 1], None, None::<Event>, Some(&mut profiling_event)).unwrap();
            }

            ocl::core::wait_for_event(&profiling_event).unwrap();
            let queue_start_kernel = profiling_event.profiling_info(ocl::enums::ProfilingInfo::Queued).unwrap().time().unwrap();
            let start_start_kernel = profiling_event.profiling_info(ocl::enums::ProfilingInfo::Start).unwrap().time().unwrap();
            let end_start_kernel = profiling_event.profiling_info(ocl::enums::ProfilingInfo::End).unwrap().time().unwrap();
            total_gpu_execution_time += end_start_kernel-queue_start_kernel;
            queue_submit_delta += start_start_kernel-queue_start_kernel;
            // upon exiting we check the stack pointer for each VM
            let vmm_pre_overhead = std::time::Instant::now();

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
            let vmm_pre_overhead_end = std::time::Instant::now();
            vmm_overhead += (vmm_pre_overhead_end - vmm_pre_overhead).as_nanos();


            // read the hypercall_buffer
            let start_hcall_dispatch = std::time::Instant::now();

            unsafe {
                let buf: &mut [u8] = &mut hcall_read_buffer.lock().unwrap();
                ocl::core::enqueue_read_buffer(&queue, &hypercall_buffer, true, 0, buf, None::<Event>, None::<&mut Event>).unwrap();
            }

            // now it is time to dispatch hypercalls
            vm_slice.as_slice().par_iter().for_each(|vm_id| {
                let hypercall_id = match hypercall_num_temp[*vm_id as usize] as i64 {
                    0 => WasiSyscalls::FdWrite,
                    1 => WasiSyscalls::ProcExit,
                    2 => WasiSyscalls::EnvironSizeGet,
                    3 => WasiSyscalls::EnvironGet,
                    4 => WasiSyscalls::FdPrestatGet,
                    5 => WasiSyscalls::FdPrestatDirName,
                    _ => WasiSyscalls::InvalidHyperCallNum,
                };
                hypercall_sender[(vm_id % num_threads) as usize].send(
                    HyperCall::new((*vm_id as u32).clone(),
                                   number_vms,
                                   stack_pointer_temp[*vm_id as usize],
                                   hypercall_id,
                                   self.is_memory_interleaved.clone(),
                                   &buffers,
                                   hcall_read_buffer.clone(),
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
                // after all of the hypercalls are finished, we should update all of the stack pointers
                hypercall_num_temp[result.get_vm_id() as usize] = -1;
                hypercall_retval_temp[result.get_vm_id() as usize] = result.get_result();
            }

            let end_hcall_dispatch = std::time::Instant::now();
            hcall_execution_time += (end_hcall_dispatch - start_hcall_dispatch).as_nanos();

            let vmm_post_overhead = std::time::Instant::now();

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
            // also don't forget to write the hcall buf back
            unsafe {
                let mut hcall_buf = hcall_read_buffer.lock().unwrap();
                ocl::core::enqueue_write_buffer(&queue, &hypercall_buffer, true, 0, &mut hcall_buf, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_write_buffer(&queue, &buffers.entry, true, 0, &mut entry_point_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_write_buffer(&queue, &buffers.sp, true, 0, &mut stack_pointer_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_write_buffer(&queue, &buffers.hypercall_num, true, 0, &mut hypercall_num_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_write_buffer(&queue, &hcall_retval_buffer, true, 0, &mut hypercall_retval_temp, None::<Event>, None::<&mut Event>).unwrap();
            }
            let vmm_post_overhead_end = std::time::Instant::now();
            vmm_overhead += (vmm_post_overhead_end - vmm_post_overhead).as_nanos();
        }

        let e2e_time_end = std::time::Instant::now();

        // To get final results back from the stack if we want for debugging stuff
        // only uncomment this out if you need to debug stuff, it will panic if you have too many VMs and too small of a buffer

        if print_return {
            let mut check_results_debug = vec![0u8; (self.num_vms * 1024) as usize];
            unsafe {
                ocl::core::enqueue_read_buffer(&queue, &buffers.stack_buffer, true, 0, &mut check_results_debug, None::<Event>, None::<&mut Event>).unwrap();
            }
            for vm_idx in 0..self.num_vms {
                if self.is_memory_interleaved {
                    let result_i32 = Interleave::read_u32(&mut check_results_debug, 32, self.num_vms, vm_idx);
                    let result_i64 = Interleave::read_u64(&mut check_results_debug, 32, self.num_vms, vm_idx);
                    dbg!(result_i32 as i32);
                    dbg!(result_i64 as i64);
                    dbg!(result_i64 as u64);
                    dbg!(result_i32 as f32);
                    dbg!(result_i64 as f64);
                } else {
                    let result = LittleEndian::read_u32(&check_results_debug[vm_idx as usize..(vm_idx+4) as usize]);
                    dbg!(result as i32);
                }
            }
        }

        println!("end: {}", Utc::now().timestamp());
        println!("E2E execution time in nanoseconds: {}", (e2e_time_end - e2e_time_start).as_nanos());
        println!("On device time in nanoseconds: {}", total_gpu_execution_time);
        println!("Device Start-Queue overhead in nanoseconds: {}", queue_submit_delta);
        println!("Device Overhead / Device Execution Time: {}", queue_submit_delta as f64 / total_gpu_execution_time as f64);
        println!("fraction of time on device: {}", total_gpu_execution_time as f64 / (e2e_time_end - e2e_time_start).as_nanos() as f64);
        println!("fraction of time on hcall dispatch: {}", hcall_execution_time as f64 / (e2e_time_end - e2e_time_start).as_nanos() as f64);
        println!("fraction of time on VMM overhead: {}", vmm_overhead as f64 / (e2e_time_end - e2e_time_start).as_nanos() as f64);

        return VMMRuntimeStatus::StatusOkay;
    }

    /*
     * This function runs the partitioned vector VMs
     */
    pub fn run_partitioned_vector_vms(self: &'static OpenCLRunner,
                                    per_vm_stack_frames_size: u32,
                                    program_map: HashMap<u32, ocl::core::Program>,
                                    queue: &'static CommandQueue,
                                    hypercall_buffer_read_buffer: &'static mut [u8],
                                    hypercall_buffer_size: u32,
                                    ctx: ocl::core::Context,
                                    print_return: bool) -> VMMRuntimeStatus {
        let mut kernels: HashMap<u32, ocl::core::Kernel> = HashMap::new();

        // setup the data kernel
        let data_program = program_map.get(&99999).unwrap();
        kernels.insert(99999, ocl::core::create_kernel(&data_program, "data_init").unwrap());

        // create the map of runnable kernels
        for (key, value) in program_map {
            if key != 99999 {
                kernels.insert(key, ocl::core::create_kernel(&value, "wasm_entry").unwrap());
            }
        }

        let mut stack_pointer_temp = vec![0u64; self.num_vms as usize];
        let mut entry_point_temp = vec![0u32; self.num_vms as usize];
        let mut hypercall_num_temp = vec![0i32; self.num_vms as usize];
        let mut hypercall_retval_temp = vec![0i32; self.num_vms as usize];
        let mut entry_point_exit_flag;
        let vm_slice: Vec<u32> = std::ops::Range { start: 0, end: (self.num_vms) }.collect();
        let mut hypercall_sender = vec![];
        let hcall_read_buffer: Arc<Mutex<&mut [u8]>> = Arc::new(Mutex::new(hypercall_buffer_read_buffer));
        let mut total_gpu_execution_time: u64 = 0;
        let mut queue_submit_delta: u64 = 0;

        let mut hcall_execution_time: u128 = 0;
        let mut vmm_overhead: u128 = 0;

        /*
         * Allocate the hypercall_buffer at the last minute, 16KiB per VM
         *
         */
        let hypercall_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(&ctx,
                                              ocl::core::MEM_READ_WRITE,
                                              (hypercall_buffer_size * self.num_vms) as usize,
                                              None).unwrap()
        };

        /*
         * Allocate the buffer to return values
         */
        let hcall_retval_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(&ctx,
                                              ocl::core::MEM_READ_WRITE,
                                              (4 * self.num_vms) as usize,
                                              None).unwrap()
        };

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
        let num_threads = num_cpus::get() as u32;
        let (result_sender, result_receiver): (Sender<HyperCallResult>, Receiver<HyperCallResult>) = bounded(0);
        for _idx in 0..num_threads {
            let (sender, recv): (Sender<HyperCall>, Receiver<HyperCall>) = unbounded();
            let sender_copy = result_sender.clone();
            hypercall_sender.push(sender.clone());
            thread::spawn(move || {
                let receiver = recv.clone();
                // create the WASI contexts for this thread
                let mut wasi_ctxs = vec![];
                // we divide up the number of VMs per thread evenly
                for vm in 0..(number_vms/num_threads) {
                    wasi_ctxs.push(VectorizedVM::new(vm));
                }

                loop {
                    // in the primary loop, we will block until someone sends us a hypercall
                    // to dispatch...
                    let mut incoming_msg = match receiver.recv() {
                        Ok(m) => m,
                        _ => {
                            // if the main sending thread is closed, we will get an error
                            // we are handling that error elsewhere, so we can just exit the thread in that case
                            break;
                        },
                    };
                    // get the WASI context
                    let wasi_context = &wasi_ctxs[(incoming_msg.vm_id % (number_vms/num_threads)) as usize];

                    wasi_context.dispatch_hypercall(&mut incoming_msg, &sender_copy.clone());
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

        // start counting only when all VM init is finished
        let e2e_time_start = std::time::Instant::now();

        // run the data kernel to init the memory
        let data_kernel = kernels.get(&99999).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 0, ArgVal::mem(&buffers.heap_buffer)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 1, ArgVal::mem(&buffers.globals_buffer)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 2, ArgVal::mem(&buffers.current_mem)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 3, ArgVal::mem(&buffers.max_mem)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 4, ArgVal::mem(&buffers.is_calling)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 5, ArgVal::mem(&buffers.sfp)).unwrap();

        let mut profiling_event = ocl::Event::empty();

        unsafe {
            ocl::core::enqueue_kernel(&queue, &data_kernel, 1, None, &[self.num_vms as usize, 1, 1], None, None::<Event>, Some(&mut profiling_event)).unwrap();
        }

        ocl::core::wait_for_event(&profiling_event).unwrap();
        let start_data_kernel = profiling_event.profiling_info(ocl::enums::ProfilingInfo::Submit).unwrap().time().unwrap();
        let end_data_kernel = profiling_event.profiling_info(ocl::enums::ProfilingInfo::End).unwrap().time().unwrap();
        total_gpu_execution_time += end_data_kernel-start_data_kernel;
        println!("Finished data_init kernel");

        // set up the clArgs for the wasm_entry kernel
        for (key, value) in kernels.iter() {
            if *key != 99999 {
                ocl::core::set_kernel_arg(&value, 0, ArgVal::mem(&buffers.stack_buffer)).unwrap();
                ocl::core::set_kernel_arg(&value, 1, ArgVal::mem(&buffers.stack_buffer)).unwrap();
                ocl::core::set_kernel_arg(&value, 2, ArgVal::mem(&buffers.heap_buffer)).unwrap();
                ocl::core::set_kernel_arg(&value, 3, ArgVal::mem(&buffers.heap_buffer)).unwrap();
                ocl::core::set_kernel_arg(&value, 4, ArgVal::mem(&hypercall_buffer)).unwrap();
                ocl::core::set_kernel_arg(&value, 5, ArgVal::mem(&buffers.globals_buffer)).unwrap();
                ocl::core::set_kernel_arg(&value, 6, ArgVal::mem(&buffers.stack_frames)).unwrap();
                ocl::core::set_kernel_arg(&value, 7, ArgVal::mem(&buffers.sp)).unwrap();
                ocl::core::set_kernel_arg(&value, 8, ArgVal::mem(&buffers.sfp)).unwrap();
                ocl::core::set_kernel_arg(&value, 9, ArgVal::mem(&buffers.call_stack)).unwrap();
                ocl::core::set_kernel_arg(&value, 10, ArgVal::mem(&buffers.call_return_stack)).unwrap();
                ocl::core::set_kernel_arg(&value, 11, ArgVal::mem(&buffers.branch_value_stack_state)).unwrap();
                ocl::core::set_kernel_arg(&value, 12, ArgVal::mem(&buffers.loop_value_stack_state)).unwrap();
                ocl::core::set_kernel_arg(&value, 13, ArgVal::mem(&buffers.hypercall_num)).unwrap();
                ocl::core::set_kernel_arg(&value, 14, ArgVal::mem(&buffers.hypercall_continuation)).unwrap();
                ocl::core::set_kernel_arg(&value, 15, ArgVal::mem(&buffers.current_mem)).unwrap();
                ocl::core::set_kernel_arg(&value, 16, ArgVal::mem(&buffers.max_mem)).unwrap();
                ocl::core::set_kernel_arg(&value, 17, ArgVal::mem(&buffers.is_calling)).unwrap();
                ocl::core::set_kernel_arg(&value, 18, ArgVal::mem(&buffers.entry)).unwrap();
                ocl::core::set_kernel_arg(&value, 19, ArgVal::mem(&hcall_retval_buffer)).unwrap();
            }
        }

        dbg!(self.entry_point);
        let mut start_kernel = kernels.get(&self.entry_point).unwrap();

        // now the data in the program has been initialized, we can run the main loop
        println!("start: {}", Utc::now().timestamp());
        loop {
            // run the kernel!
            // warning - bugged kernels can cause GPU driver hangs! Will result in the driver restarting...
            // Hangs are frequently a sign of a segmentation faults from inside of the GPU kernel
            // Unfortunately the OpenCL API doesn't give us a good way to identify what happened - the OS logs (dmesg) do have a record of this though
            profiling_event = ocl::Event::empty();
            unsafe {
                ocl::core::enqueue_kernel(&queue, &start_kernel, 1, None, &[self.num_vms as usize, 1, 1], None, None::<Event>, Some(&mut profiling_event)).unwrap();
                
                /*
                match ocl::core::finish(&queue) {
                    Err(e) => {
                        panic!("Unable to finish waiting on queue for start_kernel\n\n{}", e);
                    },
                    Ok(_) => (),
                }
                */
            }

            ocl::core::wait_for_event(&profiling_event).unwrap();
            let queue_start_kernel = profiling_event.profiling_info(ocl::enums::ProfilingInfo::Queued).unwrap().time().unwrap();
            let start_start_kernel = profiling_event.profiling_info(ocl::enums::ProfilingInfo::Start).unwrap().time().unwrap();
            let end_start_kernel = profiling_event.profiling_info(ocl::enums::ProfilingInfo::End).unwrap().time().unwrap();
            total_gpu_execution_time += end_start_kernel-queue_start_kernel;
            queue_submit_delta += start_start_kernel-queue_start_kernel;
            // upon exiting we check the stack pointer for each VM
            let vmm_pre_overhead = std::time::Instant::now();

            unsafe {
                ocl::core::enqueue_read_buffer(&queue, &buffers.sp, true, 0, &mut stack_pointer_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_read_buffer(&queue, &buffers.entry, true, 0, &mut entry_point_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_read_buffer(&queue, &buffers.hypercall_num, true, 0, &mut hypercall_num_temp, None::<Event>, None::<&mut Event>).unwrap();
            }

            // if all entry_point == -1, also exit
            entry_point_exit_flag = true;
            for e in &entry_point_temp {
                entry_point_exit_flag = (*e as i32 == (-1)) & entry_point_exit_flag;
            }

            if entry_point_exit_flag {
                break;
            }

            let vmm_pre_overhead_end = std::time::Instant::now();
            vmm_overhead += (vmm_pre_overhead_end - vmm_pre_overhead).as_nanos();

            /*
             * When we reach this point divergence may have occured!
             * 
             * Functions may be either:
             * 1) Waiting for hypercall dispatch
             * 2) Invoking another function
             * 
             * In order to resolve this, first we do a linear scan through the VMs to see if
             * anyone is calling a function and not performing a hypercall.
             * If so, we block off all functions that are performing hcalls, and run the first func that we find.
             * 
             * If everyone is either:
             * 1) Done executing
             * 2) Blocked on a hcall
             * 
             * Then we proceed to the rest of this function
             * 
             */

            let mut found = false;
            let mut hcall_idx = 0;
            // for each VM, add to the set of kernels that we need to run next
            for idx in 0..self.num_vms {
                // if we find a VM that isn't blocked on a hypercall
                if !found && hypercall_num_temp[idx as usize] == -2 {
                    // set the next function to run to be this targeted function
                    start_kernel = kernels.get(&entry_point_temp[idx as usize]).unwrap();
                    found = true;
                    break;
                } else {
                    //println!("blocked on hcall!");
                    // VMs that are set to run a hypercall need to be blocked off so they do not run
                    // BUT, we need to save all of the kernels
                }
            }

            // if we found a VM that needs to run another function, we do that first
            if found {
                //dbg!("found VM calling func");
                continue;
            } else {
                /*
                dbg!(stack_pointer_temp.clone());
                dbg!(entry_point_temp.clone());
                dbg!(hypercall_num_temp.clone());
                dbg!(start_kernel);
                //panic!("test");

                dbg!(hcall_idx as usize);
                */
                // if we don't have any VMs to run, reset the next function to run to be that of the hcall
                // we are returning to and dispatch the calls
                start_kernel = kernels.get(&entry_point_temp[hcall_idx as usize]).unwrap();
            }

            // read the hypercall_buffer
            let start_hcall_dispatch = std::time::Instant::now();
            unsafe {
                let buf: &mut [u8] = &mut hcall_read_buffer.lock().unwrap();
                ocl::core::enqueue_read_buffer(&queue, &hypercall_buffer, true, 0, buf, None::<Event>, None::<&mut Event>).unwrap();
            }

            // now it is time to dispatch hypercalls
            vm_slice.as_slice().par_iter().for_each(|vm_id| {
                let hypercall_id = match hypercall_num_temp[*vm_id as usize] as i64 {
                    0 => WasiSyscalls::FdWrite,
                    1 => WasiSyscalls::ProcExit,
                    2 => WasiSyscalls::EnvironSizeGet,
                    3 => WasiSyscalls::EnvironGet,
                    4 => WasiSyscalls::FdPrestatGet,
                    5 => WasiSyscalls::FdPrestatDirName,
                    _ => WasiSyscalls::InvalidHyperCallNum,
                };
                hypercall_sender[(vm_id % num_threads) as usize].send(
                    HyperCall::new((*vm_id as u32).clone(),
                                   number_vms,
                                   stack_pointer_temp[*vm_id as usize],
                                   hypercall_id,
                                   self.is_memory_interleaved.clone(),
                                   &buffers,
                                   hcall_read_buffer.clone(),
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
                // after all of the hypercalls are finished, we should update all of the stack pointers
                hypercall_num_temp[result.get_vm_id() as usize] = -1;
                hypercall_retval_temp[result.get_vm_id() as usize] = result.get_result();
            }

            let end_hcall_dispatch = std::time::Instant::now();
            hcall_execution_time += (end_hcall_dispatch - start_hcall_dispatch).as_nanos();

            let vmm_post_overhead = std::time::Instant::now();

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
            // also don't forget to write the hcall buf back
            unsafe {
                let mut hcall_buf = hcall_read_buffer.lock().unwrap();
                ocl::core::enqueue_write_buffer(&queue, &hypercall_buffer, true, 0, &mut hcall_buf, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_write_buffer(&queue, &buffers.entry, true, 0, &mut entry_point_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_write_buffer(&queue, &buffers.sp, true, 0, &mut stack_pointer_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_write_buffer(&queue, &buffers.hypercall_num, true, 0, &mut hypercall_num_temp, None::<Event>, None::<&mut Event>).unwrap();
                ocl::core::enqueue_write_buffer(&queue, &hcall_retval_buffer, true, 0, &mut hypercall_retval_temp, None::<Event>, None::<&mut Event>).unwrap();
            }
            let vmm_post_overhead_end = std::time::Instant::now();
            vmm_overhead += (vmm_post_overhead_end - vmm_post_overhead).as_nanos();
        }

        let e2e_time_end = std::time::Instant::now();

        // To get final results back from the stack if we want for debugging stuff
        // only uncomment this out if you need to debug stuff, it will panic if you have too many VMs and too small of a buffer
        dbg!(print_return);
        if print_return {
            let mut check_results_debug = vec![0u8; (self.num_vms * 1024) as usize];
            unsafe {
                ocl::core::enqueue_read_buffer(&queue, &buffers.stack_buffer, true, 0, &mut check_results_debug, None::<Event>, None::<&mut Event>).unwrap();
            }
            for vm_idx in 0..self.num_vms {
                if self.is_memory_interleaved {
                    let result_i32 = Interleave::read_u32(&mut check_results_debug, 32, self.num_vms, vm_idx);
                    let result_i64 = Interleave::read_u64(&mut check_results_debug, 32, self.num_vms, vm_idx);
                    dbg!(result_i32 as i32);
                    dbg!(result_i64 as i64);
                    dbg!(result_i64 as u64);
                    dbg!(result_i32 as f32);
                    dbg!(result_i64 as f64);
                } else {
                    let result = LittleEndian::read_u32(&check_results_debug[vm_idx as usize..(vm_idx+4) as usize]);
                    dbg!(result as i32);
                }
            }
        }

        println!("end: {}", Utc::now().timestamp());
        println!("E2E execution time in nanoseconds: {}", (e2e_time_end - e2e_time_start).as_nanos());
        println!("On device time in nanoseconds: {}", total_gpu_execution_time);
        println!("Device Start-Queue overhead in nanoseconds: {}", queue_submit_delta);
        println!("Device Overhead / Device Execution Time: {}", queue_submit_delta as f64 / total_gpu_execution_time as f64);
        println!("fraction of time on device: {}", total_gpu_execution_time as f64 / (e2e_time_end - e2e_time_start).as_nanos() as f64);
        println!("fraction of time on hcall dispatch: {}", hcall_execution_time as f64 / (e2e_time_end - e2e_time_start).as_nanos() as f64);
        println!("fraction of time on VMM overhead: {}", vmm_overhead as f64 / (e2e_time_end - e2e_time_start).as_nanos() as f64);

        return VMMRuntimeStatus::StatusOkay;
    }
}
