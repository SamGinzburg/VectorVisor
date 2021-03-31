use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

/*
 * This file contains conversion operators
 */


pub fn emit_i32_wrap_i64(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    format!("\t{} = (int)({});\n", result_register, reg)
}

pub fn emit_i64_extend_i32_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);
    format!("\t{} = (int)({});\n", result_register, reg)
}

pub fn emit_i64_extend_i32_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);
    format!("\t{} = (ulong)({});\n", result_register, reg)
}

pub fn emit_f64_convert_i32(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = (double)((int){});\n", result_register, reg)
}

pub fn emit_f64_convert_i32u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = (double)({});\n", result_register, reg)
}

pub fn emit_f64_convert_i64u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = (double)((ulong){});\n", result_register, reg)
}

pub fn emit_i32_trunc_f64_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    format!("\t{} = (uint)({});\n", result_register, reg)
}

pub fn emit_i64_reinterpret_f64(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);
    format!("\t{} = ({});\n", result_register, reg)
}

pub fn emit_f64_reinterpret_i64(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);
    format!("\t{} = ({});\n", result_register, reg)
}

pub fn emit_f32_reinterpret_i32(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);
    format!("\t{} = ({});\n", result_register, reg)
}

pub fn emit_i32_reinterpret_f32(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    format!("\t{} = ({});\n", result_register, reg)
}