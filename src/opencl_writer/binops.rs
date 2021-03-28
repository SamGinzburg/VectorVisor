use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

// binops have both values popped off the stack
pub fn emit_i32_add(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) + (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_add(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long)({}) + (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_f64_add(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let mut ret_str = String::from("");

 
    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\t{}\n", "double x;");
    ret_str += &format!("\t\t{}\n", "double y;");
    ret_str += &format!("\t\tulong x_old = {};\n", &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t\tulong y_old = {};\n", &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&x, &x_old, sizeof(double));");
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&y, &y_old, sizeof(double));");
    ret_str += &format!("\t{}\n", "x += y;");
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&x_old, &x, sizeof(double));");

    ret_str += &format!("\t{};\n\t{}\n",
            &emit_write_u64("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            "x_old",
                            "warp_idx"),
            "*sp -= 2;");

    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_f64_ne(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let mut ret_str = String::from("");

 
    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\t{}\n", "double x;");
    ret_str += &format!("\t\t{}\n", "double y;");
    ret_str += &format!("\t\tulong x_old = {};\n", &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t\tulong y_old = {};\n", &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&x, &x_old, sizeof(double));");
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&y, &y_old, sizeof(double));");

    ret_str += &format!("\t{};\n\t{}\n",
            &emit_write_u64("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            "x == y ? 1 : 0",
                            "warp_idx"),
            "*sp -= 2;");

    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_f64_div(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let mut ret_str = String::from("");

    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\t{}\n", "double x;");
    ret_str += &format!("\t\t{}\n", "double y;");
    ret_str += &format!("\t\tulong x_old = {};\n", &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t\tulong y_old = {};\n", &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&x, &x_old, sizeof(double));");
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&y, &y_old, sizeof(double));");
    ret_str += &format!("\t{}\n", "x /= y;");
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&x_old, &x, sizeof(double));");

    ret_str += &format!("\t{};\n\t{}\n",
            &emit_write_u64("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            "x_old",
                            "warp_idx"),
            "*sp -= 2;");

    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_f64_mul(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let mut ret_str = String::from("");

    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\t{}\n", "double x;");
    ret_str += &format!("\t\t{}\n", "double y;");
    ret_str += &format!("\t\tulong x_old = {};\n", &emit_read_u64("(ulong)(stack_u32+*sp-4)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t\tulong y_old = {};\n", &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&x, &x_old, sizeof(double));");
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&y, &y_old, sizeof(double));");
    ret_str += &format!("\t{}\n", "x *= y;");
    ret_str += &format!("\t\t{}\n", "___private_memcpy_nonmmu(&x_old, &x, sizeof(double));");

    ret_str += &format!("\t{};\n\t{}\n",
            &emit_write_u64("(ulong)(stack_u32+*sp-4)",
                            "(ulong)(stack_u32)",
                            "x_old",
                            "warp_idx"),
            "*sp -= 2;");

    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_i32_sub(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) - (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_sub(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long)({}) - (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_and(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) & (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_and(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) & (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_or(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) | (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_shr_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) >> (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_shr_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) >> (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_shr_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) >> (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_shl(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) << ({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_shl(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) << {};\n", result_register, reg2, reg1)
}

pub fn emit_i32_xor(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = {} ^ {};\n", result_register, reg2, reg1)
}

pub fn emit_i32_mul(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) * (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_div_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) / (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_div_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long)({}) / (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_div_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) / (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_div_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) / (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_rem_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (uint)({}) % (uint)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_rem_u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) % (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i32_rem_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i32);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (int)({}) % (int)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_rem_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long)({}) % (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_mul(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long)({}) * (long)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_eq(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (ulong)({}) == (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_f64_eq(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::f64);
    let reg2 = stack_ctx.vstack_pop(StackType::f64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (double)({}) == (double)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_ne(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    format!("\t{} = (ulong)({}) != (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_xor(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) ^ (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_or(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (ulong)({}) | (ulong)({});\n", result_register, reg2, reg1)
}

pub fn emit_i64_shr_s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::i64);
    let reg2 = stack_ctx.vstack_pop(StackType::i64);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    format!("\t{} = (long){} >> {};\n", result_register, reg2, reg1)
}

/*
 * Implementing rotl safely in software: https://blog.regehr.org/archives/1063
 */
pub fn emit_i32_rotl(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    if !debug {
        let reg1 = stack_ctx.vstack_pop(StackType::i32);
        let reg2 = stack_ctx.vstack_pop(StackType::i32);
        let result_register = stack_ctx.vstack_alloc(StackType::i32);
    
        format!("\t{} = rotate({}, {});\n", result_register, reg2, reg1)
    } else {
        let reg1 = stack_ctx.vstack_pop(StackType::i32);
        let reg2 = stack_ctx.vstack_pop(StackType::i32);
        let result_register = stack_ctx.vstack_alloc(StackType::i32);
    
        format!("\t{} = ({x}<<{n}) | ({x}>>(32-{n}));\n", result_register, reg2, reg1)
    }
}

pub fn emit_i64_rotl(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, debug: bool) -> String {
    if !debug {
        let reg1 = stack_ctx.vstack_pop(StackType::i64);
        let reg2 = stack_ctx.vstack_pop(StackType::i64);
        let result_register = stack_ctx.vstack_alloc(StackType::i64);
    
        format!("\t{} = rotate({}, {});\n", result_register, reg2, reg1)
    } else {
        let reg1 = stack_ctx.vstack_pop(StackType::i64);
        let reg2 = stack_ctx.vstack_pop(StackType::i64);
        let result_register = stack_ctx.vstack_alloc(StackType::i64);
    
        format!("\t{} = ({x}<<{n}) | ({x}>>(64-{n}));\n", result_register, reg2, reg1)
    }
}