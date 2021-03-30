use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

/*
 * <, >, = are relops which also pop 2 values and push one back on
 */

pub fn emit_i32_eq(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) == (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_ne(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) != (uint)({});\n", result_register, reg2, reg1)
}

// computes < with unsigned vals
pub fn emit_i32_lt_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) < (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_lt_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (ulong)({}) < (ulong)({});\n", result_register, reg2, reg1)
}

// signed version
pub fn emit_i32_lt_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) < (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_lt_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (long)({}) < (long)({});\n", result_register, reg2, reg1)
}

// computes < with unsigned vals
pub fn emit_i32_gt_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) > (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_gt_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (ulong)({}) > (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_gt_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (long)({}) > (long)({});\n", result_register, reg2, reg1)
}

// signed version
pub fn emit_i32_gt_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) > (int)({});\n", result_register, reg2, reg1)
}

// computes >= with unsigned vals
pub fn emit_i32_ge_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) >= (uint)({});\n", result_register, reg2, reg1)
}

// signed version
pub fn emit_i32_ge_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) >= (int)({});\n", result_register, reg2, reg1)
}

// computes >= with unsigned vals
pub fn emit_i32_le_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) <= (uint)({});\n", result_register, reg2, reg1)
}

// signed version
pub fn emit_i32_le_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) <= (int)({});\n", result_register, reg2, reg1)
}


// computes >= with unsigned vals
pub fn emit_i64_ge_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (ulong)({}) >= (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_le_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (ulong)({}) <= (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_le_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (long)({}) <= (long)({});\n", result_register, reg2, reg1)
}

// signed version
pub fn emit_i64_ge_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (long)({}) >= (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_f64_lt(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ({}) < ({});\n", result_register, reg2, reg1)
}

pub fn emit_f64_le(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ({}) <= ({});\n", result_register, reg2, reg1)
}