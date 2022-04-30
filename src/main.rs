extern crate ocl;

mod batch_submit;
mod opencl_runner;
mod opencl_writer;
mod wasmtime_runner;

use batch_submit::BatchSubmitServer;
use clap::{value_t, App, Arg};
use crossbeam::sync::WaitGroup;
use ocl::core::ContextProperties;
use opencl_runner::vectorized_vm::{VmRecvType, VmSenderType};
use opencl_runner::InputProgram;
use opencl_runner::PartitionedSeralizedProgram;
use opencl_runner::SeralizedProgram;
use opencl_writer::bpatch;
use rayon::prelude::*;
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::panic::panic_any;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tokio::sync::mpsc;
use tokio::sync::Mutex as AsyncMutex;
use wasmtime_runner::WasmtimeRunner;
use wast::parser::ParseBuffer;

fn main() {
    // TODO add Clap arg parsing here to get the WASM files from CLI
    // also consider supporting .wasm files as well?

    let matches = App::new("vectorvisor")
        .version("0.1")
        .author("Sam Ginzburg <ginzburg.sam@gmail.com>")
        .about("A Vectorized Virtual Machine Monitor for WebAssembly")
        .arg(Arg::with_name("input")
            .required(true)
            .short("i")
            .long("input")
            .value_name("")
            .help("The input .wasm/.wat/precompiled binary file to run")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("stack")
            .short("s")
            .long("stack")
            .value_name("STACK SIZE (bytes)")
            .help("The size of the program stack per virtual machine (bytes)")
            .default_value("1048576") // default of 1MiB
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("heap")
            .short("h")
            .long("heap")
            .value_name("HEAP SIZE (bytes)")
            .default_value("1048576") // default of 1MiB
            .help("The size of the program heap per virtual machine (bytes)")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("vmcount")
            .short("v")
            .long("vmcount")
            .value_name("NUM VMs (number of VMs to run)")
            .default_value("64") // default of 64
            .help("The number of VMs to run in parallel")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("vmgroups")
            .long("vmgroups")
            .value_name("NUM VM GROUPS (number of VMs to run * num groups = total VMs to run)")
            .default_value("1") // default of 1
            .help("The number of VMs to run in parallel")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("interleave")
            .short("l")
            .long("interleave")
            .help("Select the cell size for memory interleaving (disabled = 0, supported = 1, 8)")
            .default_value("1") // default to true
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("isgpu")
            .short("g")
            .long("isgpu")
            .help("If this flag is true, the VMM will run the given program on the GPU. Otherwise, the program is run on the OpenCL CPU backend (if available).")
            .default_value("true") // default to the GPU
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        // now for less frequently changed arguments...
        .arg(Arg::with_name("callstack")
            .short("cs")
            .long("callstack")
            .value_name("CALL STACK SIZE (function call depth)")
            .default_value("1024") // default of 1024
            .help("The maximum recursive depth of each VM")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("debugcallprint")
            .short("d")
            .long("debugcallprint")
            .value_name("DEBUG PRINT FN CALLS (true/false)")
            .default_value("false") // default of 1024
            .help("Print the name of the WASM function being called during execution (huge overhead)")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("cflags")
            .long("cflags")
            .value_name("CLI arguments to pass to compile_program")
            .default_value("")
            .help("Pass args to the program compilation step")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("printreturn")
            .long("printreturn")
            .value_name("Print the last 4 bytes on the stack after proc_exit")
            .default_value("false")
            .help("Print the last 4 bytes on the stack after proc_exit")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("ldflags")
            .long("ldflags")
            .value_name("CLI arguments to pass to link_program")
            .default_value("")
            .help("Pass args to the program linking step")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        // the following flags are *only* for use with passing in PTX files
        .arg(Arg::with_name("entry")
            .long("entry")
            .value_name("Entry point of kernel")
            .default_value("")
            .help("Indicate the numerical entry point of the GPU kernel (for use with PTX input)")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("numfuncs")
            .long("numfuncs")
            .value_name("Total number of functions in kernel")
            .default_value("")
            .help("Indicate the total number of functions in the file (for use with PTX input)")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("globals-buffer-size")
            .long("globals-buffer-size")
            .value_name("Size of the globals buffer")
            .default_value("")
            .help("Indicate the size of the globals buffer (for use with PTX input)")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        // add a param for cases where we want to only get the .cl file and compile externally from the driver
        // this is useful in cases where we want to manually compile to PTX/AMDGPU/SPIRV binaries
        .arg(Arg::with_name("compile")
            .long("compile")
            .value_name("Input WASM code to compile")
            .default_value("false")
            .help("This flag only compiles the input WASM to OpenCL C and saves the file to disk for later compilation")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        // experimental parameter for fast loading of programs
        .arg(Arg::with_name("partition")
            .long("partition")
            .value_name("Partition the program into multiple kernels for faster JIT compilation")
            .default_value("true")
            .help("This flag breaks up a given input program into many OpenCL kernels for faster JIT compilation of kernels")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("forceinline")
            .long("forceinline")
            .value_name("Force the compiler to inline all functions")
            .default_value("false")
            .help("This flag adds the inline trait to all functions")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("wasmtime")
            .long("wasmtime")
            .value_name("Run WASM code using the Wasmtime JIT runtime on the CPU-only")
            .default_value("false")
            .help("This flag runs the input program on the CPU using the Wasmtime JIT runtime")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("serverless")
            .long("serverless")
            .value_name("Enable listening on the HTTP endpoint for incoming requests")
            .default_value("false")
            .help("This flag leaves the VMM running while waiting for inputs for serverless functions")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("ip")
            .long("ip")
            .value_name("Listen on this IP for incoming requests")
            .default_value("127.0.0.1")
            .help("Listen on different IPs locally")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("port")
            .long("port")
            .value_name("Listen on this port for incoming requests")
            .default_value("8000")
            .help("Listen on different ports locally")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("hcallsize")
            .long("hcallsize")
            .value_name("Size of the hypercall buffer used for serverless inputs & system calls")
            .default_value("16384") // default val is 16KiB, this value cannot be larger than the size of the heap
            .help("The size of the hypercall buffer to allocate")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("partitions")
            .long("partitions")
            .value_name("Maximum number of functions that can be grouped together in the same GPU kernel")
            .default_value("100")
            .help("Maximum number of functions that can be grouped together in the same GPU kernel")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("maxloc")
            .long("maxloc")
            .value_name("Maximum number of lines of code that can be in a partitioned GPU kernel")
            .default_value("500000")
            .help("Maximum number of lines of code that can be in a partitioned GPU kernel")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("maxdup")
            .long("maxdup")
            .value_name("Maximum number of copies of a function that can exist across all partitioned GPU kernels")
            .default_value("1")
            .help("Maximum number of copies of a function that can exist across all partitioned GPU kernels")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("disablefastcalls")
            .long("disablefastcalls")
            .value_name("Disables fastcall optimization for target program")
            .default_value("false")
            .help("This flag disables fastcall optimization for target program if set to true")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("localworkgroup")
            .long("lgroup")
            .value_name("Specifies the number of threads to be grouped together into a local workgroup")
            .default_value("64") // 999999 is the fake default val, we just let OpenCL auto-determine in this case
            .help("This flag sets the max size of the local work group. This affects occupancy & register usage.")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("mexec")
            .long("mexec")
            .value_name("Specifies how many GPU threads per-VM are assigned to improve read parallelism")
            .default_value("1") // This value is a multiplier, so 1 || 0 = disabled.
            .help("")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("pinput")
            .long("pinput")
            .value_name("Specifies that the input WASM was generated by another compiler with proper alignments for reads/writes.")
            .default_value("false")
            .help("")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("fastreply")
            .long("fastreply")
            .value_name("Reply with only the function response and no debugging data")
            .default_value("false")
            .help("")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("max_smem_demo_space")
            .long("maxdemospace")
            .value_name("Specifies the amount of shared memory to allocate for register demotion for large kernels")
            .default_value("0")
            .help("")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .arg(Arg::with_name("reqtimeout")
            .long("rt")
            .value_name("Specifies the timeout to wait while buffering requests")
            .default_value("2000")
            .help("")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .get_matches();

    dbg!(matches.clone());

    let file_path = value_t!(matches.value_of("input"), String).unwrap_or_else(|e| e.exit());
    let interleave = value_t!(matches.value_of("interleave"), u32).unwrap_or_else(|e| e.exit());
    let stack_size = value_t!(matches.value_of("stack"), u32).unwrap_or_else(|e| e.exit());
    let heap_size = value_t!(matches.value_of("heap"), u32).unwrap_or_else(|e| e.exit());
    let call_stack_size = value_t!(matches.value_of("callstack"), u32).unwrap_or_else(|e| e.exit());
    let stack_frames_size = call_stack_size;
    let sfp_size = call_stack_size;
    let predictor_size = 4096;
    let num_vms = value_t!(matches.value_of("vmcount"), u32).unwrap_or_else(|e| e.exit());
    let num_vm_groups = value_t!(matches.value_of("vmgroups"), u32).unwrap_or_else(|e| e.exit());
    let is_gpu = value_t!(matches.value_of("isgpu"), bool).unwrap_or_else(|e| e.exit());
    let print_return = value_t!(matches.value_of("printreturn"), bool).unwrap_or_else(|e| e.exit());
    let debug_call_print =
        value_t!(matches.value_of("debugcallprint"), bool).unwrap_or_else(|e| e.exit());
    let compile_args = value_t!(matches.value_of("cflags"), String).unwrap_or_else(|e| e.exit());
    let link_args = value_t!(matches.value_of("ldflags"), String).unwrap_or_else(|e| e.exit());
    let force_inline = value_t!(matches.value_of("forceinline"), bool).unwrap_or_else(|e| e.exit());
    let partition = value_t!(matches.value_of("partition"), bool).unwrap_or_else(|e| e.exit());
    let wasmtime = value_t!(matches.value_of("wasmtime"), bool).unwrap_or_else(|e| e.exit());
    let serverless = value_t!(matches.value_of("serverless"), bool).unwrap_or_else(|e| e.exit());
    let hcall_size = value_t!(matches.value_of("hcallsize"), usize).unwrap_or_else(|e| e.exit());
    let batch_submit_ip = value_t!(matches.value_of("ip"), String).unwrap_or_else(|e| e.exit());
    let batch_submit_port = value_t!(matches.value_of("port"), u32).unwrap_or_else(|e| e.exit());
    let max_part = value_t!(matches.value_of("partitions"), u32).unwrap_or_else(|e| e.exit());
    let max_loc = value_t!(matches.value_of("maxloc"), u32).unwrap_or_else(|e| e.exit());
    let max_dup = value_t!(matches.value_of("maxdup"), u32).unwrap_or_else(|e| e.exit());
    let disable_fastcalls =
        value_t!(matches.value_of("disablefastcalls"), bool).unwrap_or_else(|e| e.exit());
    let local_work_group =
        value_t!(matches.value_of("localworkgroup"), usize).unwrap_or_else(|e| e.exit());
    let mexec = value_t!(matches.value_of("mexec"), usize).unwrap_or_else(|e| e.exit());
    let pinput = value_t!(matches.value_of("pinput"), bool).unwrap_or_else(|e| e.exit());
    let fastreply = value_t!(matches.value_of("fastreply"), bool).unwrap_or_else(|e| e.exit());
    let maxdemospace =
        value_t!(matches.value_of("max_smem_demo_space"), u32).unwrap_or_else(|e| e.exit());
    let req_timeout = value_t!(matches.value_of("reqtimeout"), u32).unwrap_or_else(|e| e.exit());

    if mexec > 1 && interleave == 0 {
        panic!("Multi-Execution is only enabled for interleaved workloads!");
    }

    dbg!(compile_args.clone());

    let compile = value_t!(matches.value_of("compile"), bool).unwrap_or_else(|e| e.exit());
    if compile {
        let extension = match Path::new(&file_path).extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => "none",
        };

        let filedata = match extension {
            "wat" => fs::read_to_string(file_path.clone()).unwrap(),
            "wasm" => wasmprinter::print_file(file_path.clone()).unwrap(),
            _ => panic!("Unknown file extension for compilation!"),
        };

        let pb = ParseBuffer::new(&filedata).unwrap();
        let pb_bpatch = ParseBuffer::new(&bpatch::PATCH_FILE).unwrap();
        let pb_debug = ParseBuffer::new(&filedata).unwrap();
        let mut ast = opencl_writer::OpenCLCWriter::new(&pb, &pb_bpatch, pinput);
        let mut ast_debug = opencl_writer::OpenCLCWriter::new(&pb_debug, &pb_bpatch, pinput);
        let _result = ast.parse_file().unwrap();
        let _result_debug = ast_debug.parse_file().unwrap();
        let (
            compiled_kernel,
            fastcall_header,
            entry_point,
            globals_buffer_size,
            num_compiled_funcs,
            _,
            _,
            _,
        ) = ast.write_opencl_file(
            hcall_size.try_into().unwrap(),
            interleave,
            stack_size,
            heap_size,
            call_stack_size,
            stack_frames_size,
            sfp_size,
            predictor_size,
            max_part,
            max_loc,
            max_dup,
            maxdemospace,
            local_work_group,
            mexec,
            disable_fastcalls,
            debug_call_print,
            force_inline,
            is_gpu,
            false,
        );

        println!("The following info is needed to later run compiled pre-compiled/externally compiled binaries");
        println!("Compiled: {} functions", num_compiled_funcs);
        println!("Entry point: {}", entry_point);
        println!("Globals buffer: {}", globals_buffer_size);
        println!("interleave: {}", interleave);

        let mut file = File::create(format!("{}.cl", file_path)).unwrap();
        file.write_all(&compiled_kernel.clone().into_bytes())
            .unwrap();

        let mut file = File::create(format!("{}_fastcalls.cl", file_path)).unwrap();
        file.write_all(&fastcall_header.clone().into_bytes())
            .unwrap();

        return;
    }

    // start an HTTP endpoint for submitting batch jobs/
    // pass in the channels we use to send requests back and forth

    if !wasmtime {
        let extension = match Path::new(&file_path).extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => "none",
        };

        let (file, entry_point, num_compiled_funcs, globals_buffer_size, interleaved) = match (
            extension, partition,
        ) {
            ("wat", false) => {
                let filedata = match fs::read_to_string(file_path.clone()) {
                    Ok(text) => text,
                    Err(e) => panic_any(e),
                };
                let pb = ParseBuffer::new(&filedata).unwrap();
                let pb_bpatch = ParseBuffer::new(&bpatch::PATCH_FILE).unwrap();
                let pb_debug = ParseBuffer::new(&filedata).unwrap();
                let mut ast = opencl_writer::OpenCLCWriter::new(&pb, &pb_bpatch, pinput);
                let mut ast_debug =
                    opencl_writer::OpenCLCWriter::new(&pb_debug, &pb_bpatch, pinput);
                let _result = ast.parse_file().unwrap();
                let _result_debug = ast_debug.parse_file().unwrap();

                // apply our compilation pass to the source WASM
                let (
                    compiled_kernel,
                    fastcall_header,
                    entry_point,
                    globals_buffer_size,
                    num_compiled_funcs,
                    _kernel_hashmap,
                    _kernel_compile_stats,
                    _kernel_partition_mappings,
                ) = ast.write_opencl_file(
                    hcall_size.try_into().unwrap(),
                    interleave,
                    stack_size,
                    heap_size,
                    call_stack_size,
                    stack_frames_size,
                    sfp_size,
                    predictor_size,
                    max_part,
                    max_loc,
                    max_dup,
                    maxdemospace,
                    local_work_group,
                    mexec,
                    disable_fastcalls,
                    debug_call_print,
                    force_inline,
                    is_gpu,
                    false,
                );
                println!("Compiled: {} functions", num_compiled_funcs);
                println!("Entry point: {}", entry_point);
                println!("Globals buffer: {}", globals_buffer_size);
                println!("interleave: {}", interleave);

                (
                    InputProgram::Text(compiled_kernel.clone(), fastcall_header.clone()),
                    entry_point,
                    num_compiled_funcs,
                    globals_buffer_size,
                    interleave,
                )
            }
            ("wasm", false) => {
                let filedata_text = wasmprinter::print_file(file_path.clone()).unwrap();
                let pb = ParseBuffer::new(&filedata_text).unwrap();
                let pb_bpatch = ParseBuffer::new(&bpatch::PATCH_FILE).unwrap();
                let pb_debug = ParseBuffer::new(&filedata_text).unwrap();
                let mut ast = opencl_writer::OpenCLCWriter::new(&pb, &pb_bpatch, pinput);
                let mut ast_debug =
                    opencl_writer::OpenCLCWriter::new(&pb_debug, &pb_bpatch, pinput);
                let _result = ast.parse_file().unwrap();
                let _result_debug = ast_debug.parse_file().unwrap();

                // apply our compilation pass to the source WASM
                let (
                    compiled_kernel,
                    fastcall_header,
                    entry_point,
                    globals_buffer_size,
                    num_compiled_funcs,
                    _kernel_hashmap,
                    _kernel_compile_stats,
                    _kernel_partition_mappings,
                ) = ast.write_opencl_file(
                    hcall_size.try_into().unwrap(),
                    interleave,
                    stack_size,
                    heap_size,
                    call_stack_size,
                    stack_frames_size,
                    sfp_size,
                    predictor_size,
                    max_part,
                    max_loc,
                    max_dup,
                    maxdemospace,
                    local_work_group,
                    mexec,
                    disable_fastcalls,
                    debug_call_print,
                    force_inline,
                    is_gpu,
                    false,
                );
                println!("Compiled: {} functions", num_compiled_funcs);
                println!("Entry point: {}", entry_point);
                println!("Globals buffer: {}", globals_buffer_size);
                println!("interleave: {}", interleave);

                (
                    InputProgram::Text(compiled_kernel.clone(), fastcall_header.clone()),
                    entry_point,
                    num_compiled_funcs,
                    globals_buffer_size,
                    interleave,
                )
            }
            ("wat", true) => {
                let filedata = match fs::read_to_string(file_path.clone()) {
                    Ok(text) => text,
                    Err(e) => panic_any(e),
                };
                let pb = ParseBuffer::new(&filedata).unwrap();
                let pb_bpatch = ParseBuffer::new(&bpatch::PATCH_FILE).unwrap();
                let pb_debug = ParseBuffer::new(&filedata).unwrap();
                let mut ast = opencl_writer::OpenCLCWriter::new(&pb, &pb_bpatch, pinput);
                let mut ast_debug =
                    opencl_writer::OpenCLCWriter::new(&pb_debug, &pb_bpatch, pinput);
                let _result = ast.parse_file().unwrap();
                let _result_debug = ast_debug.parse_file().unwrap();

                // apply our compilation pass to the source WASM
                let (
                    _compiled_kernel,
                    fastcall_header,
                    entry_point,
                    globals_buffer_size,
                    num_compiled_funcs,
                    kernel_hashmap,
                    kernel_compile_stats,
                    kernel_partition_mappings,
                ) = ast.write_opencl_file(
                    hcall_size.try_into().unwrap(),
                    interleave,
                    stack_size,
                    heap_size,
                    call_stack_size,
                    stack_frames_size,
                    sfp_size,
                    predictor_size,
                    max_part,
                    max_loc,
                    max_dup,
                    maxdemospace,
                    local_work_group,
                    mexec,
                    disable_fastcalls,
                    debug_call_print,
                    force_inline,
                    is_gpu,
                    false,
                );
                println!("Compiled: {} functions", num_compiled_funcs);
                println!("Entry point: {}", entry_point);
                println!("Globals buffer: {}", globals_buffer_size);
                println!("interleave: {}", interleave);

                (
                    InputProgram::Partitioned(
                        kernel_hashmap.clone(),
                        fastcall_header.clone(),
                        kernel_compile_stats.clone(),
                        kernel_partition_mappings.clone(),
                    ),
                    entry_point,
                    num_compiled_funcs,
                    globals_buffer_size,
                    interleave,
                )
            }
            ("wasm", true) => {
                let filedata_text = wasmprinter::print_file(file_path.clone()).unwrap();
                let pb = ParseBuffer::new(&filedata_text).unwrap();
                let pb_bpatch = ParseBuffer::new(&bpatch::PATCH_FILE).unwrap();
                let pb_debug = ParseBuffer::new(&filedata_text).unwrap();
                let mut ast = opencl_writer::OpenCLCWriter::new(&pb, &pb_bpatch, pinput);
                let mut ast_debug =
                    opencl_writer::OpenCLCWriter::new(&pb_debug, &pb_bpatch, pinput);
                let _result = ast.parse_file().unwrap();
                let _result_debug = ast_debug.parse_file().unwrap();

                // apply our compilation pass to the source WASM
                let (
                    _compiled_kernel,
                    fastcall_header,
                    entry_point,
                    globals_buffer_size,
                    num_compiled_funcs,
                    kernel_hashmap,
                    kernel_compile_stats,
                    kernel_partition_mappings,
                ) = ast.write_opencl_file(
                    hcall_size.try_into().unwrap(),
                    interleave,
                    stack_size,
                    heap_size,
                    call_stack_size,
                    stack_frames_size,
                    sfp_size,
                    predictor_size,
                    max_part,
                    max_loc,
                    max_dup,
                    maxdemospace,
                    local_work_group,
                    mexec,
                    disable_fastcalls,
                    debug_call_print,
                    force_inline,
                    is_gpu,
                    false,
                );
                println!("Compiled: {} functions", num_compiled_funcs);
                println!("Entry point: {}", entry_point);
                println!("Globals buffer: {}", globals_buffer_size);
                println!("interleave: {}", interleave);

                (
                    InputProgram::Partitioned(
                        kernel_hashmap.clone(),
                        fastcall_header.clone(),
                        kernel_compile_stats.clone(),
                        kernel_partition_mappings.clone(),
                    ),
                    entry_point,
                    num_compiled_funcs,
                    globals_buffer_size,
                    interleave,
                )
            }
            ("bin", _) => {
                // read the binary file as a Vec<u8>
                let filedata = match fs::read(file_path.clone()) {
                    Ok(text) => text,
                    Err(e) => panic_any(e),
                };

                let program: SeralizedProgram = bincode::deserialize(&filedata).unwrap();
                println!("Loaded program with entry point: {}, num_compiled_funcs: {}, globals_buffer_size: {}, interleave: {}", program.entry_point, program.num_compiled_funcs, program.globals_buffer_size, program.interleave);
                (
                    InputProgram::Binary(program.program_data),
                    program.entry_point,
                    program.num_compiled_funcs,
                    program.globals_buffer_size,
                    program.interleave,
                )
            }
            ("partbin", _) => {
                // read the binary file as a Vec<u8>
                let filedata = match fs::read(file_path.clone()) {
                    Ok(text) => text,
                    Err(e) => panic_any(e),
                };

                let program: PartitionedSeralizedProgram = bincode::deserialize(&filedata).unwrap();
                println!("Loaded partitioned program with entry point: {}, num_compiled_funcs: {}, globals_buffer_size: {}, interleave: {}", program.entry_point, program.num_compiled_funcs, program.globals_buffer_size, program.interleave);

                (
                    InputProgram::PartitionedBinary(
                        program.program_data,
                        program.partition_mapping,
                    ),
                    program.entry_point,
                    program.num_compiled_funcs,
                    program.globals_buffer_size,
                    program.interleave,
                )
            }
            // nvidia specific assembly code, prebuilt
            // this is a legacy stub from earlier testing, it still works though
            ("ptx", _) => {
                // read the binary file as a Vec<u8>
                let filedata = match fs::read(file_path.clone()) {
                    Ok(text) => text,
                    Err(e) => panic_any(e),
                };
                let entry = value_t!(matches.value_of("entry"), u32).unwrap_or_else(|e| e.exit());
                let numfuncs =
                    value_t!(matches.value_of("numfuncs"), u32).unwrap_or_else(|e| e.exit());
                let globals_buffer_size = value_t!(matches.value_of("globals-buffer-size"), u32)
                    .unwrap_or_else(|e| e.exit());

                println!("Loaded program with entry point: {}, num_compiled_funcs: {}, globals_buffer_size: {}, interleave: {}", entry, numfuncs, globals_buffer_size, interleave);
                (
                    InputProgram::Binary(filedata),
                    entry,
                    numfuncs,
                    globals_buffer_size,
                    interleave,
                )
            }
            _ => panic!("Unrecognized input filetype: {:?}", (extension, partition)),
        };
        let fname = &file_path.as_str();
        //let mut spawned_threads = Vec::new();

        // Create a single context for all vmgroups
        let platform_id = ocl::core::default_platform().unwrap();
        let device_type = if is_gpu {
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
        let temp_context =
            ocl::core::create_context(Some(&context_properties), &[device_id], None, None).unwrap();
        let context: &'static ocl::core::Context = Box::leak(Box::new(temp_context));

        let runner = opencl_runner::OpenCLRunner::new(
            num_vms,
            interleave,
            is_gpu,
            entry_point,
            file.clone(),
        );
        let (program, device_id) = runner.setup_kernel(
            context,
            device_id,
            fname,
            stack_size,
            heap_size,
            num_compiled_funcs,
            globals_buffer_size,
            compile_args.clone(),
            link_args.clone(),
        );

        rayon::ThreadPoolBuilder::new()
            .num_threads(num_cpus::get() * 2)
            .build_global()
            .unwrap();

        (0..num_vm_groups)
            .collect::<Vec<u32>>()
            .par_iter()
            .map(|idx| {
                // set up the device context
                /*
                let context_properties = ContextProperties::new().platform(platform_id);
                let temp_context = ocl::core::create_context(Some(&context_properties), &[device_id], None, None).unwrap();
                let context: &'static ocl::core::Context = Box::leak(Box::new(temp_context));

                // Compile the input program
                */
                //let runner = opencl_runner::OpenCLRunner::new(num_vms, interleave, is_gpu, entry_point, file.clone());
                //let (program, device_id) = runner.setup_kernel(context, device_id, fname, stack_size, heap_size, num_compiled_funcs, globals_buffer_size, compile_args.clone(), link_args.clone());

                let mut server_sender_vec = vec![];
                let mut vm_recv_vec = vec![];
                for _ in 0..num_vms.clone() {
                    let (sender, recv): (
                        tokio::sync::mpsc::Sender<VmRecvType>,
                        tokio::sync::mpsc::Receiver<VmRecvType>,
                    ) = mpsc::channel(16384);
                    server_sender_vec.push(AsyncMutex::new(sender));
                    vm_recv_vec.push(Mutex::new(recv));
                }

                let server_sender_vec_arc = Arc::new(server_sender_vec);
                let vm_recv_vec_arc = Arc::new(vm_recv_vec);

                let mut vm_sender_vec = vec![];
                let mut server_recv_vec = vec![];
                for _ in 0..num_vms.clone() {
                    let (sender, recv): (
                        tokio::sync::mpsc::Sender<VmSenderType>,
                        tokio::sync::mpsc::Receiver<VmSenderType>,
                    ) = mpsc::channel(16384);
                    vm_sender_vec.push(Mutex::new(sender));
                    server_recv_vec.push(AsyncMutex::new(recv));
                }

                let vm_sender_vec_arc = Arc::new(vm_sender_vec);
                let server_recv_vec_arc = Arc::new(server_recv_vec);

                // we don't need to join the server handle, this will be active as long as the runtime is
                let is_active = Arc::new(Mutex::new(false));

                if serverless {
                    println!(
                        "Starting server on: {}:{}/batch_submit",
                        batch_submit_ip.clone(),
                        (batch_submit_port + idx).to_string()
                    );

                    let batch_submit_ip_clone = batch_submit_ip.clone();
                    let port = (batch_submit_port + idx).to_string();
                    thread::spawn(move || {
                        BatchSubmitServer::start_server(
                            hcall_size,
                            fastreply,
                            is_active,
                            server_sender_vec_arc,
                            server_recv_vec_arc,
                            num_vms,
                            batch_submit_ip_clone.clone(),
                            port.clone(),
                        );
                    });
                }

                runner.clone().run(
                    context,
                    program.clone(),
                    device_id,
                    fname,
                    hcall_size,
                    stack_size,
                    heap_size,
                    call_stack_size,
                    stack_frames_size,
                    sfp_size,
                    num_compiled_funcs,
                    globals_buffer_size,
                    local_work_group,
                    mexec,
                    req_timeout,
                    vm_sender_vec_arc.clone(),
                    vm_recv_vec_arc.clone(),
                    compile_args.clone(),
                    link_args.clone(),
                    print_return,
                )
            })
            .for_each(|handler| {
                handler.join().unwrap();
            });
    } else {
        // If we are running the wasmtime runtime
        let num_threads = num_cpus::get();
        let wg = WaitGroup::new();
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads.try_into().unwrap())
            .build()
            .unwrap();

        let mut server_sender_vec = vec![];
        let mut vm_recv_vec = vec![];
        for _ in 0..num_threads {
            let (sender, recv): (
                tokio::sync::mpsc::Sender<VmRecvType>,
                tokio::sync::mpsc::Receiver<VmRecvType>,
            ) = mpsc::channel(1);
            server_sender_vec.push(AsyncMutex::new(sender));
            vm_recv_vec.push(Mutex::new(recv));
        }

        let server_sender_vec_arc = Arc::new(server_sender_vec);
        let vm_recv_vec_arc = Arc::new(vm_recv_vec);

        let mut vm_sender_vec = vec![];
        let mut server_recv_vec = vec![];
        for _ in 0..num_threads {
            let (sender, recv): (
                tokio::sync::mpsc::Sender<VmSenderType>,
                tokio::sync::mpsc::Receiver<VmSenderType>,
            ) = mpsc::channel(1);
            vm_sender_vec.push(Mutex::new(sender));
            server_recv_vec.push(AsyncMutex::new(recv));
        }

        let vm_sender_vec_arc = Arc::new(vm_sender_vec);
        let server_recv_vec_arc = Arc::new(server_recv_vec);
        let is_active = Arc::new(Mutex::new(true));

        // For each VM create a tracking context (contains sender/receiver pair for each VM)

        if serverless {
            println!(
                "Starting server on: {}:{}/batch_submit",
                batch_submit_ip.clone(),
                batch_submit_port.to_string()
            );
            thread_pool.spawn(move || {
                BatchSubmitServer::start_server(
                    hcall_size,
                    fastreply,
                    is_active,
                    server_sender_vec_arc,
                    server_recv_vec_arc,
                    num_threads.try_into().unwrap(),
                    batch_submit_ip,
                    batch_submit_port.to_string(),
                );
            });
        }

        for idx in 0..num_threads {
            println!("Starting Wasmtime VM: {:?}", idx);

            let extension = match Path::new(&file_path).extension() {
                Some(ext) => ext.to_str().unwrap(),
                None => "none",
            };

            let filedata = match extension {
                "wat" => fs::read_to_string(file_path.clone()).unwrap(),
                "wasm" => wasmprinter::print_file(file_path.clone()).unwrap(),
                _ => {
                    panic!("Unknown file type for input WASM")
                }
            };

            let vm_sender_mutex_clone = vm_sender_vec_arc.clone();
            let vm_recv_mutex_clone = vm_recv_vec_arc.clone();
            let wg = wg.clone();

            thread::spawn(move || {
                let wasmtime_runner = WasmtimeRunner::new(
                    idx,
                    vm_sender_mutex_clone.clone(),
                    vm_recv_mutex_clone.clone(),
                );
                let leaked_runner: &'static WasmtimeRunner = Box::leak(Box::new(wasmtime_runner));

                // run the WASM VM...
                match leaked_runner.run(filedata.clone(), hcall_size, heap_size / (1024 * 64)) {
                    Ok(()) => {
                        println!("Wasmtime VM: {:?} finished running!", idx);
                    }
                    Err(e) => {
                        println!(
                            "An error occured while running VM: {:?}, error: {:?}",
                            idx, e
                        );
                    }
                }
                drop(wg);
            });
        }
        wg.wait();
    }
}
