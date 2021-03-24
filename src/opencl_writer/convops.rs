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
                           &format!("(int)({})", wrap),
                           "warp_idx"),
            "*sp -= 1;")
}


pub fn emit_i64_extend_i32_s(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let extend = emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx");
    format!("\t{};\n\t{}\n",
            emit_write_u64("(ulong)(stack_u32+*sp-1)",
                           "(ulong)(stack_u32)",
                           &format!("(int)({})", extend),
                           "warp_idx"),
            // the 64 bit value takes up an extra 4 bytes of space
            "*sp += 1;")
}

pub fn emit_i64_extend_i32_u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let extend = emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx");
    format!("\t{};\n\t{}\n",
            emit_write_u64("(ulong)(stack_u32+*sp-1)",
                           "(ulong)(stack_u32)",
                           &format!("(ulong)({})", extend),
                           "warp_idx"),
            // the 64 bit value takes up an extra 4 bytes of space
            "*sp += 1;")
}

pub fn emit_f64_convert_i32(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let extend = emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx");
    format!("\t{};\n\t{}\n",
            emit_write_u64("(ulong)(stack_u32+*sp-1)",
                           "(ulong)(stack_u32)",
                           &format!("(double)((int){})", extend),
                           "warp_idx"),
            // the 64 bit value takes up an extra 4 bytes of space
            "*sp += 1;")
}

pub fn emit_f64_convert_i32u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let extend = emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx");
    format!("\t{};\n\t{}\n",
            emit_write_u64("(ulong)(stack_u32+*sp-1)",
                           "(ulong)(stack_u32)",
                           &format!("(double)((uint){})", extend),
                           "warp_idx"),
            // the 64 bit value takes up an extra 4 bytes of space
            "*sp += 1;")
}

pub fn emit_f64_convert_i64u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let extend = emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx");
    format!("\t{};\n",
            emit_write_u64("(ulong)(stack_u32+*sp-2)",
                           "(ulong)(stack_u32)",
                           &format!("(double)((ulong){})", extend),
                           "warp_idx"))
}

pub fn emit_i32_trunc_f64_u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let mut ret_str = String::from("");
    let trunc = &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx");

    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\t{}\n", "double x;");
    // read the old f64
    ret_str += &format!("\t\tulong x_old = {};\n", trunc);
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&x, &x_old, sizeof(double));");


    ret_str += &format!("\t{}\n", format!("printf(\"ulong: %lu\\n\", {});", "x"));

    ret_str += &format!("\t{};\n",
        emit_write_u32("(ulong)(stack_u32+*sp-2)",
                    "(ulong)(stack_u32)",
                    &format!("x"),
                    "warp_idx"));

    ret_str += &format!("\t{}\n", "*sp -= 1;");

    ret_str += &format!("\t}}\n");


    ret_str
}
