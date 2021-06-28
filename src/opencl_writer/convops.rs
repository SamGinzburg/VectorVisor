use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;
use crate::opencl_writer::trap::*;

/*
 * This file contains conversion operators
 */


pub fn emit_i32_wrap_i64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    format!("\t{} = (int)({});\n", result_register, reg)
}

pub fn emit_i64_extend_i32_s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);
    format!("\t{} = (int)({});\n", result_register, reg)
}

pub fn emit_i64_extend_i32_u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);
    format!("\t{} = (ulong)({});\n", result_register, reg)
}

pub fn emit_f64_convert_i32(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = (double)((int){});\n", result_register, reg)
}

pub fn emit_f64_convert_i32u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = (double)({});\n", result_register, reg)
}

pub fn emit_f32_convert_i32u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);
    format!("\t{} = (float)({});\n", result_register, reg)
}

pub fn emit_f64_convert_i64u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = (double)((ulong){});\n", result_register, reg)
}

pub fn emit_f64_convert_i64s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = (double)((long){});\n", result_register, reg)
}

pub fn emit_i32_trunc_f64_u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    format!("\t{} = (uint)({});\n", result_register, reg)
}

pub fn emit_i64_reinterpret_f64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t___private_memcpy_nonmmu(&{}, &{}, sizeof(double));\n", result_register, reg)
}

pub fn emit_f64_reinterpret_i64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);

    format!("\t___private_memcpy_nonmmu(&{}, &{}, sizeof(ulong));\n", result_register, reg)
}

pub fn emit_f32_reinterpret_i32(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);

    format!("\t___private_memcpy_nonmmu(&{}, &{}, sizeof(float));\n", result_register, reg)
}

pub fn emit_i32_reinterpret_f32(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t___private_memcpy_nonmmu(&{}, &{}, sizeof(uint));\n", result_register, reg)
}

pub fn emit_f64_promote_f32(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = (double)({});\n", result_register, reg)
}

pub fn emit_f32_demote_f64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);
    format!("\t{} = (float)({});\n", result_register, reg)
}


// x!=x is done to check for NaN, then the conversion range is checked
pub fn emit_i64_trunc_f32_u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{};\n",
            format!("({} != {}) ? ({}) : !({} > (double)-1 && {} < 4294967296.f) ? ({}) : ({} = (ulong)({}))",
                    reg, reg, emit_trap(TrapCode::TrapInvalidConversion, false),
                    reg, reg, emit_trap(TrapCode::TrapIntOverflow, false), result_register, reg))
}


pub fn emit_i64_trunc_f32_s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{};\n",
            format!("({} != {}) ? ({}) : !({} >= (float)LONG_MIN && {} < (float)LONG_MAX) ? ({}) : ({} = (ulong)(long)({}))",
                    reg, reg, emit_trap(TrapCode::TrapInvalidConversion, false),
                    reg, reg, emit_trap(TrapCode::TrapIntOverflow, false), result_register, reg))
}