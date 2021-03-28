use crate::opencl_writer;
use crate::opencl_writer::ValType;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

use std::collections::HashMap;

pub fn emit_local_get(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, parameter_offset: i32, id: &str, offsets: &HashMap<String, u32>, type_info: &HashMap<String, ValType>, stack_sizes: &mut Vec<u32>, debug: bool) -> String {
    let offset: i32 = *offsets.get(id).unwrap() as i32 + parameter_offset;
    let t = type_info.get(id).unwrap();

    match t {
        wast::ValType::I32 => {

            stack_sizes.push(1);
            let register = stack_ctx.vstack_alloc(StackType::i32);
            format!("\t{} = {};\n", register, id)
        },
        wast::ValType::I64 => {

            stack_sizes.push(2);
            let register = stack_ctx.vstack_alloc(StackType::i64);

            format!("\t{} = {};\n", register, id)
        },
        wast::ValType::F32 => {

            stack_sizes.push(1);

            let register = stack_ctx.vstack_alloc(StackType::f32);
            format!("\t{} = {};\n", register, id)
        },
        wast::ValType::F64 => {

            stack_sizes.push(2);

            let register = stack_ctx.vstack_alloc(StackType::f64);

            format!("\t{} = {};\n", register, id)
        },
        _ => panic!("emit_local_get type not handled")
    }
}

pub fn emit_local_set(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, parameter_offset: i32, id: &str, offsets: &HashMap<String, u32>, type_info: &HashMap<String, ValType>, stack_sizes: &mut Vec<u32>, debug: bool) -> String {
    let offset: i32 = *offsets.get(id).unwrap() as i32 + parameter_offset;
    let t = type_info.get(id).unwrap();

    stack_sizes.pop();

    match t {
        wast::ValType::I32 => {
            let register = stack_ctx.vstack_pop(StackType::i32);
            format!("\t{} = {};\n", id, register)
        },
        wast::ValType::I64 => {
            let register = stack_ctx.vstack_pop(StackType::i64);
            format!("\t{} = {};\n", id, register)
        },
        wast::ValType::F32 => {
            let register = stack_ctx.vstack_pop(StackType::f32);
            format!("\t{} = {};\n", id, register)
        },
        wast::ValType::F64 => {
            let register = stack_ctx.vstack_pop(StackType::f64);
            format!("\t{} = {};\n", id, register)
        },
        _ => panic!("emit_local_set type not handled")
    }
}

pub fn emit_local_tee(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, parameter_offset: i32, id: &str, offsets: &HashMap<String, u32>, type_info: &HashMap<String, ValType>, stack_sizes: &mut Vec<u32>, debug: bool) -> String {
    /*
     * peak the top of the stack, push the most recent value again
     * call local.set [x]
     */
    let offset = offsets.get(id).unwrap();
    let t = type_info.get(id).unwrap();

    match t {
        wast::ValType::I32 => {
            stack_sizes.push(1);
            let reg1 = stack_ctx.vstack_pop(StackType::i32);
            let reg2 = stack_ctx.vstack_alloc(StackType::i32);
            let reg3 = stack_ctx.vstack_alloc(StackType::i32);
            format!("\t{} = {};\n{}", reg3, reg2, emit_local_set(writer, stack_ctx, parameter_offset, id, offsets, type_info, stack_sizes, debug))
        },
        wast::ValType::I64 => {

            stack_sizes.push(2);

            let reg1 = stack_ctx.vstack_pop(StackType::i64);
            let reg2 = stack_ctx.vstack_alloc(StackType::i64);
            let reg3 = stack_ctx.vstack_alloc(StackType::i64);
            format!("\t{} = {};\n{}", reg3, reg2, emit_local_set(writer, stack_ctx, parameter_offset, id, offsets, type_info, stack_sizes, debug))
        },
        wast::ValType::F32 => {

            stack_sizes.push(1);


            let reg1 = stack_ctx.vstack_pop(StackType::f32);
            let reg2 = stack_ctx.vstack_alloc(StackType::f32);
            let reg3 = stack_ctx.vstack_alloc(StackType::f32);
            format!("\t{} = {};\n{}", reg3, reg2, emit_local_set(writer, stack_ctx, parameter_offset, id, offsets, type_info, stack_sizes, debug))
        },
        wast::ValType::F64 => {

            stack_sizes.push(2);

            let reg1 = stack_ctx.vstack_pop(StackType::f64);
            let reg2 = stack_ctx.vstack_alloc(StackType::f64);
            let reg3 = stack_ctx.vstack_alloc(StackType::f64);
            format!("\t{} = {};\n{}", reg3, reg2, emit_local_set(writer, stack_ctx, parameter_offset, id, offsets, type_info, stack_sizes, debug))
        },
        _ => panic!("emit_local_tee type not handled")
    }
}

pub fn emit_local(writer: &opencl_writer::OpenCLCWriter, local: &wast::Local, debug: bool) -> String {
    /*
     * When emitting locals we know we have access to the global stack.
     * We zero-init all values.
     * 
     */
    match local.ty {
        wast::ValType::I32 => {
            String::from(format!("\t{};\n\t{}\n",
                            &emit_write_u32("(ulong)(stack_u32+*sp)",
                                            "(ulong)(stack_u32)",
                                            "(uint)0",
                                            "warp_idx"),
                            "*sp += 1;"))
        },
        wast::ValType::I64 => {
            String::from(format!("\t{};\n\t{}\n",
                            &emit_write_u64("(ulong)(stack_u32+*sp)",
                                            "(ulong)(stack_u32)",
                                            "(ulong)0",
                                            "warp_idx"),
                            "*sp += 2;"))
        },
        wast::ValType::F32 => {
            String::from(format!("\t{};\n\t{}\n",
                            &emit_write_u32("(ulong)(stack_u32+*sp)",
                                            "(ulong)(stack_u32)",
                                            "(uint)0",
                                            "warp_idx"),
                            "*sp += 1;"))
        },
        wast::ValType::F64 => {
            String::from(format!("\t{};\n\t{}\n",
                            &emit_write_u64("(ulong)(stack_u32+*sp)",
                                            "(ulong)(stack_u32)",
                                            "(ulong)0",
                                            "warp_idx"),
                            "*sp += 2;"))
        },
        _ => panic!(),
    }
}

pub fn emit_i32_const(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, val: &i32, debug: bool) -> String {
    format!("\t{} = {};\n", stack_ctx.vstack_alloc(StackType::i32), val)
}

pub fn emit_i64_const(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, val: &i64, debug: bool) -> String {
    format!("\t{} = {};\n", stack_ctx.vstack_alloc(StackType::i64), val)
}

// the float bits are passed as unsigned integer values
pub fn emit_f32_const(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, val: &u32, debug: bool) -> String {
    format!("\t{} = {};\n", stack_ctx.vstack_alloc(StackType::f32), val)
}

pub fn emit_f64_const(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, val: &u64, debug: bool) -> String {
    format!("\t{} = {};\n", stack_ctx.vstack_alloc(StackType::f64), val)
}