/*
 * This file contains the WASI-helper functions, used to marshall and unmarshall data for performing hypercalls
 * 
 * We pre-allocate a 16KiB buffer per-VM, which we use for sending data back and forth to avoid
 * excess reads in the VMM.
 * 
 * The buffer lets us coalesce reads/writes together,
 * 
 * 
 * Each hypercall has its own format for how it manages the buffer.
 * 
 */

use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;


pub fn emit_hypercall_helpers(writer: &opencl_writer::OpenCLCWriter, call_name: Option<&str>, debug: bool) -> String {
    let mut result = String::from("");
    match call_name {
        Some(name) => {
            match name {
                "fd_write" => result += &emit_fd_write_helpers(writer, debug),
                _ => (),
            }
        },
        _ => (),
    }
    result
}


/*
 * This emits the helpers that set up fd_write before exiting.
 * 
 * We copy the iovec array to the buffer, and then follow the iovec array with the buffers we are actually writing
 * 
 * We overwrite the buf pointers to point towards where we store the buffers in the hypercall_buffer
 * 
 * The format of the hypercall_buffer is: [stack_params][iovec_arr][all buffers to write]
 * 
 */
pub fn emit_fd_write_helpers(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let mut result = String::from("");

    if debug {
        result += &String::from("\nvoid fd_write_helper(uint *stack_u32, uint* heap_u32, uint *hypercall_buffer, ulong *sp, uint warp_idx) {\n");
    } else {
        result += &String::from("\nvoid fd_write_helper(global uint *stack_u32, global uint* heap_u32, global uint *hypercall_buffer, global ulong *sp, uint warp_idx) {\n");
    }

    // first, copy all of the iovecs over to the hypercall_buffer
    // the number of iovecs and the iovec array ptr is on the stack

    // sp - 3 is the iovec*, sp-2 is the number of iovecs, sp-4 is the fd, sp-1 is the buf_len to write back to

    /*
     * Copy the stack over to the first 16 bytes of the hypercall_buffer (4, 4 byte values)
     */
    result += &format!("\t{};\n",
                       "___private_memcpy((ulong)(stack_u32+*sp-4),
                               (ulong)(stack_u32),
                               (ulong)(hypercall_buffer),
                               (ulong)(hypercall_buffer),
                               16,
                               warp_idx)");

    result += &format!("\tuint iovec_offset = {};\n",
                       emit_read_u32("(ulong)(stack_u32+*sp-3)", "(ulong)(stack_u32)", "warp_idx"));
                       
    result += &format!("\tuint iovec_loop_ctr = {};\n",
                       emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));

    result += &format!("\tuint iovec_hypercall_offset = 16;\n");
    result += &format!("\tuint buf_ptr = 0;\n");
    result += &format!("\tuint buf_len = 0;\n");
    // the next location in the hypercall_buffer where we will write our buffer
    // it starts off pointing at the end of the iovec arr
    result += &format!("\tuint next_buffer_start = iovec_loop_ctr*8;\n");

    result += &format!("\t{}\n",
                       format!("for(uint idx = 0; idx < {}; idx ++) {{", "iovec_loop_ctr"));

    // copy the iovec to the hypercall_buffer
    result += &format!("\t{}\n",
                       &format!("buf_ptr = {};\n\tbuf_len = {};\n",
                       emit_read_u32("(ulong)(((char*)heap_u32)+iovec_offset)", "(ulong)(heap_u32)", "warp_idx"),
                       emit_read_u32("(ulong)(((char*)heap_u32)+iovec_offset+4)", "(ulong)(heap_u32)", "warp_idx")));

    // write the iovec to the hypercall_buffer
    result += &format!("\t{};\n",
                       emit_write_u32("(ulong)((char*)hypercall_buffer + iovec_hypercall_offset)",
                                      "(ulong)(hypercall_buffer)",
                                      "next_buffer_start",
                                      "warp_idx")); 
    result += &format!("\t{};\n",
                        emit_write_u32("(ulong)((char*)hypercall_buffer + iovec_hypercall_offset+4)",
                                        "(ulong)(hypercall_buffer)",
                                        "buf_len",
                                        "warp_idx"));
    
    // now that we've written the iovec, we can copy the buffer 
    // we can compute the end of the iovec array, but must keep track of the lengths of all of the previous buffers

    // copy the buffer to the location (heap -> hypercall_buffer)
    result += &format!("\t{};\n",
                       "___private_memcpy((ulong)((char*)heap_u32+buf_ptr),
                               (ulong)(heap_u32),
                               (ulong)((char*)hypercall_buffer+next_buffer_start+16),
                               (ulong)(hypercall_buffer),
                               buf_len,
                               warp_idx)");

    // update next_buffer_start
    result += &format!("\tnext_buffer_start += buf_len;\n");

    // each iovec is 8 bytes long
    result += &format!("\t{}\n",
                       &format!("iovec_offset += 8;"));

    result += &format!("\t{}\n",
                       &format!("iovec_hypercall_offset += 8;"));

    // end loop
    result += &format!("\t{}\n", "}");

    result += &String::from("}\n\n");

    result
}

pub fn emit_fd_write_call_helper(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{}\n",
              format!("fd_write_helper({}, {}, {}, {}, {});",
                        "stack_u32",
                        "heap_u32",
                        "hypercall_buffer",
                        "sp",
                        "warp_idx"))
}