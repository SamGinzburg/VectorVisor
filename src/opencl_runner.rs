mod environment;
mod interleave_offsets;
mod random;
mod serverless;
pub mod vectorized_vm;
mod wasi_fd;
mod wasi_time;

use vectorized_vm::HyperCall;
use vectorized_vm::HyperCallResult;
use vectorized_vm::VectorizedVM;
use vectorized_vm::WasiSyscalls;
use vectorized_vm::{VmRecvType, VmSenderType};
use wasi_fd::WasiFd;

use interleave_offsets::Interleave;

use ocl::core::ArgVal;
use ocl::core::Event;
use std::convert::TryFrom;
use std::ffi::CString;

use crossbeam::channel::bounded;
use crossbeam::channel::unbounded;
use std::collections::HashMap;

use ocl::core::types::abs::MemMap;
use ocl::core::CommandQueue;
use ocl::core::CommandQueueProperties;
use ocl::core::MapFlags;
use rayon::prelude::*;

use std::sync::atomic::{AtomicU32, Ordering};

use bus::Bus;
use core::task::Poll;
use crossbeam::channel::Receiver as SyncReceiver;
use crossbeam::channel::Sender as SyncSender;
use tokio::sync::mpsc::{Receiver, Sender};

use std::cell::UnsafeCell;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::transmute;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;

use bincode;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::thread::JoinHandle;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

use chrono::Utc;

use indicatif::{ProgressBar, ProgressStyle};

pub enum VMMRuntimeStatus {
    StatusOkay,
    StatusUnknownError,
}

pub struct UnsafeCellWrapper<T: 'static> {
    pub buf: UnsafeCell<&'static mut [T]>,
}

impl<T> UnsafeCellWrapper<T> {
    pub fn new(t: &'static mut [T]) -> Self {
        Self {
            buf: UnsafeCell::new(t),
        }
    }
}

unsafe impl Sync for UnsafeCellWrapper<u8> {}
unsafe impl Send for UnsafeCellWrapper<u8> {}
unsafe impl Sync for UnsafeCellWrapper<u32> {}
unsafe impl Send for UnsafeCellWrapper<u32> {}
unsafe impl Sync for UnsafeCellWrapper<u64> {}
unsafe impl Send for UnsafeCellWrapper<u64> {}

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
    hypercall_num: ocl::core::Mem,
    hypercall_continuation: ocl::core::Mem,
    current_mem: ocl::core::Mem,
    max_mem: ocl::core::Mem,
    is_calling: ocl::core::Mem,
    hcall_size: ocl::core::Mem,
    entry: ocl::core::Mem,
    overhead_tracker: ocl::core::Mem,
}

impl OpenCLBuffers {
    pub fn new(
        stack_buffer: ocl::core::Mem,
        heap_buffer: ocl::core::Mem,
        stack_frames: ocl::core::Mem,
        globals_buffer: ocl::core::Mem,
        sp: ocl::core::Mem,
        sfp: ocl::core::Mem,
        call_stack: ocl::core::Mem,
        call_return_stack: ocl::core::Mem,
        hypercall_num: ocl::core::Mem,
        hypercall_continuation: ocl::core::Mem,
        current_mem: ocl::core::Mem,
        max_mem: ocl::core::Mem,
        is_calling: ocl::core::Mem,
        hcall_size: ocl::core::Mem,
        entry: ocl::core::Mem,
        overhead_tracker: ocl::core::Mem,
    ) -> OpenCLBuffers {
        OpenCLBuffers {
            stack_buffer: stack_buffer,
            heap_buffer: heap_buffer,
            stack_frames: stack_frames,
            globals_buffer: globals_buffer,
            sp: sp,
            sfp: sfp,
            call_stack: call_stack,
            call_return_stack: call_return_stack,
            hypercall_num: hypercall_num,
            hypercall_continuation: hypercall_continuation,
            current_mem: current_mem,
            max_mem: max_mem,
            is_calling: is_calling,
            hcall_size: hcall_size,
            entry: entry,
            overhead_tracker: overhead_tracker,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SeralizedProgram {
    pub program_data: Vec<u8>,
    pub kernel_partition_mappings: HashMap<u32, u32>,
    pub kernel_part_debug: HashMap<u32, Vec<String>>,
    pub entry_point: u32,
    pub num_compiled_funcs: u32,
    pub globals_buffer_size: u32,
    pub interleave: u32,
    pub data_segment: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct PartitionedSeralizedProgram {
    pub program_data: HashMap<u32, Vec<u8>>,
    pub partition_mapping: HashMap<u32, u32>,
    pub kernel_part_debug: HashMap<u32, Vec<String>>,
    pub entry_point: u32,
    pub num_compiled_funcs: u32,
    pub globals_buffer_size: u32,
    pub interleave: u32,
    pub data_segment: Vec<u8>,
}

#[derive(Clone)]
pub enum InputProgram {
    Binary(
        HashMap<u32, u32>,
        HashMap<u32, Vec<String>>,
        Vec<u8>,
        Vec<u8>,
    ),
    Text(
        HashMap<u32, u32>,
        HashMap<u32, Vec<String>>,
        String,
        String,
        Vec<u8>,
    ),
    Partitioned(
        HashMap<u32, String>,
        HashMap<u32, Vec<String>>,
        String,
        HashMap<u32, (u32, u32, u32, u32, u32, u32, u32)>,
        HashMap<u32, u32>,
        Vec<u8>,
    ),
    PartitionedBinary(
        HashMap<u32, Vec<u8>>,
        HashMap<u32, u32>,
        HashMap<u32, Vec<String>>,
        Vec<u8>,
    ),
}

#[derive(Clone)]
pub enum ProgramType {
    Standard(
        HashMap<u32, u32>,
        HashMap<u32, Vec<String>>,
        ocl::core::Program,
        Vec<u8>,
    ),
    Partitioned(
        HashMap<u32, ocl::core::Program>,
        HashMap<u32, u32>,
        HashMap<u32, Vec<String>>,
        Vec<u8>,
    ),
}

#[derive(Clone)]
pub struct OpenCLRunner {
    num_vms: u32,
    input_program: InputProgram,
    is_gpu_backend: bool,
    is_memory_interleaved: u32,
    entry_point: u32,
    buffers: Option<OpenCLBuffers>,
}

impl OpenCLRunner {
    pub fn new(
        num_vms: u32,
        mem_interleave: u32,
        running_on_gpu: bool,
        entry_point: u32,
        program: InputProgram,
    ) -> OpenCLRunner {
        OpenCLRunner {
            num_vms: num_vms,
            input_program: program,
            is_gpu_backend: running_on_gpu,
            is_memory_interleaved: mem_interleave,
            entry_point: entry_point,
            buffers: None,
        }
    }

    pub fn run(
        self,
        context: &'static ocl::core::Context,
        program: ProgramType,
        device_id: ocl::core::DeviceId,
        _input_filename: &str,
        hcall_size: usize,
        stack_size: u32,
        heap_size: u32,
        call_stack_size: u32,
        stack_frames_size: u32,
        sfp_size: u32,
        // needed for the size of the loop/branch data structures
        num_compiled_funcs: u32,
        globals_buffer_size: u32,
        local_work_group: usize,
        mexec: usize,
        req_timeout: u32,
        vm_sender: Arc<Vec<Mutex<Sender<VmSenderType>>>>,
        vm_recv: Arc<Vec<Mutex<Receiver<VmRecvType>>>>,
        _compile_flags: String,
        _link_flags: String,
        print_return: bool,
    ) -> JoinHandle<()> {
        let num_vms = self.num_vms.clone();

        // create the buffers
        let new_runner = self.create_buffers(
            stack_size,
            heap_size,
            call_stack_size,
            stack_frames_size,
            sfp_size,
            num_compiled_funcs,
            globals_buffer_size,
            mexec as u32,
            context,
        );

        let handler = std::thread::spawn(move || {
            // this function returns the channel that we will use to send it HTTP requests later

            // each vector VMM group gets its own command queue
            let properties = CommandQueueProperties::new().profiling();
            let command_queue =
                ocl::core::create_command_queue(context, &device_id, Some(properties)).unwrap();

            // We purposefully leak the runner into a static object to deal with the lifetimes of the
            // hypercall dispatch thread pools, we will clean up the new_runner object if needed
            // These values really do last for the entire program, so it is fine to make them static
            let final_runner = Box::leak(Box::new(new_runner));
            let leaked_command_queue: &'static CommandQueue = Box::leak(Box::new(command_queue));
            // We use two hcall buffers on the host-side, once for generic hcalls, the other for buffering inputs to decrease function latency
            let hypercall_buffer_read_buffer: &'static mut [u8] =
                Box::leak(vec![0u8; hcall_size * num_vms as usize].into_boxed_slice());
            let hypercall_input_buffer: &'static mut [u8] =
                Box::leak(vec![0u8; hcall_size * num_vms as usize].into_boxed_slice());

            // decide which vector runner to use based off the compiled program enum...
            let status = match program {
                ProgramType::Standard(
                    original_mapping,
                    kernel_part_debug,
                    program,
                    data_segment,
                ) => {
                    let mut program_map: HashMap<u32, ocl::core::Program> = HashMap::new();
                    program_map.insert(0, program.clone());
                    program_map.insert(99999, program);

                    let mut kernel_partition_mapping: HashMap<u32, u32> = HashMap::new();
                    kernel_partition_mapping.insert(0, 0);

                    for (key, val) in original_mapping.iter() {
                        kernel_partition_mapping.insert(*key, 0);
                    }

                    final_runner.run_partitioned_vector_vms(
                        stack_frames_size,
                        local_work_group,
                        mexec,
                        program_map,
                        kernel_partition_mapping,
                        kernel_part_debug,
                        &leaked_command_queue,
                        hypercall_buffer_read_buffer,
                        hypercall_input_buffer,
                        hcall_size.try_into().unwrap(),
                        &context,
                        print_return,
                        vm_sender,
                        vm_recv,
                        req_timeout,
                        data_segment,
                    )
                }
                ProgramType::Partitioned(
                    program_map,
                    kernel_partition_mapping,
                    kernel_part_debug,
                    data_segment,
                ) => final_runner.run_partitioned_vector_vms(
                    stack_frames_size,
                    local_work_group,
                    mexec,
                    program_map,
                    kernel_partition_mapping,
                    kernel_part_debug,
                    &leaked_command_queue,
                    hypercall_buffer_read_buffer,
                    hypercall_input_buffer,
                    hcall_size.try_into().unwrap(),
                    &context,
                    print_return,
                    vm_sender,
                    vm_recv,
                    req_timeout,
                    data_segment,
                ),
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
    pub fn create_buffers(
        mut self,
        stack_size: u32,
        heap_size: u32,
        stack_frame_size: u32,
        stack_frame_ptr_size: u32,
        call_stack_size: u32,
        // needed for loop/branch structures
        _num_compiled_funcs: u32,
        global_buffers_size: u32,
        mexec: u32,
        context: &ocl::core::Context,
    ) -> OpenCLRunner {
        let mut size_tracker: u64 = 0;

        let stack_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE, //| ocl::core::MEM_ALLOC_HOST_PTR,
                (stack_size as u64 * self.num_vms as u64) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (stack_size as u64 * self.num_vms as u64) as u64;

        let heap_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE, //| ocl::core::MEM_ALLOC_HOST_PTR,
                (heap_size as u64 * self.num_vms as u64) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (heap_size as u64 * self.num_vms as u64) as u64;

        let globals_buffer = unsafe {
            if global_buffers_size > 0 {
                size_tracker += (global_buffers_size * 8 * self.num_vms * 2) as u64;
                ocl::core::create_buffer::<_, u8>(
                    context,
                    ocl::core::MEM_READ_WRITE,
                    // global_buffers_size is in increments of 8 bytes
                    (global_buffers_size * 8 * self.num_vms * 2) as usize,
                    None,
                )
                .unwrap()
            } else {
                size_tracker += (1) as u64;
                // just to get by, create a buffer of size 1 that we will never use
                ocl::core::create_buffer::<_, u8>(
                    context,
                    ocl::core::MEM_READ_WRITE,
                    // global_buffers_size is in increments of 8 bytes
                    1,
                    None,
                )
                .unwrap()
            }
        };

        /*
         * TODO: find proper sizes for the non-heap/stack buffers
         */
        let stack_frames = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE,
                (stack_frame_size as u64 * 4 * self.num_vms as u64) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (stack_frame_size as u64 * 4 * self.num_vms as u64) as u64;

        // TODO: sp is currently 8 bytes? very unecessary - 4 bytes is probably enough
        let sp = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                (8 * self.num_vms * mexec) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (8 * self.num_vms * mexec) as u64;

        // way, way too big
        let sfp = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                (stack_frame_ptr_size * 8 * self.num_vms * mexec) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (stack_frame_ptr_size * 8 * self.num_vms * mexec) as u64;

        // 1KB call stack should be way more than enough
        let call_stack = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                (call_stack_size * 8 * self.num_vms) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (call_stack_size * 8 * self.num_vms) as u64;

        let call_return_stack = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE,
                (call_stack_size * 8 * self.num_vms) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (call_stack_size * 8 * self.num_vms) as u64;

        let hypercall_num = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                (4 * self.num_vms * mexec) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (4 * self.num_vms * mexec) as u64;

        let hypercall_continuation = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                (4 * self.num_vms * mexec) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (4 * self.num_vms * mexec) as u64;

        let current_mem = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE,
                (4 * self.num_vms * mexec) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (4 * self.num_vms * mexec) as u64;

        let max_mem = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE,
                (4 * self.num_vms * mexec) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (4 * self.num_vms * mexec) as u64;

        let entry = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                (4 * self.num_vms * mexec) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (4 * self.num_vms * mexec) as u64;

        let is_calling = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE,
                (self.num_vms * mexec) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (self.num_vms * mexec) as u64;

        let hcall_size = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE,
                (self.num_vms * 4) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (self.num_vms * 4) as u64;

        let overhead_tracker = unsafe {
            ocl::core::create_buffer::<_, u8>(
                context,
                ocl::core::MEM_READ_WRITE,
                (self.num_vms * 8) as usize,
                None,
            )
            .unwrap()
        };
        size_tracker += (self.num_vms * 8) as u64;

        println!(
            "Allocated: {:.2} MB in OpenCL Buffers",
            size_tracker as f64 / 1024.0 / 1024.0
        );

        self.buffers = Some(OpenCLBuffers::new(
            stack_buffer,
            heap_buffer,
            stack_frames,
            globals_buffer,
            sp,
            sfp,
            call_stack,
            call_return_stack,
            hypercall_num,
            hypercall_continuation,
            current_mem,
            max_mem,
            is_calling,
            hcall_size,
            entry,
            overhead_tracker,
        ));
        self
    }

    /*
     * This function starts up a new thread to start running vectorized VMs
     *
     * It returns a sending channel for the HTTP Endpoint to send requests to be processed with.
     *
     */
    pub fn setup_kernel(
        &self,
        context: &'static ocl::core::Context,
        device_id: ocl::core::DeviceId,
        input_filename: &str,
        stack_size: u32,
        heap_size: u32,
        num_compiled_funcs: u32,
        globals_buffer_size: u32,
        compile_flags: String,
        link_flags: String,
        jitcache: bool,
    ) -> (ProgramType, ocl::core::DeviceId, u128) {
        let dev_type = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::Type);
        let dev_name = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::Name);
        let vendor = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::Vendor);
        let ocl_version = ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::Version);
        let ocl_c_version =
            ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::OpenclCVersion);
        let compute_units =
            ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::MaxComputeUnits);
        let max_param_size =
            ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::MaxParameterSize);
        let max_global_mem_size =
            ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::GlobalMemSize);
        let max_constant_buffer_size =
            ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::MaxConstantBufferSize);
        let linker_available =
            ocl::core::get_device_info(&device_id, ocl::core::DeviceInfo::LinkerAvailable);
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
        println!("Compile Flags: {:?}", compile_flags);
        println!("Link Flags: {:?}", link_flags);

        let compile_time_start = std::time::Instant::now();

        // compile the GPU kernel(s)
        let program_to_run = match &self.input_program {
            InputProgram::Text(
                kernel_partition_mappings,
                kernel_part_debug,
                program,
                fastcall_header,
                data_segment,
            ) => {
                // create the build log
                File::create(format!("recent.buildlog")).unwrap();

                println!(
                    "Sucessfully compiled kernel to OpenCL C: saving to: {}",
                    format!("{}.cl", input_filename)
                );
                let mut file = File::create(format!("{}.cl", input_filename)).unwrap();
                file.write_all(&program.clone().into_bytes()).unwrap();
                println!("Starting kernel compilation...");

                let formatted_program =
                    str::replace(&program, "#include \"fastcalls.cl\"", &fastcall_header);
                let final_formatted_program = CString::new(formatted_program).unwrap();

                let compile_start = std::time::Instant::now();

                let final_program = ocl::core::create_program_with_source(
                    context,
                    &[final_formatted_program.clone()],
                )
                .unwrap();

                let build_start = std::time::Instant::now();
                let options = &CString::new(format!(
                    "{} -DNUM_THREADS={} -DVMM_STACK_SIZE_BYTES={} -DVMM_HEAP_SIZE_BYTES={}",
                    compile_flags, self.num_vms, stack_size, heap_size
                ))
                .unwrap();
                ocl::core::build_program(&final_program, Some(&[device_id]), options, None, None)
                    .unwrap();
                let build_end = std::time::Instant::now();
                println!("Build time for kernel: {:?}", build_end - build_start);

                let buildinfo = ocl::core::get_program_build_info(
                    &final_program,
                    &device_id,
                    ocl::core::ProgramBuildInfo::BuildLog,
                )
                .unwrap();
                let mut build_log = OpenOptions::new()
                    .append(true)
                    .open("recent.buildlog")
                    .unwrap();

                build_log
                    .write_all(&format!("\nbuildlog for full program:\n").into_bytes())
                    .unwrap();
                build_log
                    .write_all(&buildinfo.to_string().into_bytes())
                    .unwrap();

                let buildinfo = ocl::core::get_program_build_info(
                    &final_program,
                    &device_id,
                    ocl::core::ProgramBuildInfo::BuildLog,
                )
                .unwrap();
                dbg!(buildinfo);
                let compile_end = std::time::Instant::now();

                // if we are going to save the program, we save the binary here
                let buildinfo2 = ocl::core::get_program_build_info(
                    &final_program,
                    &device_id,
                    ocl::core::ProgramBuildInfo::BuildLog,
                )
                .unwrap();
                dbg!(buildinfo2);

                let saved_binary =
                    ocl::core::get_program_info(&final_program, ocl::core::ProgramInfo::Binaries);
                let binary = match saved_binary.unwrap() {
                    ocl::core::types::enums::ProgramInfoResult::Binaries(binary_vec) => {
                        binary_vec.get(0).unwrap().clone()
                    }
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
                    kernel_partition_mappings: kernel_partition_mappings.clone(),
                    kernel_part_debug: kernel_part_debug.clone(),
                    globals_buffer_size: globals_buffer_size,
                    entry_point: self.entry_point,
                    num_compiled_funcs: num_compiled_funcs,
                    interleave: self.is_memory_interleaved,
                    data_segment: data_segment.to_vec(),
                };

                let serialized_program = bincode::serialize(&program_to_serialize).unwrap();
                let mut file = File::create(format!("{}.bin", input_filename)).unwrap();
                file.write_all(&serialized_program).unwrap();

                // while we are at it, might as well save the input .cl file as well
                let mut file = File::create(format!("{}.cl", input_filename)).unwrap();
                file.write_all(&program.clone().into_bytes()).unwrap();

                ProgramType::Standard(
                    kernel_partition_mappings.clone(),
                    kernel_part_debug.clone(),
                    final_program,
                    data_segment.to_vec(),
                )
            }
            InputProgram::Binary(kernel_partition_mappings, kernel_part_debug, b, data_segment) => {
                let binary_start = std::time::Instant::now();

                let program_to_run =
                    match ocl::core::create_program_with_binary(context, &[device_id], &[&b]) {
                        Ok(binary) => binary,
                        Err(e) => panic!("Unable to create program from given binary: {:?}", e),
                    };
                ocl::core::build_program(
                    &program_to_run,
                    Some(&[device_id]),
                    &CString::new(format!(
                        "{} -DNUM_THREADS={} -DVMM_STACK_SIZE_BYTES={} -DVMM_HEAP_SIZE_BYTES={}",
                        compile_flags, self.num_vms, stack_size, heap_size
                    ))
                    .unwrap(),
                    None,
                    None,
                )
                .unwrap();
                let knames = ocl::core::get_program_info(
                    &program_to_run,
                    ocl::core::ProgramInfo::KernelNames,
                );
                println!("Loaded kernels: {}", knames.unwrap());
                let binary_prep_end = std::time::Instant::now();
                println!(
                    "Time to load program from binary: {:?}",
                    binary_prep_end - binary_start
                );
                ProgramType::Standard(
                    kernel_partition_mappings.clone(),
                    kernel_part_debug.clone(),
                    program_to_run,
                    data_segment.to_vec(),
                )
            }
            InputProgram::Partitioned(
                map,
                kernel_part_debug,
                fastcall_header,
                compile_stats_map,
                kernel_partition_mappings,
                data_segment,
            ) => {
                let mut final_hashmap: HashMap<u32, ocl::core::Program> = HashMap::new();
                let mut final_binarized_hashmap: HashMap<u32, Vec<u8>> = HashMap::new();

                let kernel_compile = std::time::Instant::now();

                // Spin up N threads (n = ncpus)
                // Use fewer than N threads to minimize memory consumption during compilation
                let num_threads = 2; //num_cpus::get();
                let _num_vms = self.num_vms.clone();

                // create the build log
                File::create(format!("recent.buildlog")).unwrap();

                let (finished_sender, finished_receiver): (
                    SyncSender<(u32, ocl::core::Program, u64)>,
                    SyncReceiver<(u32, ocl::core::Program, u64)>,
                ) = unbounded();

                let mut submit_compile_job = vec![];

                for _idx in 0..num_threads {
                    let (compile_sender, compile_receiver): (
                        SyncSender<(u32, String, String)>,
                        SyncReceiver<(u32, String, String)>,
                    ) = unbounded();
                    let _cflags = compile_flags.clone();
                    let sender = finished_sender.clone();
                    let num_vms_clone = self.num_vms.clone();
                    let link_flags_clone = link_flags.clone();
                    let compile_flags_clone = compile_flags.clone();
                    let _context_clone = context.clone();

                    submit_compile_job.push(compile_sender);

                    thread::spawn(move || {
                        let receiver = compile_receiver.clone();
                        loop {
                            // receive the function to compile
                            let (key, fastcall_header, program_as_string) = match receiver.recv() {
                                Ok(m) => m,
                                _ => {
                                    // if the main sending thread is closed, we will get an error
                                    // we are handling that error elsewhere, so we can just exit the thread in that case
                                    break;
                                }
                            };
                            let start = Utc::now().timestamp_nanos();
                            let options = &CString::new(format!("{} -DNUM_THREADS={} -DVMM_STACK_SIZE_BYTES={} -DVMM_HEAP_SIZE_BYTES={}", compile_flags_clone, num_vms_clone, stack_size, heap_size)).unwrap();
                            // hack to deal with header file (compile/link program don't emit the debug info we want on nvidia)
                            let formatted_program = str::replace(
                                &program_as_string,
                                "#include \"fastcalls.cl\"",
                                &fastcall_header,
                            );
                            let final_formatted_program = CString::new(formatted_program).unwrap();
                            let program_to_build = match ocl::core::create_program_with_source(
                                context,
                                &[final_formatted_program],
                            ) {
                                Ok(binary) => binary,
                                Err(e) => {
                                    panic!("Unable to create program from given text: {:?}", e)
                                }
                            };

                            ocl::core::build_program(
                                &program_to_build,
                                Some(&[device_id]),
                                options,
                                None,
                                None,
                            )
                            .unwrap();

                            let buildinfo = ocl::core::get_program_build_info(
                                &program_to_build,
                                &device_id,
                                ocl::core::ProgramBuildInfo::BuildLog,
                            )
                            .unwrap();
                            let mut build_log = OpenOptions::new()
                                .append(true)
                                .open("recent.buildlog")
                                .unwrap();

                            build_log
                                .write_all(
                                    &format!("\nbuildlog for partition idx: {:?}\n", key)
                                        .into_bytes(),
                                )
                                .unwrap();
                            build_log
                                .write_all(&buildinfo.to_string().into_bytes())
                                .unwrap();

                            let end = Utc::now().timestamp_nanos();

                            sender
                                .send((key, program_to_build, (end - start).try_into().unwrap()))
                                .unwrap();
                        }
                    });
                }

                // for each function, submit it to be compiled
                let mut counter = 0;
                for (key, value) in map.iter() {
                    let src_cstring = CString::new(value.clone()).unwrap();
                    //let header_cstring = CString::new(fastcall_header.clone()).unwrap();
                    //let compiled_program = ocl::core::create_program_with_source(context, &[src_cstring.clone()]).unwrap();
                    //let fastcall_header = ocl::core::create_program_with_source(context, &[header_cstring.clone()]).unwrap();

                    submit_compile_job[counter % submit_compile_job.len() as usize]
                        .send((*key, fastcall_header.clone(), String::from(value)))
                        .unwrap();
                    counter += 1;
                }
                let pb = ProgressBar::new(map.len().try_into().unwrap());
                pb.enable_steady_tick(1000);

                pb.set_style(ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{wide_bar.cyan/blue}] (Compiled functions: {pos}) / (Total functions in program {len}) ({eta})")
                    .progress_chars("#>-"));

                for idx in 0..map.len() {
                    let (key, compiled_program, time_to_compile) =
                        finished_receiver.recv().unwrap();

                    // extract the binary and save that as well
                    let binary_to_save = ocl::core::get_program_info(
                        &compiled_program,
                        ocl::core::ProgramInfo::Binaries,
                    );
                    let binary = match binary_to_save.unwrap() {
                        ocl::core::types::enums::ProgramInfoResult::Binaries(binary_vec) => {
                            binary_vec.get(0).unwrap().clone()
                        }
                        _ => panic!("Incorrect result from get_program_info"),
                    };

                    // Don't record build times for data_init
                    if key != 99999 {
                        //dbg!(time_to_compile, compile_stats_map.get(&key).unwrap());
                        let (
                            total_instr_count,
                            total_func_count,
                            total_fastcall_count,
                            total_indirect_count,
                            total_block_count,
                            total_loop_count,
                            total_local_count,
                        ) = compile_stats_map.get(&key).unwrap();
                        let mut file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .create(true)
                            .open("compile-times-log.csv")
                            .unwrap();
                        if let Err(e) = writeln!(
                            file,
                            "{}",
                            format!(
                                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                                key,
                                total_instr_count,
                                total_func_count,
                                total_fastcall_count,
                                total_indirect_count,
                                total_block_count,
                                total_loop_count,
                                total_local_count,
                                time_to_compile
                            )
                        ) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                    }

                    pb.set_position(idx.try_into().unwrap());
                    final_binarized_hashmap.insert(key, binary);
                    final_hashmap.insert(key, compiled_program);
                }

                let kernel_compile_end = std::time::Instant::now();

                pb.finish_with_message(&format!(
                    "Finished, time to compile all functions: {:?}",
                    kernel_compile_end - kernel_compile
                ));

                let program_to_serialize = PartitionedSeralizedProgram {
                    program_data: final_binarized_hashmap,
                    partition_mapping: kernel_partition_mappings.clone(),
                    kernel_part_debug: kernel_part_debug.clone(),
                    globals_buffer_size: globals_buffer_size,
                    entry_point: self.entry_point,
                    num_compiled_funcs: num_compiled_funcs,
                    interleave: self.is_memory_interleaved,
                    data_segment: data_segment.to_vec(),
                };

                let serialized_program = bincode::serialize(&program_to_serialize).unwrap();
                let mut file = File::create(format!("{}.partbin", input_filename)).unwrap();
                file.write_all(&serialized_program).unwrap();

                ProgramType::Partitioned(
                    final_hashmap,
                    kernel_partition_mappings.clone(),
                    kernel_part_debug.clone(),
                    data_segment.to_vec(),
                )
            }
            InputProgram::PartitionedBinary(
                map,
                kernel_partition_mappings,
                kernel_part_debug,
                data_segment,
            ) => {
                let mut final_hashmap: HashMap<u32, ocl::core::Program> = HashMap::new();
                // create the build log
                File::create(format!("recent.buildlog")).unwrap();

                // map contains a mapping of u32 (function ID) -> program
                let binary_start = std::time::Instant::now();

                let pb = ProgressBar::new(map.len().try_into().unwrap());
                pb.enable_steady_tick(1000);
                pb.set_style(ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{wide_bar.cyan/blue}] (Loaded functions: {pos}) / (Total functions in program {len}) ({eta})")
                    .progress_chars("#>-"));

                let mut count: u64 = 0;
                for (id, program_binary) in map.iter() {
                    let program_to_run = match ocl::core::create_program_with_binary(
                        context,
                        &[device_id],
                        &[&program_binary],
                    ) {
                        Ok(binary) => binary,
                        Err(e) => panic!("Unable to create program from given binary: {:?}", e),
                    };
                    ocl::core::build_program(&program_to_run, Some(&[device_id]), &CString::new(format!("{} -DNUM_THREADS={} -DVMM_STACK_SIZE_BYTES={} -DVMM_HEAP_SIZE_BYTES={}", compile_flags, self.num_vms, stack_size, heap_size)).unwrap(), None, None).unwrap();
                    let buildinfo = ocl::core::get_program_build_info(
                        &program_to_run,
                        &device_id,
                        ocl::core::ProgramBuildInfo::BuildLog,
                    )
                    .unwrap();
                    let mut build_log = OpenOptions::new()
                        .append(true)
                        .open("recent.buildlog")
                        .unwrap();

                    build_log
                        .write_all(
                            &format!("Compiler output for partition ID: {}\n\n", *id).into_bytes(),
                        )
                        .unwrap();
                    build_log
                        .write_all(&format!("{}\n", buildinfo.to_string()).into_bytes())
                        .unwrap();

                    pb.set_position(count);
                    count += 1;
                    final_hashmap.insert(*id, program_to_run);
                }

                let binary_prep_end = std::time::Instant::now();
                pb.finish_with_message(&format!(
                    "Time to load program from binary: {:?}",
                    binary_prep_end - binary_start
                ));

                ProgramType::Partitioned(
                    final_hashmap,
                    kernel_partition_mappings.clone(),
                    kernel_part_debug.clone(),
                    data_segment.to_vec(),
                )
            }
        };

        // If we are only generate the JIT cache, exit now

        let compile_time_end = std::time::Instant::now();
        if jitcache {
            process::exit(0);
        }

        return (
            program_to_run,
            device_id,
            (compile_time_end - compile_time_start).as_nanos(),
        );
    }

    /*
     * This function runs the partitioned vector VMs
     */
    pub fn run_partitioned_vector_vms(
        self: &'static OpenCLRunner,
        per_vm_stack_frames_size: u32,
        local_work_group: usize,
        mexec: usize,
        program_map: HashMap<u32, ocl::core::Program>,
        kernel_partition_mappings: HashMap<u32, u32>,
        kernel_part_debug: HashMap<u32, Vec<String>>,
        queue: &'static CommandQueue,
        hypercall_buffer_read_buffer: &'static mut [u8],
        hypercall_input_buffer: &'static mut [u8],
        hypercall_buffer_size: u32,
        ctx: &ocl::core::Context,
        print_return: bool,
        vm_sender: Arc<Vec<Mutex<Sender<VmSenderType>>>>,
        vm_recv: Arc<Vec<Mutex<Receiver<VmRecvType>>>>,
        req_timeout: u32,
        data_segment: Vec<u8>,
    ) -> VMMRuntimeStatus {
        let mut kernels: HashMap<u32, ocl::core::Kernel> = HashMap::new();

        // setup the data kernel
        let data_program = program_map.get(&99999).unwrap();
        kernels.insert(
            99999,
            ocl::core::create_kernel(&data_program, "data_init").unwrap(),
        );

        // create the map of runnable kernels
        for (key, value) in program_map {
            if key != 99999 {
                kernels.insert(key, ocl::core::create_kernel(&value, "wasm_entry").unwrap());
            }
        }

        let mut stack_pointer_temp: &mut [u64] = &mut vec![0u64; self.num_vms as usize * mexec];
        let mut overhead_tracker: &'static mut [u64] =
            Box::leak(vec![0u64; self.num_vms as usize].into_boxed_slice());
        let mut entry_point_temp = vec![0u32; self.num_vms as usize * mexec];
        let mut hypercall_num_temp = vec![0i32; self.num_vms as usize * mexec];
        let mut hypercall_retval_temp = vec![0i32; self.num_vms as usize];
        let mut entry_point_exit_flag;
        let vm_slice: Vec<u32> = std::ops::Range {
            start: 0,
            end: (self.num_vms),
        }
        .collect();
        //let mut hypercall_sender = vec![];
        let overhead_tracker_buffer: Arc<UnsafeCellWrapper<u64>> =
            Arc::new(UnsafeCellWrapper::new(overhead_tracker));
        let hcall_read_buffer: Arc<UnsafeCellWrapper<u8>> =
            Arc::new(UnsafeCellWrapper::new(hypercall_buffer_read_buffer));
        let hcall_async_buffer: Arc<Mutex<UnsafeCellWrapper<u8>>> =
            Arc::new(Mutex::new(UnsafeCellWrapper::new(hypercall_input_buffer)));

        let mut total_gpu_execution_time: u64 = 0;
        let mut queue_submit_delta: u64 = 0;

        let mut hcall_execution_time: u128 = 0;
        let mut vmm_overhead: u128 = 0;

        /*
         * Allocate the hypercall_buffer at the last minute, 16KiB per VM
         *
         */
        let hypercall_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(
                ctx,
                ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                (hypercall_buffer_size * self.num_vms) as usize,
                None,
            )
            .unwrap()
        };

        /*
         * Allocate the buffer to return values
         */
        let hcall_retval_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(
                ctx,
                ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                (4 * self.num_vms) as usize,
                None,
            )
            .unwrap()
        };

        /*
         * Allocate the data_segment buffer
         */
        let data_seg_buffer_len = if data_segment.len() == 0 {
            1
        } else {
            data_segment.len() as usize
        };
        let data_segment_buffer = unsafe {
            ocl::core::create_buffer::<_, u8>(
                ctx,
                ocl::core::MEM_READ_WRITE | ocl::core::MEM_ALLOC_HOST_PTR,
                data_seg_buffer_len,
                None,
            )
            .unwrap()
        };
        let ds_result = unsafe {
            ocl::core::enqueue_write_buffer(
                &queue,
                &data_segment_buffer,
                true,
                0,
                &data_segment,
                None::<Event>,
                None::<&mut Event>,
            )
        };

        match ds_result {
            Err(e) => panic!("Failed to write data_segment to GPU memory, Error: {}", e),
            _ => (),
        }

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

        let num_threads = if num_cpus::get() as u32 > self.num_vms {
            self.num_vms as u32
        } else {
            num_cpus::get() as u32
        };

        //let num_threads = 1;
        //let num_threads = num_cpus::get() as u32;
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads.try_into().unwrap())
            .stack_size(1024 * 256)
            .build()
            .unwrap();

        let number_vms = self.num_vms.clone();
        let (result_sender, result_receiver): (
            SyncSender<HyperCallResult>,
            SyncReceiver<HyperCallResult>,
        ) = unbounded();

        let mut hcall_sender_vec = vec![];
        let mut hcall_recv_vec = vec![];
        for idx in 0..number_vms {
            let (hcall_sender, hcall_recv): (
                SyncSender<(Box<HyperCall>, bool)>,
                SyncReceiver<(Box<HyperCall>, bool)>,
            ) = unbounded();
            hcall_sender_vec.push(hcall_sender);
            hcall_recv_vec.push(hcall_recv);
        }
        //let hcall_queue: Arc<ArrayQueue<Box<HyperCall>>> = Arc::new(ArrayQueue::new((number_vms).try_into().unwrap()));
        let vm_idx_count = Arc::new(AtomicU32::new(0));
        let mut invoke_complete = Bus::new(1);
        let mut invoke_rx = vec![];
        for idx in 0..num_threads {
            invoke_rx.push(Mutex::new(invoke_complete.add_rx()));
        }
        let shareable_rx = Arc::new(invoke_rx);

        for thread_idx in 0..num_threads {
            //let (sender, recv): (SyncSender<Box<HyperCall>>, SyncReceiver<Box<HyperCall>>) = unbounded();
            let hcall_recv_clone = hcall_recv_vec.clone();
            let sender_copy = result_sender.clone();
            //hypercall_sender.push(sender.clone());
            //let hcall_queue_cloned = hcall_queue.clone();
            let vm_sender_copy = vm_sender.clone();
            let vm_recv_copy = vm_recv.clone();
            let vm_counter = vm_idx_count.clone();
            let async_buffer = hcall_async_buffer.clone();
            let invoke_blocker = shareable_rx.clone();

            thread_pool.spawn(move || {
                let receiver = hcall_recv_clone.clone();
                // copy the references to the WASI contexts for this thread
                let mut worker_vms: Vec<VectorizedVM> = vec![];
                let mut vm_id_vec = vec![];
                let mut vm_id_mapping: HashMap<u32, u32> = HashMap::new();
                for idx in 0..number_vms / num_threads {
                    let vm_index = idx + (thread_idx * (number_vms / num_threads));
                    //let vm_index = vm_counter.fetch_add(1, Ordering::Relaxed);
                    worker_vms.push(VectorizedVM::new(
                        (vm_index as u32).try_into().unwrap(),
                        hypercall_buffer_size,
                        number_vms,
                        vm_sender_copy.clone(),
                        vm_recv_copy.clone(),
                    ));
                    vm_id_vec.push(vm_index.clone());
                    vm_id_mapping.insert(vm_index.clone(), idx);
                }

                let waker = futures::task::noop_waker();
                let mut cx = std::task::Context::from_waker(&waker);
                let mut avail_vms = true;
                let mut avail_vm_count = number_vms / num_threads;
                let mut counter = 0;
                let mut recv_reqs = 0;
                let mut dispatchable_hcalls = vec![];
                let mut block_on_inputs = false;
                let poll_freq = 10;
                let buffer_timeout: u64 = req_timeout.into();
                let mut ellapsed_time = 0;
                let sleep_time = time::Duration::from_millis(poll_freq);
                let invoke_blocker_rx = invoke_blocker;
                let mut is_empty_vm: HashSet<usize> = HashSet::new();

                loop {
                    thread::sleep(sleep_time);
                    ellapsed_time += poll_freq;
                    // Check if we have an incoming function input to write to the hcall buffer
                    // Each thread polls a set of VMs to see if we can write to the hcall buf yet
                    // We will only poll if we know we have open slots in our group of VMs
                    // for each VM that we have, check if we have a request waiting
                    if recv_reqs < avail_vm_count {
                        for vm_idx in &vm_id_vec {
                            let worker_vm_idx = *vm_id_mapping.get(vm_idx).unwrap() as usize;
                            if !is_empty_vm.contains(&worker_vm_idx) {
                                match vm_recv_copy[*vm_idx as usize]
                                    .lock()
                                    .unwrap()
                                    .poll_recv(&mut cx)
                                {
                                    Poll::Ready(Some((msg, _, uuid))) => {
                                        let wasi_context = &mut worker_vms[worker_vm_idx];
                                        // Queue the input in the VM
                                        let buffer = async_buffer.clone();
                                        let deref_buf =
                                            unsafe { &mut *buffer.lock().unwrap().buf.get() };
                                        wasi_context.queue_request(msg, *deref_buf, uuid);
                                        recv_reqs += 1;
                                        is_empty_vm.insert(worker_vm_idx);
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }

                    // Check to see if we have a hypercall to dispatch for the VM
                    for vm_idx in &vm_id_vec {
                        match receiver[*vm_idx as usize].try_recv() {
                            Ok((m, is_serverless_invoke)) => {
                                dispatchable_hcalls.push(m);
                                block_on_inputs = is_serverless_invoke;
                                if block_on_inputs {
                                    ellapsed_time = 0;
                                }
                            }
                            _ => {
                                // if the main sending thread is closed, we will get an error
                                // we are handling that error elsewhere, so we can just exit the thread in that case
                                //break;
                            }
                        };
                    }

                    if (recv_reqs > 0 && recv_reqs >= avail_vm_count && block_on_inputs)
                        || (ellapsed_time >= buffer_timeout)
                    {
                        block_on_inputs = false;
                    }
                    // If serverless invoke, block until no vms available
                    // else, we just dispatch
                    if !block_on_inputs {
                        for mut incoming_call in &mut dispatchable_hcalls {
                            let worker_vm_idx =
                                *vm_id_mapping.get(&incoming_call.vm_id).unwrap() as usize;
                            let wasi_context = &mut worker_vms[worker_vm_idx];
                            wasi_context.dispatch_hypercall(&mut incoming_call, &sender_copy);
                        }
                        dispatchable_hcalls.clear();
                        let hcall_end = std::time::Instant::now();
                        //println!("hcall time: {:?}", (hcall_end - hcall_start).as_nanos());
                    }

                    // Now block until we have written the requests to the GPU
                    if !block_on_inputs {
                        match invoke_blocker_rx[thread_idx as usize]
                            .lock()
                            .unwrap()
                            .try_recv()
                        {
                            Ok(true) => {
                                recv_reqs = 0;
                                ellapsed_time = 0;
                                is_empty_vm.clear();
                            }
                            _ => (),
                        }
                    }
                }
            });
        }

        let buffers = match &self.buffers {
            Some(b) => b,
            _ => panic!("run_vector_vms called before allocating buffers for kernels..."),
        };

        println!("{:?}", buffers.stack_buffer);

        let default_hcall_size: [u8; 4] =
            unsafe { std::mem::transmute::<u32, [u8; 4]>(hypercall_buffer_size as u32) };
        let default_sp: [u8; 8] = unsafe { std::mem::transmute((0 as u64).to_le()) };
        let default_hypercall_num: [u8; 4] = unsafe { std::mem::transmute((-2 as i32).to_le()) };
        let default_hypercall_continuation: [u8; 4] =
            unsafe { std::mem::transmute((0 as i32).to_le()) };
        // points to _start
        let default_entry_point: [u8; 4] =
            unsafe { std::mem::transmute((self.entry_point as i32).to_le()) };
        // Important!! std::mem::transmute puts the bytes in the reverse order, we have to change it back!
        //default_entry_point.reverse();
        //default_sp.reverse();
        //default_hypercall_num.reverse();

        println!("{:?}", default_entry_point);
        // first, set up the default values for the VMs
        unsafe {
            // values that need to be set up for M.E.
            for idx in 0..(self.num_vms * mexec as u32) {
                let sp_result = ocl::core::enqueue_write_buffer(
                    &queue,
                    &buffers.sp,
                    true,
                    (idx * 8) as usize,
                    &default_sp,
                    None::<Event>,
                    None::<&mut Event>,
                );

                match sp_result {
                    Err(e) => panic!("sp_result, Error: {}", e),
                    _ => (),
                }

                // set the entry point!
                let entry_point_result = ocl::core::enqueue_write_buffer(
                    &queue,
                    &buffers.entry,
                    true,
                    (idx * 4) as usize,
                    &default_entry_point,
                    None::<Event>,
                    None::<&mut Event>,
                );

                match entry_point_result {
                    Err(e) => panic!("entry_point_result, Error: {}", e),
                    _ => (),
                }

                // set the default hypercall number to -2
                let hypercall_continuation_result = ocl::core::enqueue_write_buffer(
                    &queue,
                    &buffers.hypercall_continuation,
                    true,
                    (idx * 4) as usize,
                    &default_hypercall_continuation,
                    None::<Event>,
                    None::<&mut Event>,
                );

                match hypercall_continuation_result {
                    Err(e) => panic!("hypercall_continuation_result, Error: {}", e),
                    _ => (),
                }

                // set the default hypercall number to -2
                let hypercall_num_result = ocl::core::enqueue_write_buffer(
                    &queue,
                    &buffers.hypercall_num,
                    true,
                    (idx * 4) as usize,
                    &default_hypercall_num,
                    None::<Event>,
                    None::<&mut Event>,
                );

                match hypercall_num_result {
                    Err(e) => panic!("hypercall_num_result, Error: {}", e),
                    _ => (),
                }
            }

            for idx in 0..(self.num_vms) {
                // set the stack frame: stack_frames[sfp - 1] = sp
                let stack_frame_result = ocl::core::enqueue_write_buffer(
                    &queue,
                    &buffers.stack_frames,
                    true,
                    (idx * per_vm_stack_frames_size) as usize,
                    &default_sp,
                    None::<Event>,
                    None::<&mut Event>,
                );

                match stack_frame_result {
                    Err(e) => panic!("stack_frame_result, Error: {}", e),
                    _ => (),
                }

                // set the hcall_size
                let hcall_size_result = ocl::core::enqueue_write_buffer(
                    &queue,
                    &buffers.hcall_size,
                    true,
                    (idx * 4) as usize,
                    &default_hcall_size,
                    None::<Event>,
                    None::<&mut Event>,
                );

                match hcall_size_result {
                    Err(e) => panic!("hcall_size_result, Error: {}", e),
                    _ => (),
                }
            }
        }

        let global_dims = &[self.num_vms as usize * mexec, 1, 1];
        let local_dims = if local_work_group == 999999 {
            None
        } else {
            Some([local_work_group, 1, 1])
        };

        // run the data kernel to init the memory
        let data_kernel = kernels.get(&99999).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 0, ArgVal::mem(&buffers.stack_buffer)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 1, ArgVal::mem(&buffers.heap_buffer)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 2, ArgVal::mem(&buffers.globals_buffer)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 3, ArgVal::mem(&buffers.current_mem)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 4, ArgVal::mem(&buffers.max_mem)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 5, ArgVal::mem(&buffers.is_calling)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 6, ArgVal::mem(&buffers.sfp)).unwrap();
        ocl::core::set_kernel_arg(&data_kernel, 7, ArgVal::mem(&data_segment_buffer)).unwrap();

        // start counting only when all VM init is finished
        let e2e_time_start = std::time::Instant::now();

        let mut profiling_event = ocl::Event::empty();
        let mut map_event = ocl::Event::empty();

        unsafe {
            ocl::core::enqueue_kernel(
                &queue,
                &data_kernel,
                1,
                None,
                global_dims,
                local_dims,
                None::<Event>,
                Some(&mut profiling_event),
            )
            .unwrap();
        }

        ocl::core::wait_for_event(&profiling_event).unwrap();
        let queue_start_kernel = profiling_event
            .profiling_info(ocl::enums::ProfilingInfo::Queued)
            .unwrap()
            .time()
            .unwrap();
        let start_start_kernel = profiling_event
            .profiling_info(ocl::enums::ProfilingInfo::Start)
            .unwrap()
            .time()
            .unwrap();
        let end_start_kernel = profiling_event
            .profiling_info(ocl::enums::ProfilingInfo::End)
            .unwrap()
            .time()
            .unwrap();
        total_gpu_execution_time += end_start_kernel - start_start_kernel;
        queue_submit_delta += start_start_kernel - queue_start_kernel;

        // Free the data segment, we don't need it anymore as it has been copied to the heap
        // TODO: Figure out why release_mem_object causes a crash on program exit
        //unsafe { ocl::core::release_mem_object(&data_segment_buffer).unwrap() };
        let data_init_setup = std::time::Instant::now();

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
                ocl::core::set_kernel_arg(&value, 10, ArgVal::mem(&buffers.call_return_stack))
                    .unwrap();
                ocl::core::set_kernel_arg(&value, 11, ArgVal::mem(&buffers.hypercall_num)).unwrap();
                ocl::core::set_kernel_arg(&value, 12, ArgVal::mem(&buffers.hypercall_continuation))
                    .unwrap();
                ocl::core::set_kernel_arg(&value, 13, ArgVal::mem(&buffers.current_mem)).unwrap();
                ocl::core::set_kernel_arg(&value, 14, ArgVal::mem(&buffers.max_mem)).unwrap();
                ocl::core::set_kernel_arg(&value, 15, ArgVal::mem(&buffers.is_calling)).unwrap();
                ocl::core::set_kernel_arg(&value, 16, ArgVal::mem(&buffers.entry)).unwrap();
                ocl::core::set_kernel_arg(&value, 17, ArgVal::mem(&hcall_retval_buffer)).unwrap();
                ocl::core::set_kernel_arg(&value, 18, ArgVal::mem(&buffers.hcall_size)).unwrap();
                ocl::core::set_kernel_arg(&value, 19, ArgVal::mem(&buffers.overhead_tracker))
                    .unwrap();
            }
        }

        dbg!(self.entry_point);
        let mut start_kernel = kernels
            .get(&kernel_partition_mappings.get(&self.entry_point).unwrap())
            .unwrap();

        println!(
            "Set entry point: {:?}",
            kernel_part_debug
                .get(&kernel_partition_mappings.get(&self.entry_point).unwrap())
                .unwrap()
        );

        let mut max_queue_time: u64 = 0;
        let mut min_queue_time: u64 = std::u64::MAX;
        let mut num_queue_submits: u64 = 0;

        let mut called_funcs = HashSet::new();

        // We keep track of functions that we have currently blocked off here
        let mut divergence_stack = BTreeSet::new();
        // Also keep track of encountered hypercall entry points
        let mut hcall_divergence_stack = BTreeSet::new();
        // track stored entry points of hcalls
        let mut stored_entry_points =
            vec![0u32; (self.num_vms as usize * mexec).try_into().unwrap()];
        // Flag to write entry point at end of critical path
        let mut set_entry_point = false;

        let mut is_first: HashMap<u32, bool> = HashMap::new();
        let mut first_invokes: Vec<u64> = vec![];
        let mut repeat_invokes: Vec<u64> = vec![];
        let mut curr_func_id = *kernel_partition_mappings.get(&self.entry_point).unwrap();
        is_first.insert(curr_func_id, true);
        called_funcs.insert(curr_func_id);
        let mut num_batches = 0 as u128;
        let mut kernel_exec_time = 0 as u128;
        let mut is_serverless_invoke = false;

        let end_data_init_setup = std::time::Instant::now();
        vmm_overhead += (end_data_init_setup - data_init_setup).as_nanos();

        // now the data in the program has been initialized, we can run the main loop
        println!("start: {}", Utc::now().timestamp());
        loop {
            // run the kernel!
            // warning - bugged kernels can cause GPU driver hangs! Will result in the driver restarting...
            // Hangs are frequently a sign of a segmentation faults from inside of the GPU kernel
            // Unfortunately the OpenCL API doesn't give us a good way to identify what happened - the OS logs (dmesg) do have a record of this though

            let kernel_start = std::time::Instant::now();
            profiling_event = ocl::Event::empty();

            // For debugging
            println!(
                "Running partition: {:?}",
                kernel_part_debug.get(&curr_func_id).unwrap()
            );
            unsafe {
                num_queue_submits += 1;
                ocl::core::finish(&queue).unwrap();
                ocl::core::enqueue_kernel(
                    &queue,
                    &start_kernel,
                    1,
                    None,
                    global_dims,
                    local_dims,
                    None::<Event>,
                    Some(&mut profiling_event),
                )
                .expect(&format!(
                    "enqueue_kernel (start_kernel) error occured in partition group: {:?}",
                    kernel_part_debug.get(&curr_func_id).unwrap()
                ));
            }
            ocl::core::wait_for_event(&profiling_event).expect(&format!(
                "wait_for_event (start_kernel, profiling) error occured in partition group: {:?}",
                kernel_part_debug.get(&curr_func_id).unwrap()
            ));
            let kernel_end = std::time::Instant::now();
            kernel_exec_time += (kernel_end - kernel_start).as_nanos();

            let queue_start_kernel = profiling_event
                .profiling_info(ocl::enums::ProfilingInfo::Queued)
                .unwrap()
                .time()
                .unwrap();
            let start_start_kernel = profiling_event
                .profiling_info(ocl::enums::ProfilingInfo::Start)
                .unwrap()
                .time()
                .unwrap();
            let end_start_kernel = profiling_event
                .profiling_info(ocl::enums::ProfilingInfo::End)
                .unwrap()
                .time()
                .unwrap();

            total_gpu_execution_time += end_start_kernel - start_start_kernel;

            if start_start_kernel - queue_start_kernel > max_queue_time {
                max_queue_time = start_start_kernel - queue_start_kernel;
            }

            if start_start_kernel - queue_start_kernel < min_queue_time {
                min_queue_time = start_start_kernel - queue_start_kernel;
            }

            match is_first.get(&curr_func_id) {
                Some(true) => {
                    first_invokes.push(start_start_kernel - queue_start_kernel);
                    is_first.insert(curr_func_id, false);
                }
                Some(false) => {
                    repeat_invokes.push(start_start_kernel - queue_start_kernel);
                }
                None => {
                    first_invokes.push(start_start_kernel - queue_start_kernel);
                    is_first.insert(curr_func_id, false);
                }
            }

            queue_submit_delta += start_start_kernel - queue_start_kernel;
            // upon exiting we check the stack pointer for each VM
            let vmm_pre_overhead = std::time::Instant::now();

            unsafe {
                ocl::core::enqueue_read_buffer(
                    &queue,
                    &buffers.entry,
                    false,
                    0,
                    &mut entry_point_temp,
                    None::<Event>,
                    None::<&mut Event>,
                )
                .unwrap();
                ocl::core::enqueue_read_buffer(
                    &queue,
                    &buffers.hypercall_num,
                    true,
                    0,
                    &mut hypercall_num_temp,
                    None::<Event>,
                    None::<&mut Event>,
                )
                .unwrap();
            }

            // if all entry_point == -1, also exit
            /*
            entry_point_exit_flag = true;
            for e in &entry_point_temp {
                entry_point_exit_flag = (*e as i32 == (-1)) & entry_point_exit_flag;
            }

            if entry_point_exit_flag {
                let vmm_pre_overhead_end = std::time::Instant::now();
                vmm_overhead += (vmm_pre_overhead_end - vmm_pre_overhead).as_nanos();
                break;
            }
            */

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

            // for each VM, add to the set of kernels that we need to run next
            let mut non_serverless_invoke_call_found = false;
            for idx in 0..(self.num_vms as usize * mexec) {
                if hypercall_num_temp[idx] != 9999 {
                    non_serverless_invoke_call_found = true;
                }
                // For each VM, add the next function to run to the divergence stack
                // If the following conditions are met:
                // 1) We are not blocked on a hypercall
                // 2) The VM is not currently masked off
                if entry_point_temp[idx] != ((-1) as i32) as u32
                    && (hypercall_num_temp[idx] == -2 || hypercall_num_temp[idx] == -1)
                {
                    divergence_stack.insert(
                        *kernel_partition_mappings
                            .get(&entry_point_temp[idx])
                            .unwrap(),
                    );
                    stored_entry_points[idx] = entry_point_temp[idx];
                } else if hypercall_num_temp[idx] != -2
                    && entry_point_temp[idx] != ((-1) as i32) as u32
                {
                    hcall_divergence_stack.insert(
                        *kernel_partition_mappings
                            .get(&entry_point_temp[idx])
                            .unwrap(),
                    );
                    stored_entry_points[idx] = entry_point_temp[idx];
                    entry_point_temp[idx] = ((-1) as i32) as u32;
                }
            }

            //println!("divergence stack: {:?}", &divergence_stack);
            //println!("hcall divergence stack: {:?}", &hcall_divergence_stack);

            // if we found a VM that needs to run another function, we do that first
            if divergence_stack.len() > 0 {
                // Pop off the next function to run
                let first_item = divergence_stack.clone().into_iter().collect::<Vec<u32>>()[0];
                let next_func = divergence_stack.take(&first_item).unwrap();

                start_kernel = kernels.get(&next_func).unwrap();
                curr_func_id = next_func;
                called_funcs.insert(curr_func_id);

                /*
                for idx in 0..(self.num_vms as usize) {
                    entry_point_temp[idx] = next_func;
                }
                */

                unsafe {
                    ocl::core::enqueue_write_buffer(
                        &queue,
                        &buffers.entry,
                        false,
                        0,
                        &mut entry_point_temp,
                        None::<Event>,
                        None::<&mut Event>,
                    )
                    .unwrap();
                }

                let vmm_pre_overhead_end = std::time::Instant::now();
                vmm_overhead += (vmm_pre_overhead_end - vmm_pre_overhead).as_nanos();
                continue;
            } else {
                // if we don't have any VMs to run, reset the next function to run to be that of the hcall
                // we are returning to and dispatch the calls

                // To do this, we pop from the hcall_divergence stack, and set this as the next
                // function to execute. The rest of the hcall_divergence stack gets added to the
                // regular divergence stack.
                let first_item = hcall_divergence_stack
                    .clone()
                    .into_iter()
                    .collect::<Vec<u32>>()[0];
                let next_func = hcall_divergence_stack.take(&first_item).unwrap();

                start_kernel = kernels.get(&next_func).unwrap();
                curr_func_id = next_func;
                called_funcs.insert(curr_func_id);

                for func_to_return_to in hcall_divergence_stack.clone().into_iter() {
                    divergence_stack
                        .insert(hcall_divergence_stack.take(&func_to_return_to).unwrap());
                }

                // Remember to write the entry points back at the end
                set_entry_point = true;

                for idx in 0..(self.num_vms as usize * mexec) {
                    entry_point_temp[idx] = stored_entry_points[idx];
                }

                let vmm_pre_overhead_end = std::time::Instant::now();
                vmm_overhead += (vmm_pre_overhead_end - vmm_pre_overhead).as_nanos();
            }

            /*
             * For serverless_invoke we can only copy inputs to the GPU using our async API if
             * we have no other calls to dispatch. So if we see >0 non-invoke calls we block
             * VMs waiting for inputs until we are ready.
             */
            if non_serverless_invoke_call_found {
                for idx in 0..(self.num_vms as usize * mexec) {
                    if hypercall_num_temp[idx] == (WasiSyscalls::ServerlessInvoke as i32) {
                        entry_point_temp[idx] = ((-1) as i32) as u32;
                    }
                }
            }

            let mut resp_ctr = 0;
            let mut rand_ctr = 0;
            let mut invoke = 0;
            for call in &hypercall_num_temp {
                if *call == 10000 {
                    resp_ctr += 1;
                } else if *call == 6 {
                    rand_ctr += 1;
                } else if *call == 9999 {
                    invoke += 1;
                }
            }

            // read the hypercall_buffer
            let start_hcall_dispatch = std::time::Instant::now();
            unsafe {
                let buf: &mut [u8] = *hcall_read_buffer.buf.get();
                let overhead_buf: &mut [u64] = *overhead_tracker_buffer.buf.get();

                // We don't need to read previous buffer values for serverless invoke
                ocl::core::enqueue_read_buffer(
                    &queue,
                    &hypercall_buffer,
                    false,
                    0,
                    buf,
                    None::<Event>,
                    None::<&mut Event>,
                )
                .unwrap();
                ocl::core::enqueue_read_buffer(
                    &queue,
                    &buffers.sp,
                    false,
                    0,
                    &mut stack_pointer_temp,
                    None::<Event>,
                    None::<&mut Event>,
                )
                .unwrap();
                ocl::core::enqueue_read_buffer(
                    &queue,
                    &buffers.overhead_tracker,
                    true,
                    0,
                    overhead_buf,
                    None::<Event>,
                    None::<&mut Event>,
                )
                .unwrap();
            }

            num_batches += 1;
            println!("vmm overhead {}, context: {:p}", vmm_overhead, ctx.as_ptr());
            println!("kernel_exec_time: {}", kernel_exec_time);
            kernel_exec_time = 0;
            // now it is time to dispatch hypercalls

            //dbg!(&hypercall_num_temp);

            let start_hcall_dispatch2 = std::time::Instant::now();
            vm_slice.as_slice().iter().for_each(|vm_idx| {
                let hypercall_id =
                    WasiSyscalls::try_from(hypercall_num_temp[*vm_idx as usize * mexec]).unwrap();

                let hcall = Box::new(HyperCall::new(
                    (*vm_idx as u32).clone(),
                    number_vms,
                    total_gpu_execution_time,
                    queue_submit_delta,
                    num_queue_submits,
                    called_funcs.clone(),
                    hypercall_id,
                    self.is_memory_interleaved,
                    &buffers,
                    hcall_read_buffer.clone(),
                    queue,
                    overhead_tracker_buffer.clone(),
                    non_serverless_invoke_call_found,
                ));
                let serverless_invoke = if hypercall_num_temp[*vm_idx as usize]
                    == (WasiSyscalls::ServerlessInvoke as i32)
                {
                    true
                } else {
                    false
                };
                hcall_sender_vec[*vm_idx as usize]
                    .send((hcall, serverless_invoke))
                    .unwrap();
            });

            let end_hcall_dispatch2 = std::time::Instant::now();
            println!(
                "hcall send time: {}",
                (end_hcall_dispatch2 - start_hcall_dispatch2).as_nanos()
            );

            //println!("messages in channel: {}", hcall_sender.len());
            //println!("responses in channel: {}", result_receiver.len());

            // now block until all of the hypercalls have been successfully dispatched
            // first, check for how many hcalls we have to wait on
            let mut number_hcalls_blocking = 0;
            let mut num_hcall_resp = 0;
            // count how many serverless response calls we get
            for hypercall_idx in 0..hypercall_num_temp.len() / mexec {
                match WasiSyscalls::try_from(hypercall_num_temp[hypercall_idx * mexec]).unwrap() {
                    // We are performing async responses for all serverless_response calls
                    WasiSyscalls::ServerlessResponse => {
                        for m in 0..mexec {
                            hypercall_num_temp[hypercall_idx * mexec + m] = -1;
                        }
                        hypercall_retval_temp[hypercall_idx] = 0;
                        num_hcall_resp += 1;
                    }
                    WasiSyscalls::ServerlessInvoke => {
                        // no-ops for serverless invoke calls that are still blocked
                        if !non_serverless_invoke_call_found {
                            number_hcalls_blocking += 1;
                        }
                    }
                    _ => {
                        number_hcalls_blocking += 1;
                    }
                };
            }

            let mut total_recv = 0;
            for _idx in 0..(number_hcalls_blocking + num_hcall_resp) {
                let start_recv = std::time::Instant::now();
                let result = result_receiver.recv().unwrap();
                let end_recv = std::time::Instant::now();
                //dbg!(&_idx);
                //println!("hcall recv time: {}", (end_recv-start_recv).as_nanos());
                total_recv += (end_recv - start_recv).as_nanos();
                // we want to special case proc_exit to exit the VM
                match result.get_type() {
                    WasiSyscalls::ProcExit => {
                        for m in 0..mexec {
                            entry_point_temp[result.get_vm_id() as usize * mexec + m] =
                                ((-1) as i32) as u32
                        }
                    }
                    WasiSyscalls::ServerlessInvoke => is_serverless_invoke = true,
                    WasiSyscalls::ServerlessResponse => (),
                    _ => (),
                }
                // after all of the hypercalls are finished, we should update all of the stack pointers
                for m in 0..mexec {
                    hypercall_num_temp[result.get_vm_id() as usize * mexec + m] = -1;
                }

                hypercall_retval_temp[result.get_vm_id() as usize] = result.get_result();
            }

            let end_hcall_dispatch = std::time::Instant::now();
            println!(
                "hcall dispatch time: {}",
                (end_hcall_dispatch - start_hcall_dispatch).as_nanos()
            );
            hcall_execution_time += (end_hcall_dispatch - start_hcall_dispatch).as_nanos();

            println!("hcall total recv time: {}", total_recv);
            let vmm_post_overhead = std::time::Instant::now();

            // check again for threads that may be done - this is because
            // proc_exit(...) can actually block off additional threads
            // we don't have to read again, we can have proc_exit write directly to entry_point_temp
            entry_point_exit_flag = true;
            for e in &entry_point_temp {
                entry_point_exit_flag = (*e as i32 == (-1)) & entry_point_exit_flag;
            }

            if entry_point_exit_flag {
                let vmm_post_overhead_end = std::time::Instant::now();
                vmm_overhead += (vmm_post_overhead_end - vmm_post_overhead).as_nanos();
                break;
            }

            // wait until *sp is read
            ocl::core::finish(&queue).unwrap();

            // now set the entry_point of exited procs to -1 if sp == 0
            for (idx, sp) in stack_pointer_temp.iter().enumerate() {
                if *sp == 0 as u64 {
                    // this cast is hacky, but it does the C equivalent of (uint)(-1)
                    entry_point_temp[idx] = ((-1) as i32) as u32;
                    set_entry_point = true;
                }
            }

            // update the entry point to resume execution
            // update all of the stack pointers
            // update the hypercall numbers to -1 to indicate that we are now returning from the hypercall
            // also don't forget to write the hcall buf back
            let write_start = std::time::Instant::now();
            unsafe {
                // if number_hcalls_blocking == 0, then that means every hcall was a response,
                // and we can skip writing this buffer back to the VM
                if number_hcalls_blocking > 0 {
                    let hcall_buf = if is_serverless_invoke && !non_serverless_invoke_call_found {
                        &*hcall_async_buffer.lock().unwrap().buf.get()
                    } else {
                        &*hcall_read_buffer.buf.get()
                    };

                    ocl::core::enqueue_write_buffer(
                        &queue,
                        &hypercall_buffer,
                        true,
                        0,
                        hcall_buf,
                        None::<Event>,
                        None::<&mut Event>,
                    )
                    .unwrap();
                    if is_serverless_invoke && !non_serverless_invoke_call_found {
                        invoke_complete.broadcast(true);
                        is_serverless_invoke = false;
                    }
                }
                if set_entry_point {
                    ocl::core::enqueue_write_buffer(
                        &queue,
                        &buffers.entry,
                        false,
                        0,
                        &mut entry_point_temp,
                        None::<Event>,
                        None::<&mut Event>,
                    )
                    .unwrap();
                    set_entry_point = false;
                }
                ocl::core::enqueue_write_buffer(
                    &queue,
                    &buffers.hypercall_num,
                    false,
                    0,
                    &mut hypercall_num_temp,
                    None::<Event>,
                    None::<&mut Event>,
                )
                .unwrap();
                ocl::core::enqueue_write_buffer(
                    &queue,
                    &hcall_retval_buffer,
                    false,
                    0,
                    &mut hypercall_retval_temp,
                    None::<Event>,
                    None::<&mut Event>,
                )
                .unwrap();
            }
            let vmm_post_overhead_end = std::time::Instant::now();
            let write_end = std::time::Instant::now();
            println!("write time: {}", (write_end - write_start).as_nanos());
            vmm_overhead += (vmm_post_overhead_end - vmm_post_overhead).as_nanos();
        }

        let e2e_time_end = std::time::Instant::now();

        // To get final results back from the stack if we want for debugging stuff
        // only uncomment this out if you need to debug stuff, it will panic if you have too many VMs and too small of a buffer
        dbg!(print_return);
        if print_return {
            let mut check_results_debug = vec![0u8; (self.num_vms * 1024) as usize];
            unsafe {
                ocl::core::enqueue_read_buffer(
                    &queue,
                    &buffers.stack_buffer,
                    true,
                    0,
                    &mut check_results_debug,
                    None::<Event>,
                    None::<&mut Event>,
                )
                .unwrap();
            }

            /*
             * When multi-execution is enabled, VMs with nearby idx's have the same result. (i.e. 0,1,2,3 -> are all really VM 0)
             */
            for vm_idx in 0..self.num_vms as u32 {
                if self.is_memory_interleaved > 0 {
                    let result_i32 = Interleave::read_u32(
                        &mut check_results_debug[512..],
                        0,
                        self.num_vms,
                        vm_idx,
                        self.is_memory_interleaved,
                    );
                    let result_i64 = Interleave::read_u64(
                        &mut check_results_debug[512..],
                        0,
                        self.num_vms,
                        vm_idx,
                        self.is_memory_interleaved,
                    );

                    let result_u128 = Interleave::read_u128(
                        &mut check_results_debug[512..],
                        0,
                        self.num_vms,
                        vm_idx,
                        self.is_memory_interleaved,
                    );

                    dbg!(result_i32 as i32);
                    dbg!(result_i64 as i64);
                    dbg!(result_i32 as u32);
                    dbg!(result_i64 as u64);
                    dbg!(result_u128 as u128);

                    let bytes_i32: [u8; 4] = unsafe { transmute(result_i32.to_le()) };
                    let bytes_i64: [u8; 8] = unsafe { transmute(result_i64.to_le()) };

                    dbg!(f32::from_le_bytes(bytes_i32));
                    dbg!(f64::from_le_bytes(bytes_i64));
                } else {
                    let result_i32 = LittleEndian::read_u32(&check_results_debug[512..516]);
                    let result_i64 = LittleEndian::read_u64(&check_results_debug[512..520]);
                    let result_u128 = LittleEndian::read_u128(&check_results_debug[512..528]);

                    dbg!(result_i32 as i32);
                    dbg!(result_i64 as i64);
                    dbg!(result_i32 as u32);
                    dbg!(result_i64 as u64);
                    dbg!(result_u128 as u128);

                    let bytes_i32: [u8; 4] = unsafe { transmute(result_i32.to_le()) };
                    let bytes_i64: [u8; 8] = unsafe { transmute(result_i64.to_le()) };

                    dbg!(f32::from_le_bytes(bytes_i32));
                    dbg!(f64::from_le_bytes(bytes_i64));
                }
            }
        }

        println!("end: {}", Utc::now().timestamp());
        println!(
            "E2E execution time in nanoseconds: {}",
            (e2e_time_end - e2e_time_start).as_nanos()
        );
        println!(
            "On device time in nanoseconds: {}",
            total_gpu_execution_time
        );
        println!(
            "Device Start-Queue overhead in nanoseconds: {}",
            queue_submit_delta
        );
        println!(
            "Max Device Start-Queue overhead in nanoseconds: {}",
            max_queue_time
        );
        println!(
            "Min Device Start-Queue overhead in nanoseconds: {}",
            min_queue_time
        );
        println!(
            "Average Device Start-Queue overhead in nanoseconds: {}",
            queue_submit_delta / num_queue_submits
        );
        println!("Number of queue submits: {}", num_queue_submits);
        println!(
            "fraction of time on device overhead: {}",
            queue_submit_delta as f64 / (e2e_time_end - e2e_time_start).as_nanos() as f64
        );
        println!(
            "fraction of time on device: {}",
            total_gpu_execution_time as f64 / (e2e_time_end - e2e_time_start).as_nanos() as f64
        );
        println!(
            "fraction of time on hcall dispatch: {}",
            hcall_execution_time as f64 / (e2e_time_end - e2e_time_start).as_nanos() as f64
        );
        println!(
            "fraction of time on VMM overhead: {}",
            vmm_overhead as f64 / (e2e_time_end - e2e_time_start).as_nanos() as f64
        );

        unsafe {
            let overhead_buf: &mut [u64] = *overhead_tracker_buffer.buf.get();
            ocl::core::enqueue_read_buffer(
                &queue,
                &buffers.overhead_tracker,
                true,
                0,
                overhead_buf,
                None::<Event>,
                None::<&mut Event>,
            )
            .unwrap();
            println!(
                "Context saving overhead time for VM #0 (ns): {:?}",
                overhead_buf[0]
            );
        }

        //dbg!(&first_invokes);
        let mut avg = 0;
        for val in &first_invokes {
            avg += val;
        }
        if first_invokes.len() > 0 {
            println!(
                "Average of first invokes: {:?}, # first invokes: {:?}",
                avg / first_invokes.len() as u64,
                first_invokes.len()
            );
        }

        avg = 0;
        for val in &repeat_invokes {
            avg += val;
        }

        if repeat_invokes.len() > 0 {
            println!(
                "Average of repeat invokes: {:?}, # repeats: {:?}",
                avg / repeat_invokes.len() as u64,
                repeat_invokes.len()
            );
        }
        return VMMRuntimeStatus::StatusOkay;
    }
}
