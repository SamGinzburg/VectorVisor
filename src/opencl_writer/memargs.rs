use crate::opencl_writer;

use std::collections::HashMap;
use crate::opencl_writer::mem_interleave::*;

use wast::MemArg;
use wast::MemoryArg;

// Functions for loading from memory

pub fn emit_memload_i32_8u(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the previous value i off of the stack, we load from i+offset

    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    if debug {
        ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp-1)",
                            "(ulong)(stack_u32)",
                            &emit_read_u8(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp-1)",
                            "(ulong)(stack_u32)",
                            &emit_read_u8(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                            "warp_idx"));
    }

    ret_str
}

pub fn emit_memload_i32_16u(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the previous value i off of the stack, we load from i+offset

    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    if debug {
        ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp-1)",
                            "(ulong)(stack_u32)",
                            &emit_read_u16(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp-1)",
                            "(ulong)(stack_u32)",
                            &emit_read_u16(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                            "warp_idx"));
    }

    ret_str
}

pub fn emit_memload_i32_8s(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the previous value i off of the stack, we load from i+offset

    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    let read = if debug {
        format!("(char){}", &emit_read_u8(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("(char){}", &emit_read_u8(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };

    ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp-1)",
                                                   "(ulong)(stack_u32)",
                                                   &read,
                                                   "warp_idx"));

    ret_str
}


pub fn emit_memload_i32(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the previous value i off of the stack, we load from i+offset

    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    if debug {
        ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp-1)",
                            "(ulong)(stack_u32)",
                            &emit_read_u32(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u32("(ulong)(stack_u32+*sp-1)",
                            "(ulong)(stack_u32)",
                            &emit_read_u32(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                            "warp_idx"));
    }

    ret_str
}

pub fn emit_memload_i64(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // i_load is always a i32 const
    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    if debug {
        ret_str += &format!("\t{};\n", &emit_write_u64("(ulong)(stack_u32+*sp-1)",
                            "(ulong)(stack_u32)",
                            &emit_read_u64(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u64("(ulong)(stack_u32+*sp-1)",
                            "(ulong)(stack_u32)",
                            &emit_read_u64(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"),
                            "warp_idx"));
    }

    ret_str += &format!("\t{};\n",
                        "*sp += 1");

    ret_str
}

pub fn emit_memload_i64_8u(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // i_load is always a i32 const
    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    let read = if debug {
        format!("(char){}", &emit_read_u8(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("(char){}", &emit_read_u8(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };
    
    ret_str += &format!("\t{};\n", &emit_write_u64("(ulong)(stack_u32+*sp-1)",
                                                   "(ulong)(stack_u32)",
                                                   &read,
                                                   "warp_idx"));

    ret_str += &format!("\t{};\n",
                        "*sp += 1");

    ret_str
}

pub fn emit_memload_i64_32u(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // i_load is always a i32 const
    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
    let read = if debug {
        format!("(char){}", &emit_read_u32(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("(char){}", &emit_read_u32(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };

    ret_str += &format!("\t{};\n", &emit_write_u64("(ulong)(stack_u32+*sp-1)",
                                                   "(ulong)(stack_u32)",
                                                   &read,
                                                   "warp_idx"));

    ret_str += &format!("\t{};\n",
                        "*sp += 1");

    ret_str
}

// Functions for loading from memory

pub fn emit_memstore_i32(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the i value we use to compute the offset: ea=i+offset
    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));

    // then pop the stored value
    let stored_val = &format!("{}", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));

    if debug {
        ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            stored_val,
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            stored_val,
                            "warp_idx"));
    }

    // no values are pushed back
    ret_str += &format!("\t{}\n",
                        "*sp -= 2;");

    ret_str
}


pub fn emit_memstore8_i32(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the i value we use to compute the offset: ea=i+offset
    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));

    // pop the value we are going to store
    let stored_val = &format!("(uchar)({})", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));

    if debug {
        ret_str += &format!("\t{};\n", &emit_write_u8(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            stored_val,
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u8(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            stored_val,
                            "warp_idx"));
    }

    // no values are pushed back
    ret_str += &format!("\t{}\n",
                        "*sp -= 2;");

    ret_str
}

pub fn emit_memstore16_i32(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the i value we use to compute the offset: ea=i+offset
    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));

    // pop the value we are going to store
    let stored_val = &format!("(ushort)({})", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));

    if debug {
        ret_str += &format!("\t{};\n", &emit_write_u16(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            stored_val,
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u16(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            stored_val,
                            "warp_idx"));
    }

    // no values are pushed back
    ret_str += &format!("\t{}\n",
                        "*sp -= 2;");

    ret_str
}

pub fn emit_memstore_i64(writer: &opencl_writer::OpenCLCWriter, args: &MemArg, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop the i value we use to compute the offset: ea=i+offset (always i32)
    let i_load = &format!("(int){}", emit_read_u32("(ulong)(stack_u32+*sp-3)", "(ulong)(stack_u32)", "warp_idx"));

    // pop the value we are going to store
    let stored_val = &format!("{}", emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));

    if debug {
        ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)((char*)heap_u32+{}+{})", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            stored_val,
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)((global char*)heap_u32+{}+{})", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            stored_val,
                            "warp_idx"));
    }

    // no values are pushed back
    ret_str += &format!("\t{}\n",
                        "*sp -= 3;");

    ret_str
}

/*
 * This function is essentially a no-op, since we pre-allocate the heaps for all procs!
 * All we do is update the metadata saying that the heap has grown by N pages
 */
pub fn emit_mem_grow(writer: &opencl_writer::OpenCLCWriter, arg: &MemoryArg, debug: bool) -> String {
    // arg is the index of the memory space, however we are assuming that there is only 1 so it doesn't matter
    let num_pages_to_grow_by = emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx");
    let mut ret_str = String::from("");
    
    // if curr_size + num_pages_to_grow_by <= max size (in terms of pages)
    ret_str += &format!("\t{}\n",
                        format!("if (*current_mem_size + {} <= *max_mem_size) {{", num_pages_to_grow_by));
    // the grow is successful and push curr_size, else push -1 

    ret_str += &format!("\t\tulong temp = {};\n", num_pages_to_grow_by);

    ret_str += &format!("\t\t{};\n",
                        emit_write_u32("(ulong)(stack_u32+*sp-1)",
                                       "(ulong)(stack_u32)",
                                       "*current_mem_size",
                                       "warp_idx"));
    
    ret_str += &format!("\t\t*current_mem_size += temp;\n");

    ret_str += &format!("\t{}\n",
                        "} else {");
    // the grow failed, push an error onto the stack
    ret_str += &format!("\t\t{};\n",
                        emit_write_u32("(ulong)(stack_u32+*sp-1)",
                                       "(ulong)(stack_u32)",
                                       "(uint)-1",
                                       "warp_idx"));

    ret_str += &format!("\t{}\n",
                        "}");

    ret_str
}