use crate::opencl_writer;

use std::collections::HashMap;
use crate::opencl_writer::mem_interleave::*;

use wast::MemArg;

// Functions for loading from memory

pub fn emit_memload_i32_8u(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the previous value i off of the stack, we load from i+offset

    let i_load = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp)",
                                                   "(ulong)(stack_u32)",
                                                   &emit_read_u8(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                                                   "warp_idx"));

    ret_str += &format!("\t{};\n",
                        "*sp += 1");

    ret_str
}

pub fn emit_memload_i32_16u(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the previous value i off of the stack, we load from i+offset

    let i_load = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp)",
                                                   "(ulong)(stack_u32)",
                                                   &emit_read_u16(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                                                   "warp_idx"));

    ret_str += &format!("\t{};\n",
                        "*sp += 1");

    ret_str
}

pub fn emit_memload_i32_8s(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the previous value i off of the stack, we load from i+offset

    let i_load = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    let read = format!("(char){}", &emit_read_u8(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));
    ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp)",
                                                   "(ulong)(stack_u32)",
                                                   &read,
                                                   "warp_idx"));

    ret_str += &format!("\t{};\n",
                        "*sp += 1");

    ret_str
}


pub fn emit_memload_i32(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the previous value i off of the stack, we load from i+offset

    let i_load = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp)",
                                                   "(ulong)(stack_u32)",
                                                   &emit_read_u32(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                                                   "warp_idx"));

    
    ret_str += &format!("\t{};\n",
    "*sp += 1");

    ret_str
}

pub fn emit_memload_i64(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // i_load is always a i32 const
    let i_load = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    ret_str += &format!("\t{};\n", &emit_write_u64("(ulong)(stack_u32+*sp)",
                                                   "(ulong)(stack_u32)",
                                                   &emit_read_u64(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                                                   "warp_idx"));

    ret_str += &format!("\t{};\n",
                        "*sp += 2");

    ret_str
}

pub fn emit_memload_i64_8u(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // i_load is always a i32 const
    let i_load = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    let read = format!("(char){}", &emit_read_u8(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));
    ret_str += &format!("\t{};\n", &emit_write_u64("(ulong)(stack_u32+*sp)",
                                                   "(ulong)(stack_u32)",
                                                   &read,
                                                   "warp_idx"));

    ret_str += &format!("\t{};\n",
                        "*sp += 2");

    ret_str
}

pub fn emit_memload_i64_32u(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // i_load is always a i32 const
    let i_load = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    let read = format!("(char){}", &emit_read_u32(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));
    ret_str += &format!("\t{};\n", &emit_write_u64("(ulong)(stack_u32+*sp)",
                                                   "(ulong)(stack_u32)",
                                                   &read,
                                                   "warp_idx"));

    ret_str += &format!("\t{};\n",
                        "*sp += 2");

    ret_str
}

// Functions for loading from memory

pub fn emit_memstore_i32(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the value we are going to store first
    let i_load = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));

    // then pop the i value we use to compute the offset: ea=i+offset
    let stored_val = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));

    ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load),
                                                   "(ulong)(heap_u32)",
                                                   stored_val,
                                                   "warp_idx"));

    // no values are pushed back
    ret_str += &format!("\t{}\n",
                        "*sp -= 2;");

    ret_str
}


pub fn emit_memstore8_i32(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the value we are going to store first
    let i_load = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));

    // then pop the i value we use to compute the offset: ea=i+offset
    let stored_val = &format!("(uchar)({})", emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));

    ret_str += &format!("\t{};\n", &emit_write_u8(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load),
                                                   "(ulong)(heap_u32)",
                                                   stored_val,
                                                   "warp_idx"));

    // no values are pushed back
    ret_str += &format!("\t{}\n",
                        "*sp -= 2;");

    ret_str
}

pub fn emit_memstore16_i32(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the value we are going to store first
    let i_load = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));

    // then pop the i value we use to compute the offset: ea=i+offset
    let stored_val = &format!("(ushort)({})", emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));

    ret_str += &format!("\t{};\n", &emit_write_u16(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load),
                                                   "(ulong)(heap_u32)",
                                                   stored_val,
                                                   "warp_idx"));

    // no values are pushed back
    ret_str += &format!("\t{}\n",
                        "*sp -= 2;");

    ret_str
}

pub fn emit_memstore_i64(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the value we are going to store first
    let i_load = &format!("{}", emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));

    // then pop the i value we use to compute the offset: ea=i+offset (always i32)
    let stored_val = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-3)", "(ulong)(stack_u32)", "warp_idx"));

    ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load),
                                                   "(ulong)(heap_u32)",
                                                   stored_val,
                                                   "warp_idx"));

    // no values are pushed back
    ret_str += &format!("\t{}\n",
                        "*sp -= 3;");

    ret_str
}