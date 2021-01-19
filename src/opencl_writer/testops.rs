use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;

/*
 * Ops like eqz pop 1 value off the stack, and push 1 back on
 */

pub fn emit_i32_eqz(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let read_prev = &format!("((int)({}) == (int)0) ? 1 : 0", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    format!("\t{};\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-1)",
                            "(ulong)(stack_u32)",
                            read_prev,
                            "warp_idx"))
}

pub fn emit_i64_eqz(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let read_prev = &format!("((long)({}) == (long)0) ? 1 : 0", emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            read_prev,
                            "warp_idx"),
            "*sp -= 1;")
}