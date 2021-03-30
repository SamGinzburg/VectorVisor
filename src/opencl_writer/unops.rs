use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

// binops have both values popped off the stack
pub fn emit_i32_clz(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i32);

    format!("\t{} = clz({});\n", reg, reg)
}

pub fn emit_i64_clz(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i64);

    format!("\t{} = clz({});\n", reg, reg)
}

pub fn emit_i32_ctz(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i32);

    format!("\t{} = 31-clz({}&-{});\n", reg, reg, reg)
}

pub fn emit_i64_ctz(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i64);

    format!("\t{} = 63-clz({}&-{});\n", reg, reg, reg)
}

pub fn emit_i32_popcnt(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i32);

    format!("\t{} = popcount({});\n", reg, reg)
}

pub fn emit_f64_neg(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::f64);

    format!("\t{} = -{};\n", reg, reg)
}
