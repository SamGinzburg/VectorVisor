/*
 * This file contains the WASI-helper functions, used to marshall and unmarshall data for performing hypercalls
 *
 * We pre-allocate a hcall buffer per-VM, which we use for sending data back and forth to avoid
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
//use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::{emit_write_u32, emit_write_u32_aligned_checked};
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;
use crate::opencl_writer::trap::*;

use wasi_common::snapshots::preview_1::types::*;

pub fn emit_hypercall_helpers(
    writer: &opencl_writer::OpenCLCWriter,
    call_name: Option<&str>,
    debug: bool,
) -> String {
    let mut result = String::from("");
    match call_name {
        Some(name) => match name {
            "fd_write" => result += &emit_fd_write_helpers(writer, debug),
            _ => (),
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
pub fn emit_fd_write_helpers(_writer: &opencl_writer::OpenCLCWriter, _debug: bool) -> String {
    let mut result = String::from("");

    result += &String::from("\nvoid fd_write_helper(global uint *stack_u32, global uint* heap_u32, global uint *hypercall_buffer, global ulong *sp, uint warp_idx, uint thread_idx, uint read_idx, local uchar *scratch_space, uint fd_write_buf_len, uint iovec_count, uint iovec, uint fd, uint hcall_size) {\n");

    // first, copy all of the iovecs over to the hypercall_buffer
    // the number of iovecs and the iovec array ptr is on the stack

    // sp - 3 is the iovec*, sp-2 is the number of iovecs, sp-4 is the fd, sp-1 is the buf_len to write back to

    /*
     * Copy the stack over to the first 16 bytes of the hypercall_buffer (4, 4 byte values)
     */

    result += &format!("\t*({}) = fd;\n",
                       "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx))");

    result += &format!("\t*({}) = iovec;\n",
                       "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)+4)");

    result += &format!("\t*({}) = iovec_count;\n",
                       "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)+8)");

    result += &format!("\t*({}) = fd_write_buf_len;\n",
                       "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)+12)");

    result += &format!("\tuint iovec_offset = {};\n", "iovec");

    result += &format!("\tuint iovec_loop_ctr = {};\n", "iovec_count");

    result += &format!("\tuint iovec_hypercall_offset = 16;\n");
    result += &format!("\tuint buf_ptr = 0;\n");
    result += &format!("\tuint buf_len = 0;\n");
    // the next location in the hypercall_buffer where we will write our buffer
    // it starts off pointing at the end of the iovec arr
    result += &format!("\tuint next_buffer_start = iovec_loop_ctr*8;\n");

    result += &format!(
        "\t{}\n",
        format!("for(uint idx = 0; idx < {}; idx ++) {{", "iovec_loop_ctr")
    );

    // copy the iovec to the hypercall_buffer
    result += &format!(
        "\t{}\n",
        &format!(
            "buf_ptr = {};\n\tbuf_len = {};\n",
            emit_read_u32(
                "(ulong)(((global char*)heap_u32)+iovec_offset)",
                "(ulong)(heap_u32)",
                "warp_idx"
            ),
            emit_read_u32(
                "(ulong)(((global char*)heap_u32)+iovec_offset+4)",
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    );

    // write the iovec to the hypercall_buffer

    result += &format!("\t*({}) = next_buffer_start;\n",
                       "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)+iovec_hypercall_offset)");

    result += &format!("\t*({}) = buf_len;\n",
                       "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)+iovec_hypercall_offset+4)");

    result += &format!("\t___private_memcpy_gpu2cpu((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx, thread_idx, scratch_space);\n",
                       &format!("(global char *)heap_u32+buf_ptr"),
                       "heap_u32", // mem_start_src
                       "(ulong)((global char *)hypercall_buffer+(hcall_size*warp_idx)+next_buffer_start+16)", //dst, first 4 bytes are the len
                       "hypercall_buffer", // mem_start_dst
                       "buf_len"); // the length of the buffer


    // update next_buffer_start
    result += &format!("\tnext_buffer_start += buf_len;\n");

    // each iovec is 8 bytes long
    result += &format!("\t{}\n", &format!("iovec_offset += 8;"));

    result += &format!("\t{}\n", &format!("iovec_hypercall_offset += 8;"));

    // end loop
    result += &format!("\t{}\n", "}");

    result += &String::from("}\n\n");

    result
}

pub fn emit_fd_write_call_helper(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let buf_len = stack_ctx.vstack_peak(StackType::i32, 0);
    let iovec_count = stack_ctx.vstack_peak(StackType::i32, 1);
    let iovec_ptr = stack_ctx.vstack_peak(StackType::i32, 2);
    let fd = stack_ctx.vstack_peak(StackType::i32, 3);

    //uint buf_len, uint iovec_count, uint iovec, uint fd
    format!(
        "\t{}\n",
        format!(
            "fd_write_helper({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});",
            "stack_u32",
            "heap_u32",
            "hypercall_buffer",
            "sp",
            "warp_idx",
            "thread_idx",
            "read_idx",
            "scratch_space",
            buf_len,
            iovec_count,
            iovec_ptr,
            fd,
            "hcall_size",
        )
    )
}

pub fn emit_fd_prestat_get_helper(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let fd = stack_ctx.vstack_peak(StackType::i32, 1);
    /*
     * We only need to copy over the fd
     */

    ret_str += &format!("\t*({}) = {};\n",
                        "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx))",
                        &fd);

    ret_str
}

pub fn emit_fd_fdstat_get_helper(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let _fdstat_ptr = stack_ctx.vstack_peak(StackType::i32, 1);

    // copy the fd
    ret_str += &format!("\t*({}) = {};\n",
                        "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx))",
                        &fd);

    ret_str
}

pub fn emit_fd_prestat_dir_name_helper(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    /*
     * We need to copy over the fd, string length, and the string itself
     */

    let str_len = stack_ctx.vstack_peak(StackType::i32, 0);
    let fd = stack_ctx.vstack_peak(StackType::i32, 2);

    // fd
    ret_str += &format!("\t*({}) = {};\n",
                        "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx))",
                        &fd);

    // str len
    ret_str += &format!("\t*({}) = {};\n",
                        "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)+4)",
                        &str_len);

    ret_str
}

pub fn emit_fd_prestat_dir_name_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let str_len = stack_ctx.vstack_pop(StackType::i32);
    let str_ptr = stack_ctx.vstack_pop(StackType::i32);
    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let result_regsiter = stack_ctx.vstack_alloc(StackType::i32);

    // we need to copy back the directory name that we just read
    /*
    ret_str += &format!(
        "\t{};\n",
        format!(
            "___private_memcpy({}, {}, {}, {}, {}, {}, {})",
            "(ulong)(hypercall_buffer+2)",
            "(ulong)(hypercall_buffer)",
            format!("(ulong)(global uchar *)heap_u32+{}", str_ptr),
            "(ulong)(heap_u32)",
            str_len,
            "warp_idx",
            "read_idx"
        )
    );
    */

    ret_str += &format!("\t___private_memcpy_cpu2gpu((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx, thread_idx, scratch_space);\n",
                        "(ulong)((global char *)hypercall_buffer+(hcall_size*warp_idx)+4)", //dst, first 4 bytes are the len
                        "hypercall_buffer", // mem_start_dst
                        &format!("(global char *)heap_u32+{}", str_ptr),
                        "heap_u32", // mem_start_src
                        &str_len); // the length of the buffer


    // now return the error code
    ret_str += &format!("\t{} = {};\n", result_regsiter, "hcall_ret_val");

    ret_str
}

pub fn emit_path_unlink_file_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let _path_ptr = stack_ctx.vstack_peak(StackType::i32, 1);
    let _path_len = stack_ctx.vstack_peak(StackType::i32, 2);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_path_unlink_file_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let _path_ptr = stack_ctx.vstack_pop(StackType::i32);
    let _path_len = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_path_symlink_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _old_path_ptr = stack_ctx.vstack_peak(StackType::i32, 0);
    let _old_path_len = stack_ctx.vstack_peak(StackType::i32, 1);
    let _fd = stack_ctx.vstack_peak(StackType::i32, 2);
    let _new_path_ptr = stack_ctx.vstack_peak(StackType::i32, 3);
    let _new_path_len = stack_ctx.vstack_peak(StackType::i32, 4);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_path_symlink_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _old_path_ptr = stack_ctx.vstack_pop(StackType::i32);
    let _old_path_len = stack_ctx.vstack_pop(StackType::i32);
    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let _new_path_ptr = stack_ctx.vstack_pop(StackType::i32);
    let _new_path_len = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_path_rename_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let _old_path_ptr = stack_ctx.vstack_peak(StackType::i32, 1);
    let _old_path_len = stack_ctx.vstack_peak(StackType::i32, 2);
    let _new_fd = stack_ctx.vstack_peak(StackType::i32, 3);
    let _new_path_ptr = stack_ctx.vstack_peak(StackType::i32, 4);
    let _new_path_len = stack_ctx.vstack_peak(StackType::i32, 5);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_path_rename_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let _old_path_ptr = stack_ctx.vstack_pop(StackType::i32);
    let _old_path_len = stack_ctx.vstack_pop(StackType::i32);
    let _new_fd = stack_ctx.vstack_pop(StackType::i32);
    let _new_path_ptr = stack_ctx.vstack_pop(StackType::i32);
    let _new_path_len = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_path_remove_directory_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let _path_ptr = stack_ctx.vstack_peak(StackType::i32, 1);
    let _path_len = stack_ctx.vstack_peak(StackType::i32, 2);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_path_remove_directory_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let _path_ptr = stack_ctx.vstack_pop(StackType::i32);
    let _path_len = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_path_create_directory_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let _path_ptr = stack_ctx.vstack_peak(StackType::i32, 1);
    let _path_len = stack_ctx.vstack_peak(StackType::i32, 2);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_path_create_directory_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let _path_ptr = stack_ctx.vstack_pop(StackType::i32);
    let _path_len = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}


pub fn emit_fd_readdir_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let _buf = stack_ctx.vstack_peak(StackType::i32, 1);
    let _buflen = stack_ctx.vstack_peak(StackType::i32, 2);
    let _cookie = stack_ctx.vstack_peak(StackType::i64, 0);
    let _return_ptr = stack_ctx.vstack_peak(StackType::i32, 3);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_fd_readdir_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let _buf = stack_ctx.vstack_pop(StackType::i32);
    let _buflen = stack_ctx.vstack_pop(StackType::i32);
    let _cookie = stack_ctx.vstack_pop(StackType::i64);
    let _return_ptr = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");
    ret_str
}

pub fn emit_fd_fdstat_set_flags_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let _flags = stack_ctx.vstack_peak(StackType::i32, 1);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_fd_fdstat_set_flags_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let _flags = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");
    ret_str
}

pub fn emit_fd_fdstat_get_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let fdstat_ptr = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // copy the fd_fdstat_get result back to GPU mem
    ret_str += &format!("\t___private_memcpy_cpu2gpu((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx, thread_idx, scratch_space);\n",
    "(ulong)((global char *)hypercall_buffer+(hcall_size*warp_idx))",
    "hypercall_buffer", // mem_start_dst
    &format!("(global char *)heap_u32+{}", fdstat_ptr),
    "heap_u32", // mem_start_src
    std::mem::size_of::<Fdstat>());

    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_fd_prestat_get_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    /*
     * We need to copy back the (i32) size of the string describing the fd name
     */
    let str_len = &format!("\t*({})\n",
                           "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx))");

    let offset = stack_ctx.vstack_pop(StackType::i32);
    stack_ctx.vstack_pop(StackType::i32); // fd

    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    ret_str += &format!(
        "\t{};\n",
        emit_write_u32(
            &format!("(ulong)((global char*)heap_u32+{}+4)", offset),
            "(ulong)(heap_u32)",
            str_len,
            "warp_idx"
        )
    );

    // now return the error code

    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_environ_sizes_get_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    // This function takes two u32 arguments, so we need to pop those off
    // arg1: offset for size, arg2: offset for arg string data size
    // We also need to copy back the two results from the hcall buf
    // offset 0 in the hcall buf is the number of arguments

    let size_string_buf = stack_ctx.vstack_pop(StackType::i32);
    let size_ptr_buf = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    ret_str += &format!(
        "\t{};\n",
        emit_write_u32(
            &format!("(ulong)((global char*)heap_u32+{})", &size_ptr_buf),
            "(ulong)(heap_u32)",
            "*((global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)))",
            "warp_idx"
        )
    );

    // offset 4 in the hcall buf is the size of the argument string data
    ret_str += &format!(
        "\t{};\n",
        emit_write_u32(
            &format!("(ulong)((global char*)heap_u32+{})", &size_string_buf),
            "(ulong)(heap_u32)",
            "*((global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)+4))",
            "warp_idx"
        )
    );

    // now return the error code
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_environ_get_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    // This function takes two u32 arguments, so we need to pop those off
    // arg1: pointer to a buffer of pointers
    // arg2: pointer to a buffer to store the string data
    // when we return, the hcall_buffer will include the two buf_lens as the first two 4 bytes values

    let env_count = format!("*((global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)))");

    let size_string_buf = stack_ctx.vstack_pop(StackType::i32);
    let size_ptr_buf = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // copy over the buffer of pointers

    ret_str += &format!("\t___private_memcpy_cpu2gpu((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx, thread_idx, scratch_space);\n",
                        "(ulong)((global char *)hypercall_buffer+(hcall_size*warp_idx)+8)",
                        "hypercall_buffer", // mem_start_dst
                        &format!("(global char *)heap_u32+{}", size_ptr_buf),
                        "heap_u32", // mem_start_src
                        &"*((global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)))");

    /*
    ret_str += &format!("\t___private_memcpy((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx);\n",
                        "(ulong)((global char *)hypercall_buffer+8)",
                        "hypercall_buffer", // mem_start_src
                        &format!("(global char *)heap_u32+{}", size_ptr_buf), //dst
                        "heap_u32", // mem_start_dst`
                        &emit_read_u32("(ulong)(hypercall_buffer)", "(ulong)(hypercall_buffer)", "warp_idx")); // buf_len_bytes;
    */

    // copy the string data
    ret_str += &format!("\t___private_memcpy_cpu2gpu((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx, thread_idx, scratch_space);\n",
                        &format!("(ulong)((global char *)hypercall_buffer+(hcall_size*warp_idx)+8+({}*4))", env_count),
                        "hypercall_buffer", // mem_start_dst
                        &format!("(global char *)heap_u32+{}", size_ptr_buf),
                        "heap_u32", // mem_start_src
                        &"*((global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)+4))");

    /*
    ret_str += &format!("\t___private_memcpy((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx);\n",
                        &format!("(ulong)((global char *)hypercall_buffer+8+({}*4))", env_count), //src
                        "hypercall_buffer", // mem_start_src
                        &format!("((global char *)heap_u32+{})", size_string_buf), //dst
                        "heap_u32", // mem_start_dst
                        &emit_read_u32("(ulong)(hypercall_buffer+1)", "(ulong)(hypercall_buffer)", "warp_idx")); // buf_len_bytes;
    */

    // now return the error code
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_fd_write_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let buf_len = stack_ctx.vstack_pop(StackType::i32);
    let _iovec_count = stack_ctx.vstack_pop(StackType::i32);
    let _iovec_ptr = stack_ctx.vstack_pop(StackType::i32);
    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // fd_write takes 4 u32 parameters
    // we just assume that we always succeed
    ret_str += &format!("\t{} = {};\n", result_register, "0");

    // write back nwritten as well
    ret_str += &format!(
        "\t{};\n",
        emit_write_u32(
            &format!("(ulong)((global char*)heap_u32+(int)({}))", buf_len),
            "(ulong)(heap_u32)",
            "hcall_ret_val",
            "warp_idx"
        )
    );

    ret_str
}

pub fn emit_serverless_invoke_pre(_writer: &opencl_writer::OpenCLCWriter, _debug: bool) -> String {
    let ret_str = String::from("");

    ret_str
}

pub fn emit_serverless_invoke_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let _json_buf_len = stack_ctx.vstack_pop(StackType::i32);
    let json_buf_ptr = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    //ret_str += &format!("\tprintf(\"json_buf_ptr: %p\\n\", {});\n", json_buf_ptr);
    //ret_str += &format!("\tprintf(\"json_buf_len: %p\\n\", {});\n", json_buf_len);
    //ret_str += &format!("\tprintf(\"hcall_ret_val: %p\\n\", {});\n", "hcall_ret_val");

    // we need to copy the data stored in the hcall buffer, to the json_buf_ptr
    // specifically, we need to de-interleave the data, so the CPU sees the data `normally`
    ret_str += &format!("\t___private_memcpy_cpu2gpu((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx, thread_idx, scratch_space);\n",
                        "(ulong)((global char *)hypercall_buffer + (hcall_size*warp_idx))", // src
                        "hypercall_buffer", // mem_start_src
                        &format!("(global char *)heap_u32+{}", json_buf_ptr), //dst
                        "heap_u32", // mem_start_dst
                        "hcall_ret_val"); // hcall_ret_val is the number of bytes read;

    // this function returns the length of the message in bytes
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_serverless_response_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let json_buf_len = stack_ctx.vstack_peak(StackType::i32, 0);
    let json_buf_ptr = stack_ctx.vstack_peak(StackType::i32, 1);

    // copy the buffer to the hcall buf so we can return it back via our middleware setup
    ret_str += &format!("\t___private_memcpy_gpu2cpu((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx, thread_idx, scratch_space);\n",
                        &format!("(global char *)heap_u32+{}", json_buf_ptr),
                        "heap_u32", // mem_start_src
                        "(ulong)((global char *)hypercall_buffer+(hcall_size*warp_idx)+4)", //dst, first 4 bytes are the len
                        "hypercall_buffer", // mem_start_dst
                        &json_buf_len); // the length of the buffer

    ret_str += &format!(
        "\t*({}) = {};\n",
        "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx))", &json_buf_len
    );

    ret_str
}

pub fn emit_serverless_response_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let ret_str = String::from("");

    let _json_buf_len = stack_ctx.vstack_pop(StackType::i32);
    let _json_buf_ptr = stack_ctx.vstack_pop(StackType::i32);

    ret_str
}

pub fn emit_poll_oneoff_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let subscription_ptr = stack_ctx.vstack_peak(StackType::i32, 0);
    let _events_ptr = stack_ctx.vstack_peak(StackType::i32, 1);
    let subscription_num = stack_ctx.vstack_peak(StackType::i32, 2);
    let _result_ptr = stack_ctx.vstack_peak(StackType::i32, 3);

    // copy the subscription_num
    ret_str += &format!("\t*({}) = {};\n",
                        "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx))", &subscription_num);
    // copy the subscriptions
    ret_str += &format!("\t___private_memcpy_gpu2cpu((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx, thread_idx, scratch_space);\n",
                        &format!("(global char *)heap_u32+{}", subscription_ptr),
                        "heap_u32", // mem_start_src
                        "(ulong)((global char *)hypercall_buffer+(hcall_size*warp_idx)+next_buffer_start+4)", //dst, first 4 bytes are the len
                        "hypercall_buffer", // mem_start_dst
                        format!("{} * {}", std::mem::size_of::<Subscription>(), subscription_num)); // the length of the buffer

    ret_str
}

pub fn emit_poll_oneoff_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let subscription_ptr = stack_ctx.vstack_pop(StackType::i32);
    let _events_ptr = stack_ctx.vstack_pop(StackType::i32);
    let subscription_num = stack_ctx.vstack_pop(StackType::i32);
    let result_ptr = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // copy the subscriptions
    ret_str += &format!("\t___private_memcpy_gpu2cpu((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx, thread_idx, scratch_space);\n",
                        &format!("(global char *)heap_u32+{}", subscription_ptr),
                        "heap_u32", // mem_start_src
                        "(ulong)((global char *)hypercall_buffer+(hcall_size*warp_idx)+next_buffer_start+4)", //dst, first 4 bytes are the len
                        "hypercall_buffer", // mem_start_dst
                        format!("{} * {}", std::mem::size_of::<Subscription>(), subscription_num)); // the length of the buffer

    // copy the nwritten result
    let nwritten = &format!("\t*({})\n",
                            "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx))");

    // write back result
    ret_str += &format!("\t{};\n", emit_write_u32_aligned_checked(&format!("(ulong)((global char*)heap_u32+(int)({}))", result_ptr),
                                                                  "(ulong)(heap_u32)",
                                                                  nwritten,
                                                                  "warp_idx"));

    // error code
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_path_open_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let dirflags = stack_ctx.vstack_peak(StackType::i32, 1);
    let path_ptr = stack_ctx.vstack_peak(StackType::i32, 2);
    let path_len = stack_ctx.vstack_peak(StackType::i32, 3);
    let oflags = stack_ctx.vstack_peak(StackType::i32, 4);
    let fdflags = stack_ctx.vstack_peak(StackType::i32, 5);
    let return_ptr = stack_ctx.vstack_peak(StackType::i32, 6);

    let fs_rights_base = stack_ctx.vstack_peak(StackType::i64, 0);
    let fs_rights_inherit = stack_ctx.vstack_peak(StackType::i64, 1);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_path_open_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let fd = stack_ctx.vstack_pop(StackType::i32);
    let dirflags = stack_ctx.vstack_pop(StackType::i32);
    let path_ptr = stack_ctx.vstack_pop(StackType::i32);
    let path_len = stack_ctx.vstack_pop(StackType::i32);
    let oflags = stack_ctx.vstack_pop(StackType::i32);
    let fdflags = stack_ctx.vstack_pop(StackType::i32);
    let return_ptr = stack_ctx.vstack_pop(StackType::i32);

    let fs_rights_base = stack_ctx.vstack_pop(StackType::i64);
    let fs_rights_inherit = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_fd_close_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let fd = stack_ctx.vstack_peak(StackType::i32, 0);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_fd_close_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let fd = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_fd_read_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let iovec_ptr = stack_ctx.vstack_peak(StackType::i32, 0);
    let iovec_len = stack_ctx.vstack_peak(StackType::i32, 0);
    let result_ptr = stack_ctx.vstack_peak(StackType::i32, 0);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_fd_read_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let fd = stack_ctx.vstack_pop(StackType::i32);
    let iovec_ptr = stack_ctx.vstack_pop(StackType::i32);
    let iovec_len = stack_ctx.vstack_pop(StackType::i32);
    let result_ptr = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}


pub fn emit_fd_filestat_get_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let _fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let _result_ptr = stack_ctx.vstack_peak(StackType::i32, 1);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_fd_filestat_get_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let _fd = stack_ctx.vstack_pop(StackType::i32);
    let _result_ptr = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");
    ret_str
}

pub fn emit_fd_seek_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let fd = stack_ctx.vstack_peak(StackType::i32, 0);
    let whence = stack_ctx.vstack_peak(StackType::i32, 1);
    let result_ptr = stack_ctx.vstack_peak(StackType::i32, 2);
    let offset = stack_ctx.vstack_peak(StackType::i64, 0);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str
}

pub fn emit_fd_seek_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let fd = stack_ctx.vstack_pop(StackType::i32);
    let whence = stack_ctx.vstack_pop(StackType::i32);
    let result_ptr = stack_ctx.vstack_pop(StackType::i32);
    let offset = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // TODO
    ret_str += &emit_trap(TrapCode::TrapUnimplemented, true);

    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");
    ret_str
}

pub fn emit_random_get_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let random_buf_len = stack_ctx.vstack_peak(StackType::i32, 0);

    // copy the buf len over so we know how many random bytes to generate
    ret_str += &format!("\t*({}) = {};\n",
                        "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx))", &random_buf_len);

    ret_str
}

pub fn emit_random_get_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let random_buf_len = stack_ctx.vstack_pop(StackType::i32);
    let random_buf_ptr = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    // Copy the random bytes back from the hcall_buf to the heap
    ret_str += &format!("\t___private_memcpy_cpu2gpu((ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), (ulong)({}), warp_idx, read_idx, thread_idx, scratch_space);\n",
                        "(ulong)((global char *)hypercall_buffer + (hcall_size*warp_idx))", // src
                        "hypercall_buffer", // mem_start_src
                        &format!("(global char *)heap_u32+{}", &random_buf_ptr), //dst
                        "heap_u32", // mem_start_dst
                        &random_buf_len); // number of bytes read;

    // return the error code associated with random_get
    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_clock_time_get_pre(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let _offset = stack_ctx.vstack_peak(StackType::i32, 0);
    let clock_id = stack_ctx.vstack_peak(StackType::i32, 1);
    let precision = stack_ctx.vstack_pop(StackType::i64);
    ret_str += &format!("\t*({}) = {};\n",
                        "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx))", &clock_id);

    ret_str += &format!("\t*({}) = {};\n",
                        "(global uint*)((global char*)hypercall_buffer+(hcall_size*warp_idx)+4)", &precision);

    ret_str
}

pub fn emit_clock_time_get_post(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let offset = stack_ctx.vstack_pop(StackType::i32);
    let _clock_id = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    
    let timestamp = &format!("\t*({})\n",
                             "(global ulong*)((global char*)hypercall_buffer+(hcall_size*warp_idx))");

    ret_str += &format!(
        "\t{};\n",
        emit_write_u64(
            &format!("(ulong)((global char*)heap_u32+(int)({}))", offset),
            "(ulong)(heap_u32)",
            &timestamp,
            "warp_idx"
        )
    );

    ret_str += &format!("\t{} = {};\n", result_register, "hcall_ret_val");

    ret_str
}

pub fn emit_sched_yield(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    ret_str += &format!("\t{} = {};\n", result_register, "0");

    ret_str
}
