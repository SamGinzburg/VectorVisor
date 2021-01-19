use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;

/*
 * <, >, = are relops which also pop 2 values and push one back on
 */

pub fn emit_i32_eq(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{}\n\t{}\n",
            "write_u32((ulong)(stack_u32+*sp-2),
                        (ulong)stack_u32,
                        (int)(read_u32((ulong)(stack_u32+*sp-1), (ulong)stack_u32, warp_idx)) == (int)(read_u32((ulong)(stack_u32+*sp-2), (ulong)stack_u32, warp_idx)),
                        warp_idx);",
            "*sp -= 1;")
}

pub fn emit_i32_ne(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            &format!("((int)({}) != (int)({})) ? 1 : 0",
                                     &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 1;")
}

// computes < with unsigned vals
pub fn emit_i32_lt_u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            &format!("((uint)({}) < (uint)({})) ? 1 : 0",
                                     &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 1;")
}

pub fn emit_i64_lt_u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            &format!("((ulong)({}) < (ulong)({})) ? 1 : 0",
                                     &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 3;")
}

// signed version
pub fn emit_i32_lt_s(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            &format!("((int)({}) < (int)({})) ? 1 : 0",
                                     &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 1;")
}

pub fn emit_i64_lt_s(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            &format!("((long)({}) < (long)({})) ? 1 : 0",
                                     &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 3;")
}

// computes < with unsigned vals
pub fn emit_i32_gt_u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            &format!("((uint)({}) > (uint)({})) ? 1 : 0",
                                     &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 1;")
}

pub fn emit_i64_gt_u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            &format!("((ulong)({}) > (ulong)({})) ? 1 : 0",
                                     &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 3;")
}

pub fn emit_i64_gt_s(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            &format!("((long)({}) > (long)({})) ? 1 : 0",
                                     &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 3;")
}

// signed version
pub fn emit_i32_gt_s(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            &format!("((int)({}) > (int)({})) ? 1 : 0",
                                     &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 1;")
}

// computes >= with unsigned vals
pub fn emit_i32_ge_u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            &format!("((uint)({}) >= (uint)({})) ? 1 : 0",
                                     &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 1;")
}

// signed version
pub fn emit_i32_ge_s(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            &format!("((int)({}) >= (int)({})) ? 1 : 0",
                                     &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 1;")
}

// computes >= with unsigned vals
pub fn emit_i32_le_u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            &format!("((uint)({}) <= (uint)({})) ? 1 : 0",
                                     &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 1;")
}

// signed version
pub fn emit_i32_le_s(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-2)",
                            "(ulong)(stack_u32)",
                            &format!("((int)({}) <= (int)({})) ? 1 : 0",
                                     &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 1;")
}


// computes >= with unsigned vals
pub fn emit_i64_ge_u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            &format!("((ulong)({}) >= (ulong)({})) ? 1 : 0",
                                     &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 3;")
}

pub fn emit_i64_le_u(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            &format!("((ulong)({}) <= (ulong)({})) ? 1 : 0",
                                     &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 3;")
}

pub fn emit_i64_le_s(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            &format!("((long)({}) <= (long)({})) ? 1 : 0",
                                     &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 3;")
}

// signed version
pub fn emit_i64_ge_s(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            &format!("((long)({}) >= (long)({})) ? 1 : 0",
                                     &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"),
                                     &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx")),
                            "warp_idx"),
            "*sp -= 3;")
}

pub fn emit_f64_lt(writer: &opencl_writer::OpenCLCWriter, debug: bool) -> String {
    let mut ret_str = String::from("");

    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\t{}\n", "double x;");
    ret_str += &format!("\t\t{}\n", "double y;");
    ret_str += &format!("\t\tulong x_old = {};\n", &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t\tulong y_old = {};\n", &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t\t{}\n", "memcpy(&x, &x_old, sizeof(double));");
    ret_str += &format!("\t\t{}\n", "memcpy(&y, &y_old, sizeof(double));");
    ret_str += &format!("\t{}\n", "x_old = x < y;");

    ret_str += &format!("\t{};\n\t{}\n",
            &emit_write_u32("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            "x_old",
                            "warp_idx"),
            "*sp -= 3;");

    ret_str += &format!("\t}}\n");

    ret_str
}