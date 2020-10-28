use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;

// binops have both values popped off the stack
pub fn emit_i32_add(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            &format!("(int){} + (int){}",
                                     &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 1;")
}

/*
    * addition is a binop - pops 2 values off the stack and pushes one back on
    */
pub fn emit_i64_add(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{}\n\t{}\n",
            "write_u64((ulong)(stack_u32+*sp-4), (ulong)(stack_u32),
                        (long)read_u64((ulong)(stack_u32+*sp-2), warp_idx) + (long)read_u64((ulong)(stack_u32+*sp-4), warp_idx),
                        warp_idx);",
            "*sp -= 2;")
}