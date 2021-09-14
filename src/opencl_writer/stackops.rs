use crate::opencl_writer;
use crate::opencl_writer::ValType;
use crate::opencl_writer::mem_interleave::emit_write_u32_aligned;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

use std::collections::HashMap;

pub fn emit_local_get(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _parameter_offset: i32, id: &str, _offsets: &HashMap<String, u32>, type_info: &HashMap<String, ValType>, stack_sizes: &mut Vec<u32>, _debug: bool) -> String {
    let t = type_info.get(id).unwrap();

    let local_id = if stack_ctx.is_local_local(id.to_string()) {
        format!("{}[thread_idx]", id)
    } else {
        id.to_string()
    };

    match t {
        wast::ValType::I32 => {

            stack_sizes.push(1);

            let register = stack_ctx.vstack_alloc(StackType::i32);
            format!("\t{} = {};\n", register, local_id)
        },
        wast::ValType::I64 => {

            stack_sizes.push(2);

            let register = stack_ctx.vstack_alloc(StackType::i64);
            format!("\t{} = {};\n", register, local_id)
        },
        wast::ValType::F32 => {

            stack_sizes.push(1);

            let register = stack_ctx.vstack_alloc(StackType::f32);
            format!("\t{} = {};\n", register, local_id)
        },
        wast::ValType::F64 => {

            stack_sizes.push(2);

            let register = stack_ctx.vstack_alloc(StackType::f64);
            format!("\t{} = {};\n", register, local_id)
        },
        _ => panic!("emit_local_get type not handled")
    }
}

pub fn emit_local_set(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _parameter_offset: i32, id: &str, offsets: &HashMap<String, u32>, type_info: &HashMap<String, ValType>, stack_sizes: &mut Vec<u32>, is_fastcall: bool, _debug: bool) -> String {
    let cache_offset: u32 = *offsets.get(id).unwrap();
    let t = type_info.get(id).unwrap();
    let cache = if !is_fastcall {
        format!("\tlocal_cache[{}] = 1;\n", cache_offset)
    } else {
        String::from("")
    };

    stack_sizes.pop();

    let local_id = if stack_ctx.is_local_local(id.to_string()) {
        format!("{}[thread_idx]", id)
    } else {
        id.to_string()
    };

    match t {
        wast::ValType::I32 => {
            let register = stack_ctx.vstack_pop(StackType::i32);
            format!("\t{} = {};\n{}", local_id, register, cache)
        },
        wast::ValType::I64 => {
            let register = stack_ctx.vstack_pop(StackType::i64);
            format!("\t{} = {};\n{}", local_id, register, cache)
        },
        wast::ValType::F32 => {
            let register = stack_ctx.vstack_pop(StackType::f32);
            format!("\t{} = {};\n{}", local_id, register, cache)
        },
        wast::ValType::F64 => {
            let register = stack_ctx.vstack_pop(StackType::f64);
            format!("\t{} = {};\n{}", local_id, register, cache)
        },
        _ => panic!("emit_local_set type not handled")
    }
}

pub fn emit_local_tee(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, parameter_offset: i32, id: &str, offsets: &HashMap<String, u32>, type_info: &HashMap<String, ValType>, stack_sizes: &mut Vec<u32>, is_fastcall: bool, debug: bool) -> String {
    let t = type_info.get(id).unwrap();
    let cache_offset: u32 = *offsets.get(id).unwrap();
    let cache = if !is_fastcall {
        format!("\tlocal_cache[{}] = 1;\n", cache_offset)
    } else {
        String::from("")
    };

    let register = match t {
        wast::ValType::I32 => {
            stack_ctx.vstack_peak(StackType::i32, 0)
        },
        wast::ValType::I64 => {
            stack_ctx.vstack_peak(StackType::i64, 0)
        },
        wast::ValType::F32 => {
            stack_ctx.vstack_peak(StackType::f32, 0)
        },
        wast::ValType::F64 => {
            stack_ctx.vstack_peak(StackType::f64, 0)
        },
        _ => panic!("emit_local_tee type not handled")
    };

    if stack_ctx.is_local_local(id.to_string()) {
        format!("\t{}[thread_idx] = {};\n{}", id, register, cache)
    } else {
        format!("\t{} = {};\n{}", id, register, cache)
    }
}

pub fn emit_local(_writer: &opencl_writer::OpenCLCWriter, local: &wast::Local, _debug: bool) -> String {
    /*
     * When emitting locals we know we have access to the global stack.
     * We zero-init all values.
     * 
     */
    match local.ty {
        wast::ValType::I32 => {
            String::from(format!("\t{};\n\t{}\n",
                            &emit_write_u32_aligned("(ulong)(stack_u32+*sp)",
                                            "(ulong)(stack_u32)",
                                            "(uint)0",
                                            "warp_idx"),
                            "*sp += 2;"))
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
                            &emit_write_u32_aligned("(ulong)(stack_u32+*sp)",
                                            "(ulong)(stack_u32)",
                                            "(uint)0",
                                            "warp_idx"),
                            "*sp += 2;"))
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

pub fn emit_i32_const(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, val: &i32, _debug: bool) -> String {
    format!("\t{} = {};\n", stack_ctx.vstack_alloc(StackType::i32), val)
}

pub fn emit_i64_const(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, val: &i64, _debug: bool) -> String {
    format!("\t{} = {};\n", stack_ctx.vstack_alloc(StackType::i64), val)
}

// the float bits are passed as unsigned integer values
pub fn emit_f32_const(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, val: &u32, _debug: bool) -> String {
    let mut ret_val = String::from("");

    ret_val += &format!("\t{{\n");
    ret_val += &format!("\t\tulong temp = {};\n", *val);
    ret_val += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(float));\n", stack_ctx.vstack_alloc(StackType::f32));
    ret_val += &format!("\t}}\n");

    ret_val
}

pub fn emit_f64_const(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, val: &u64, _debug: bool) -> String {    
    let mut ret_val = String::from("");

    ret_val += &format!("\t{{\n");
    ret_val += &format!("\t\tulong temp = {};\n", *val);
    ret_val += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(double));\n", stack_ctx.vstack_alloc(StackType::f64));
    ret_val += &format!("\t}}\n");

    ret_val
}
