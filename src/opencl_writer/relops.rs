use crate::opencl_writer;

/*
    * <, >, = are relops which also pop 2 values and push one back on
    */
pub fn emit_i32_lt_s(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{}\n",
            "write_u32((ulong)(stack_u32+*sp-1),
                        (ulong)stack_u32,
                        (int)read_u32((ulong)(stack_u32+*sp-1), (ulong)stack_u32, warp_idx) < (int)read_u32((ulong)(stack_u32+*sp-2), (ulong)stack_u32, warp_idx),
                        warp_idx);")
}

pub fn emit_i32_eq(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{}\n\t{}\n",
            "write_u32((ulong)(stack_u32+*sp-2),
                        (ulong)stack_u32,
                        (int)(read_u32((ulong)(stack_u32+*sp-1), (ulong)stack_u32, warp_idx)) == (int)(read_u32((ulong)(stack_u32+*sp-2), (ulong)stack_u32, warp_idx)),
                        warp_idx);",
            "*sp -= 1;")
}