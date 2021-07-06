use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

// binops have both values popped off the stack
pub fn emit_i32_clz(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i32, 0);

    format!("\t{} = clz({});\n", reg, reg)
}

pub fn emit_i64_clz(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i64, 0);

    format!("\t{} = clz({});\n", reg, reg)
}

pub fn emit_i32_ctz(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i32, 0);

    format!("\t{} = 31-clz({}&-{});\n", reg, reg, reg)
}

pub fn emit_i64_ctz(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i64, 0);

    format!("\t{} = 63-clz({}&-{});\n", reg, reg, reg)
}

pub fn emit_i32_popcnt(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i32, 0);

    format!("\t{} = popcount({});\n", reg, reg)
}

pub fn emit_f64_neg(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::f64, 0);

    format!("\t{} = -{};\n", reg, reg)
}

pub fn emit_f32_neg(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::f32, 0);

    format!("\t{} = -{};\n", reg, reg)
}

// we reply on the OpenCL built-in for these calls
pub fn emit_f64_ceil(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = ceil({});\n", result_register, reg)
}

pub fn emit_f32_ceil(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);
    format!("\t{} = ceil({});\n", result_register, reg)
}

pub fn emit_f64_floor(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = floor({});\n", result_register, reg)
}

pub fn emit_f32_floor(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);
    format!("\t{} = floor({});\n", result_register, reg)
}

pub fn emit_f64_trunc(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = trunc({});\n", result_register, reg)
}

pub fn emit_f32_trunc(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);
    format!("\t{} = trunc({});\n", result_register, reg)
}
