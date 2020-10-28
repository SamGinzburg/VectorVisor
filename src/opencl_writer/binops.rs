use crate::opencl_writer;

// binops have both values popped off the stack
pub fn emit_i32_add(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{}\n\t{}\n",
            "write_u32((ulong)(stack_u32+*sp-2),
                        (int)read_u32((ulong)(stack_u32+*sp-1), warp_idx) + (int)read_u32((ulong)(stack_u32+*sp-2), warp_idx),
                        warp_idx);",
            "*sp -= 1;")
}

/*
    * addition is a binop - pops 2 values off the stack and pushes one back on
    */
pub fn emit_i64_add(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{}\n\t{}\n",
            "write_u64((ulong)(stack_u32+*sp-4),
                        (long)read_u64((ulong)(stack_u32+*sp-2), warp_idx) + (long)read_u64((ulong)(stack_u32+*sp-4), warp_idx),
                        warp_idx);",
            "*sp -= 2;")
}