use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;


/*
 * This file contains conversion operators
 */


pub fn emit_i32_wrap_i64(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let wrap = emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx");
    format!("\t{};\n\t{}\n",
            emit_write_u32("(ulong)(stack_u32+*sp-2)",
                           "(ulong)(stack_u32)",
                           &format!("(uint){}", wrap),
                           "warp_idx"),
            "*sp -= 1;")
}