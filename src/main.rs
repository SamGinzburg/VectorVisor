#![feature(get_mut_unchecked)]
extern crate ocl;

mod opencl_writer;
mod opencl_runner;

use std::fs;
use wast::parser::{ParseBuffer};
use opencl_runner::VMMRuntimeStatus;
use ocl::core::CommandQueue;
use rayon::prelude::*;

fn main() {

    // TODO add Clap arg parsing here to get the WASM files from CLI
    // also consider supporting .wasm files as well?


    //let file = fs::read_to_string("examples/binops/lt.wat");
    //let file = fs::read_to_string("examples/binops/sub.wat");
    //let file = fs::read_to_string("examples/call/call64.wat");
    //let file = fs::read_to_string("examples/call/call32.wat");
    //let file = fs::read_to_string("examples/call/call_indirect.wat");
    //let file = fs::read_to_string("examples/branches/loop.wat");
    //let file = fs::read_to_string("examples/wasi_examples/fd_write.wat");
    //let file = fs::read_to_string("examples/globals/simple_global.wat");
    //let file = fs::read_to_string("examples/globals/global_set.wat");
    let file = fs::read_to_string("examples/rust_hello.wat");


    let filedata = match file {
        Ok(text) => text,
        Err(e) => panic!(e),
    };

    let pb = ParseBuffer::new(&filedata).unwrap();
    // emitting a debug version of the program is optional...
    let pb_debug = ParseBuffer::new(&filedata).unwrap();

    let mut ast = opencl_writer::OpenCLCWriter::new(&pb);
    let mut ast_debug = opencl_writer::OpenCLCWriter::new(&pb_debug);

    let result = ast.parse_file().unwrap();
    let result_debug = ast_debug.parse_file().unwrap();

    // per-VM parameters (all in bytes)
    let stack_size = 1024 * 16;
    let heap_size = 1024 * 16;
    let call_stack_size = 1024;
    let stack_frames_size = 1024;
    let sfp_size = 1024;
    let predictor_size = 4096;
    let num_vms = 16;
    let interleaved = true;

    match (result, result_debug) {
        (true, true) => {
            // apply our compilation pass to the source WASM 
            let (compiled_kernel, entry_point, globals_buffer_size, num_compiled_funcs) = ast.write_opencl_file(interleaved as u32,
                                                                                                                stack_size,
                                                                                                                heap_size, 
                                                                                                                call_stack_size, 
                                                                                                                stack_frames_size, 
                                                                                                                sfp_size, 
                                                                                                                predictor_size, 
                                                                                                                false);
            println!("Compiled: {} functions", num_compiled_funcs);
            /*
            let (compiled_debug_kernel, _, _, _) = ast_debug.write_opencl_file(interleaved as u32,
                                                                               stack_size,
                                                                               heap_size, 
                                                                               call_stack_size, 
                                                                               stack_frames_size, 
                                                                               sfp_size, 
                                                                               predictor_size, 
                                                                               true);

            std::fs::write("test.c", compiled_debug_kernel).expect("Unable to write file");
            */

            // 16KB stack/heap by default - TODO: change these values after done testing
            (0..1).into_par_iter().for_each(|_idx| {
                let runner = opencl_runner::OpenCLRunner::new(num_vms, interleaved, true, entry_point, compiled_kernel.clone());
                runner.run(stack_size,
                           heap_size, 
                           call_stack_size, 
                           stack_frames_size, 
                           sfp_size, 
                           num_compiled_funcs,
                           globals_buffer_size);
            });
        },
        (_, _) => panic!("Unable to parse wat file"),
    }
}
 