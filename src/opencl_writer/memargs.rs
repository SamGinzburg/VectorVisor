use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

use std::collections::HashMap;
use crate::opencl_writer::mem_interleave::*;

use wast::MemArg;
use wast::MemoryArg;

// Functions for loading from memory

pub fn emit_memload_i32_8u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let read = format!("(uchar)({})", emit_read_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i32_16u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let read = format!("(uint)({})", emit_read_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i32_16s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let read = format!("(short)({})", emit_read_u16(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i32_8s(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let read = format!("(char)({})", emit_read_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}


pub fn emit_memload_i32(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let read = format!("({})", emit_read_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = format!("({})", emit_read_u64(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_f64(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);

    let read = format!("({})", emit_read_u64(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64_8u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = format!("(ulong)({})", &emit_read_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64_32u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = format!("(ulong)({})", &emit_read_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64_16u(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let result_register = stack_ctx.vstack_alloc(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    let read = format!("(ulong)({})", &emit_read_u16(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

// Functions for loading from memory

pub fn emit_memstore_i32(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                        "(ulong)(heap_u32)",
                        &stored_val,
                        "warp_idx"));

    ret_str
}


pub fn emit_memstore8_i32(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\t{};\n", &emit_write_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                        "(ulong)(heap_u32)",
                        &format!("(char)({})", stored_val),
                        "warp_idx"));

    ret_str
}

pub fn emit_memstore8_i64(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\t{};\n", &emit_write_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                        "(ulong)(heap_u32)",
                        &format!("(char)({})", stored_val),
                        "warp_idx"));

    ret_str
}

pub fn emit_memstore16_i64(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\t{};\n", &emit_write_u16(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                        "(ulong)(heap_u32)",
                        &format!("(short)({})", stored_val),
                        "warp_idx"));

    ret_str
}

pub fn emit_memstore16_i32(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\t{};\n", &emit_write_u16(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                        "(ulong)(heap_u32)",
                        &format!("(short)({})", stored_val),
                        "warp_idx"));

    ret_str
}

pub fn emit_memstore_i64(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                        "(ulong)(heap_u32)",
                        &stored_val,
                        "warp_idx"));

    ret_str
}

pub fn emit_memstore_f64(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::f64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                        "(ulong)(heap_u32)",
                        &stored_val,
                        "warp_idx"));

    ret_str
}

pub fn emit_memstore32_i64(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                        "(ulong)(heap_u32)",
                        &format!("(int)({})", stored_val),
                        "warp_idx"));

    ret_str
}

/*
 * This function is essentially a no-op, since we pre-allocate the heaps for all procs!
 * All we do is update the metadata saying that the heap has grown by N pages
 */
pub fn emit_mem_grow(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, arg: &MemoryArg, debug: bool) -> String {
    // arg is the index of the memory space, however we are assuming that there is only 1 so it doesn't matter
    let num_pages_to_grow_by = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let mut ret_str = String::from("");
    
    // if curr_size + num_pages_to_grow_by <= max size (in terms of pages)
    ret_str += &format!("\t{}\n",
                        format!("if (*current_mem_size + {} <= *max_mem_size) {{", num_pages_to_grow_by));
    // the grow is successful and push curr_size, else push -1 

    ret_str += &format!("\t\tulong temp = {};\n", num_pages_to_grow_by);

    ret_str += &format!("\t\t{} = {};\n", &result_register, "*current_mem_size");
    
    ret_str += &format!("\t\t*current_mem_size += temp;\n");

    ret_str += &format!("\t{}\n",
                        "} else {");
    // the grow failed, push an error onto the stack
    ret_str += &format!("\t\t{} = {};\n", &result_register, "(uint)(-1)");

    ret_str += &format!("\t{}\n",
                        "}");

    ret_str
}

pub fn emit_mem_size(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, arg: &MemoryArg, debug: bool) -> String {
    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    format!("\t{} = {};\n", result_register, "*current_mem_size")
}
