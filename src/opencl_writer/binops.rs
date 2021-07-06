use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

// binops have both values popped off the stack
pub fn emit_i32_add(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) + (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_add(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long)({}) + (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_f64_add(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);

    format!("\t{} = {} + {};\n", result_register, reg2, reg1)
}

pub fn emit_f64_max(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);

    format!("\t{} = FMAX({}, {});\n", result_register, reg2, reg1)
}

pub fn emit_f32_max(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);

    format!("\t{} = FMAX({}, {});\n", result_register, reg2, reg1)
}

pub fn emit_f64_min(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);

    format!("\t{} = FMIN({}, {});\n", result_register, reg2, reg1)
}

pub fn emit_f32_min(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);

    format!("\t{} = FMIN({}, {});\n", result_register, reg2, reg1)
}

pub fn emit_f64_sub(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);

    format!("\t{} = {} - {};\n", result_register, reg2, reg1)
}

pub fn emit_f32_sub(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);

    format!("\t{} = {} - {};\n", result_register, reg2, reg1)
}

pub fn emit_f32_add(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);

    format!("\t{} = {} + {};\n", result_register, reg2, reg1)
}

pub fn emit_f32_mul(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);

    format!("\t{} = (float){} * (float){};\n", result_register, reg2, reg1)
}

pub fn emit_f64_ne(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = {} != {};\n", result_register, reg2, reg1)
}

pub fn emit_f32_ne(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = {} != {};\n", result_register, reg2, reg1)
}

pub fn emit_f64_div(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);

    format!("\t{} = {} / {};\n", result_register, reg2, reg1)
}

pub fn emit_f32_div(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);

    format!("\t{} = {} / {};\n", result_register, reg2, reg1)
}

pub fn emit_f64_mul(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);

    format!("\t{} = {} * {};\n", result_register, reg2, reg1)
}

pub fn emit_i32_sub(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) - (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_sub(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long)({}) - (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_and(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) & (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_and(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) & (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_or(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) | (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_shr_u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) >> (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_shr_u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) >> (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_shr_s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) >> (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_shl(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) << (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_shl(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) << {};\n", result_register, reg2, reg1)
}

pub fn emit_i32_xor(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = {} ^ {};\n", result_register, reg2, reg1)
}

pub fn emit_i32_mul(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) * (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_div_u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) / (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_div_s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long)({}) / (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_div_u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) / (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_div_s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) / (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_rem_u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) % (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_rem_u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) % (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_rem_s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) % (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_rem_s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long)({}) % (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_mul(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long)({}) * (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_eq(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (ulong)({}) == (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_f64_eq(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (double)({}) == (double)({});\n", result_register, reg2, reg1)
}

pub fn emit_f32_eq(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f32);
    let reg2 = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (float)({}) == (float)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_ne(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (ulong)({}) != (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_xor(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) ^ (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_or(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) | (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_shr_s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long){} >> {};\n", result_register, reg2, reg1)
}

/*
 * Implementing rotl safely in software: https://blog.regehr.org/archives/1063
 */
pub fn emit_i32_rotl(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    if !debug {
        let reg1 = stack_ctx.vstack_pop(StackType::i32);
        let reg2 = stack_ctx.vstack_pop(StackType::i32);
        let result_register = stack_ctx.vstack_alloc(StackType::i32);

        format!("\t{} = rotate({}, {});\n", result_register, reg2, reg1)
    } else {
        let reg1 = stack_ctx.vstack_pop(StackType::i32);
        let reg2 = stack_ctx.vstack_pop(StackType::i32);
        let result_register = stack_ctx.vstack_alloc(StackType::i32);
    
        format!("\t{} = ({x}<<{n}) | ({x}>>(32-{n}));\n", result_register, x=reg2, n=reg1)
    }
}

pub fn emit_i64_rotl(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    if !debug {
        let reg1 = stack_ctx.vstack_pop(StackType::i64);
        let reg2 = stack_ctx.vstack_pop(StackType::i64);
        let result_register = stack_ctx.vstack_alloc(StackType::i64);
    
        format!("\t{} = rotate({}, {});\n", result_register, reg2, reg1)
    } else {
        let reg1 = stack_ctx.vstack_pop(StackType::i64);
        let reg2 = stack_ctx.vstack_pop(StackType::i64);
        let result_register = stack_ctx.vstack_alloc(StackType::i64);
    
        format!("\t{} = ({x}<<{n}) | ({x}>>(64-{n}));\n", result_register, x=reg2, n=reg1)
    }
}
