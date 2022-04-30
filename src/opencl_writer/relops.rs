use crate::opencl_writer;

use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

/*
 * <, >, = are relops which also pop 2 values and push one back on
 */

pub fn emit_i32_eq(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (int)({}) == (int)({});\n",
        result_register, reg2, reg1
    )
}

pub fn emit_i32_ne(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (uint)({}) != (uint)({});\n",
        result_register, reg2, reg1
    )
}

// computes < with unsigned vals
pub fn emit_i32_lt_u(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (uint)({}) < (uint)({});\n",
        result_register, reg2, reg1
    )
}

pub fn emit_i64_lt_u(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (ulong)({}) < (ulong)({});\n",
        result_register, reg2, reg1
    )
}

// signed version
pub fn emit_i32_lt_s(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (int)({}) < (int)({});\n",
        result_register, reg2, reg1
    )
}

pub fn emit_i64_lt_s(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (long)({}) < (long)({});\n",
        result_register, reg2, reg1
    )
}

// computes < with unsigned vals
pub fn emit_i32_gt_u(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (uint)({}) > (uint)({});\n",
        result_register, reg2, reg1
    )
}

pub fn emit_i64_gt_u(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (ulong)({}) > (ulong)({});\n",
        result_register, reg2, reg1
    )
}

pub fn emit_i64_gt_s(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (long)({}) > (long)({});\n",
        result_register, reg2, reg1
    )
}

// signed version
pub fn emit_i32_gt_s(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (int)({}) > (int)({});\n",
        result_register, reg2, reg1
    )
}

// computes >= with unsigned vals
pub fn emit_i32_ge_u(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (uint)({}) >= (uint)({});\n",
        result_register, reg2, reg1
    )
}

// signed version
pub fn emit_i32_ge_s(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (int)({}) >= (int)({});\n",
        result_register, reg2, reg1
    )
}

// computes >= with unsigned vals
pub fn emit_i32_le_u(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (uint)({}) <= (uint)({});\n",
        result_register, reg2, reg1
    )
}

// signed version
pub fn emit_i32_le_s(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (int)({}) <= (int)({});\n",
        result_register, reg2, reg1
    )
}

// computes >= with unsigned vals
pub fn emit_i64_ge_u(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (ulong)({}) >= (ulong)({});\n",
        result_register, reg2, reg1
    )
}

pub fn emit_i64_le_u(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (ulong)({}) <= (ulong)({});\n",
        result_register, reg2, reg1
    )
}

pub fn emit_i64_le_s(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (long)({}) <= (long)({});\n",
        result_register, reg2, reg1
    )
}

// signed version
pub fn emit_i64_ge_s(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!(
        "\t{} = (long)({}) >= (long)({});\n",
        result_register, reg2, reg1
    )
}

pub fn emit_f64_lt(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ({}) < ({});\n", result_register, reg2, reg1)
}

pub fn emit_f32_gt(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ({}) > ({});\n", result_register, reg2, reg1)
}

pub fn emit_f32_lt(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ({}) < ({});\n", result_register, reg2, reg1)
}

pub fn emit_f32_le(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ({}) <= ({});\n", result_register, reg2, reg1)
}

pub fn emit_f32_ge(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ({}) >= ({});\n", result_register, reg2, reg1)
}

pub fn emit_f64_le(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ({}) <= ({});\n", result_register, reg2, reg1)
}

pub fn emit_f64_ge(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ({}) >= ({});\n", result_register, reg2, reg1)
}

pub fn emit_f64_gt(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = ({}) > ({});\n", result_register, reg2, reg1)
}
