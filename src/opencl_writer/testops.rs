use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

/*
 * Ops like eqz pop 1 value off the stack, and push 1 back on
 */

pub fn emit_i32_eqz(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i32);
    format!("\t{} = ((int)({}) == (int)0) ? 1 : 0;\n", reg, reg)
}

pub fn emit_i64_eqz(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ((int)({}) == (int)0) ? 1 : 0;\n", result_register, reg)
}