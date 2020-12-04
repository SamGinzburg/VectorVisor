extern crate ocl;

mod opencl_writer;
mod opencl_runner;

use std::fs;
use std::path::Path;

use wast::parser::{ParseBuffer};
use rayon::prelude::*;

use clap::{Arg, App, value_t};
use opencl_runner::InputProgram;
use opencl_runner::SeralizedProgram;

fn main() {

    // TODO add Clap arg parsing here to get the WASM files from CLI
    // also consider supporting .wasm files as well?

    let matches = App::new("wasm-vectorvmm")
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
            .default_value("16") // default of 16
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
        .arg(Arg::with_name("isinterleaved")
            .short("l")
            .long("isinterleaved")
            .help("If this flag is set to true, the compiler will interleave all reads/writes such that they are properly coalesced")
            .default_value("true") // default to true
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
        .arg(Arg::with_name("ldflags")
            .long("ldflags")
            .value_name("CLI arguments to pass to link_program")
            .default_value("")
            .help("Pass args to the program linking step")
            .multiple(false)
            .number_of_values(1)
            .takes_value(true))
        .get_matches();

    dbg!(matches.clone());

    let file_path = value_t!(matches.value_of("input"), String).unwrap_or_else(|e| e.exit());
    let interleaved = value_t!(matches.value_of("isinterleaved"), bool).unwrap_or_else(|e| e.exit());
    let stack_size = value_t!(matches.value_of("stack"), u32).unwrap_or_else(|e| e.exit());
    let heap_size = value_t!(matches.value_of("heap"), u32).unwrap_or_else(|e| e.exit());
    let call_stack_size = value_t!(matches.value_of("callstack"), u32).unwrap_or_else(|e| e.exit());
    let stack_frames_size = call_stack_size;
    let sfp_size = call_stack_size;
    let predictor_size = 4096;
    let num_vms = value_t!(matches.value_of("vmcount"), u32).unwrap_or_else(|e| e.exit());
    let num_vm_groups = value_t!(matches.value_of("vmgroups"), u32).unwrap_or_else(|e| e.exit());
    let is_gpu = value_t!(matches.value_of("isgpu"), bool).unwrap_or_else(|e| e.exit());
    let debug_call_print = value_t!(matches.value_of("debugcallprint"), bool).unwrap_or_else(|e| e.exit());
    let compile_args = value_t!(matches.value_of("cflags"), String).unwrap_or_else(|e| e.exit());
    let link_args = value_t!(matches.value_of("ldflags"), String).unwrap_or_else(|e| e.exit());
    dbg!(compile_args.clone());

    let extension = match Path::new(&file_path).extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => "none",
    };

    dbg!(extension);

    let (file, entry_point, num_compiled_funcs, globals_buffer_size, interleaved) = match extension {
        "wat" => {
            let filedata = match fs::read_to_string(file_path.clone()) {
                Ok(text) => text,
                Err(e) => panic!(e),
            };
            let pb = ParseBuffer::new(&filedata).unwrap();
            let pb_debug = ParseBuffer::new(&filedata).unwrap();
            let mut ast = opencl_writer::OpenCLCWriter::new(&pb);
            let mut ast_debug = opencl_writer::OpenCLCWriter::new(&pb_debug);
            let result = ast.parse_file().unwrap();
            let result_debug = ast_debug.parse_file().unwrap();
        
            // apply our compilation pass to the source WASM 
            let (compiled_kernel, entry_point, globals_buffer_size, num_compiled_funcs) = ast.write_opencl_file(interleaved as u32,
                                                                                                                            stack_size,
                                                                                                                            heap_size, 
                                                                                                                            call_stack_size, 
                                                                                                                            stack_frames_size, 
                                                                                                                            sfp_size, 
                                                                                                                            predictor_size,
                                                                                                                            debug_call_print,
                                                                                                                            false);
            println!("Compiled: {} functions", num_compiled_funcs);
            (InputProgram::text(compiled_kernel.clone()), entry_point, num_compiled_funcs, globals_buffer_size, interleaved)
        },
        "wasm" => {
            panic!(".wasm files not supported yet")
        },
        "bin" => {
            // read the binary file as a Vec<u8>
            let filedata = match fs::read(file_path.clone()) {
                Ok(text) => text,
                Err(e) => panic!(e),
            };

            let program: SeralizedProgram = bincode::deserialize(&filedata).unwrap();
            println!("Loaded program with entry point: {}, num_compiled_funcs: {}, globals_buffer_size: {}, is_interleaved: {}", program.entry_point, program.num_compiled_funcs, program.globals_buffer_size, program.interleaved);
            (InputProgram::binary(program.program_data), program.entry_point, program.num_compiled_funcs, program.globals_buffer_size, program.interleaved)
        },
        _ => panic!("Unrecognized input filetype: {}", extension),
    };

    let fname = &file_path.as_str();
    (0..num_vm_groups).into_par_iter().for_each(|_idx| {
        let runner = opencl_runner::OpenCLRunner::new(num_vms, interleaved, is_gpu, entry_point, file.clone());
        runner.run(fname,
                   stack_size,
                   heap_size, 
                   call_stack_size, 
                   stack_frames_size, 
                   sfp_size, 
                   num_compiled_funcs,
                   globals_buffer_size,
                   compile_args.clone(),
                   link_args.clone());
    });
}
 