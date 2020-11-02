#![feature(fn_traits)]

extern crate ocl;

mod opencl_writer;
mod opencl_runner;

use std::fs;
use wast::parser::{ParseBuffer};
use opencl_runner::VMMRuntimeStatus;
use ocl::core::CommandQueue;

fn main() {

    // TODO add Clap arg parsing here to get the WASM files from CLI
    // also consider supporting .wasm files as well?


    //let file = fs::read_to_string("examples/arithmetic/lt.wat");
    //let file = fs::read_to_string("examples/call/call64.wat");
    //let file = fs::read_to_string("examples/call/call32.wat");
    //let file = fs::read_to_string("examples/call/call_indirect.wat");
    //let file = fs::read_to_string("examples/branches/loop.wat");
    let file = fs::read_to_string("examples/wasi_examples/fd_write.wat");
    
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

    // per-VM parameters
    let stack_size = 1024 * 16;
    let heap_size = 1024 * 16;
    let call_stack_size = 1024;
    let stack_frames_size = 1024;
    let sfp_size = 1024;
    let predictor_size = 4096;

    match (result, result_debug) {
        (true, true) => {
            // apply our compilation pass to the source WASM 
            let (compiled_kernel, entry_point) = ast.write_opencl_file(0,
                                                                       stack_size,
                                                                       heap_size, 
                                                                       call_stack_size, 
                                                                       stack_frames_size, 
                                                                       sfp_size, 
                                                                       predictor_size, 
                                                                       false);

            let (compiled_debug_kernel, _) = ast_debug.write_opencl_file(0,
                                                                         stack_size,
                                                                         heap_size, 
                                                                         call_stack_size, 
                                                                         stack_frames_size, 
                                                                         sfp_size, 
                                                                         predictor_size, 
                                                                         true);
                                                                
            std::fs::write("test.c", compiled_debug_kernel).expect("Unable to write file");


            // 16KB stack/heap by default - TODO: change these values after done testing
            let runner = opencl_runner::OpenCLRunner::new(1024 * 16, false, true, entry_point, compiled_kernel);


            let (program, context, device_id) = runner.setup_kernel();

            // create the buffers
            let (new_runner, context) = runner.create_buffers(stack_size,
                                                                         heap_size, 
                                                                         call_stack_size, 
                                                                         stack_frames_size, 
                                                                         sfp_size, 
                                                                         predictor_size,
                                                                         context);

            let handler = std::thread::spawn(move || {
                // this function returns the channel that we will use to send it HTTP requests later


                // each vector VMM group gets its own command queue - in the future we may have 1 queue per [Large N] number of VMs
                let command_queue = ocl::core::create_command_queue(&context, &device_id, None).unwrap();

                // We purposefully leak the runner into a static object to deal with the lifetimes of the
                // hypercall dispatch thread pools, we will clean up the new_runner object if needed
                // These values really do last for the entire program, so it is fine to make them static
                let final_runner = Box::leak(Box::new(new_runner));
                let leaked_command_queue: &'static CommandQueue = Box::leak(Box::new(command_queue));

                let status = final_runner.run_vector_vms(stack_frames_size, program, context, device_id, &leaked_command_queue);
                // this line should never be reached, reaching it signifies that either
                // 1) The VMM has exited normally
                // 2) The VMM has exited prematurely due to a crash
                match status {
                    VMMRuntimeStatus::StatusUnknownError => panic!("Vector VMM has crashed!!!"),
                    VMMRuntimeStatus::StatusOkay => (),
                }

                // In the future if we want to make this dynamic, we need to cleanup the leaked objects

            });
            handler.join().unwrap();
        },
        (_, _) => panic!("Unable to parse wat file"),
    }
}
