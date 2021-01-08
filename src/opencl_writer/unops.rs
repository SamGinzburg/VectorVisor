use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;

// binops have both values popped off the stack
pub fn emit_i32_clz(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let read_val = format!("clz({})", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));

    let mut ret_str = String::from("");

    ret_str += &format!("\t{};\n",
                        emit_write_u32("(ulong)(stack_u32+*sp-1)",
                                       "(ulong)(stack_u32)",
                                       &read_val,
                                       "warp_idx"));

    ret_str
}

pub fn emit_i64_clz(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let read_val = format!("clz({})", emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
    let mut ret_str = String::from("");

    ret_str += &format!("\t{};\n",
                        emit_write_u64("(ulong)(stack_u32+*sp-2)",
                                       "(ulong)(stack_u32)",
                                       &read_val,
                                       "warp_idx"));

    ret_str
}

pub fn emit_i32_ctz(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let read_val = format!("31-clz({}&-{})",
                emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"),
                emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));

    let mut ret_str = String::from("");

    ret_str += &format!("\t{};\n",
                        emit_write_u32("(ulong)(stack_u32+*sp-1)",
                                       "(ulong)(stack_u32)",
                                       &read_val,
                                       "warp_idx"));

    ret_str
}

pub fn emit_i64_ctz(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let read_val = format!("63-clz({}&-{})",
                emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
    let mut ret_str = String::from("");

    ret_str += &format!("\t{};\n",
                        emit_write_u64("(ulong)(stack_u32+*sp-2)",
                                       "(ulong)(stack_u32)",
                                       &read_val,
                                       "warp_idx"));

    ret_str
}