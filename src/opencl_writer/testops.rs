use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

/*
 * Ops like eqz pop 1 value off the stack, and push 1 back on
 */

pub fn emit_i32_eqz(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg = stack_ctx.vstack_peak(StackType::i32, 0);
    format!("\t{} = ((int)({}) == (int)0) ? 1 : 0;\n", reg, reg)
}

pub fn emit_i64_eqz(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = ((long)({}) == (long)0) ? 1 : 0;\n",
        result_register, reg
    )
}
