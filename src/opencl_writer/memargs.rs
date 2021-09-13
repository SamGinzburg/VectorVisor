use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

use crate::opencl_writer::mem_interleave::*;

use wast::MemArg;
use wast::MemoryArg;

// Functions for loading from memory

pub fn emit_memload_i32_8u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let read = format!("(uchar)({})", emit_read_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i32_16u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let read = if args.align >= 2 {
        format!("(uint)({})", emit_read_u16_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("(uint)({})", emit_read_u16(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };

    ret_str += &format!("\t{} = ({});\n", result_register, read);

    ret_str
}

pub fn emit_memload_i32_16s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let read = if args.align >= 2 {
        format!("(int)({})", emit_read_u16_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("(int)({})", emit_read_u16(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };


    ret_str += &format!("\t{} = (uint)({});\n", result_register, read);

    ret_str
}

pub fn emit_memload_i32_8s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let read = format!("(char)({})", emit_read_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));

    ret_str += &format!("\t{} = convert_int({});\n", result_register, read);

    ret_str
}


pub fn emit_memload_i32(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let read = if args.align >= 4 {
        format!("({})", emit_read_u32_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("({})", emit_read_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = if args.align >= 8 {
        format!("({})", emit_read_u64_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("({})", emit_read_u64(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_f64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);

    let read = if args.align >= 8 {
        format!("({})", emit_read_u64_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("({})", emit_read_u64(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };

    ret_str += &format!("\t{{\n"); 
    ret_str += &format!("\t\tulong temp = {};\n", read);
    ret_str += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(double));\n", result_register);
    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_memload_f32(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);

    let read = if args.align >= 4 {
        format!("({})", emit_read_u32_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("({})", emit_read_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };

    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\tuint temp = {};\n", read);
    ret_str += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(float));\n", result_register);
    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_memload_i64_8u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = format!("(ulong)({})", &emit_read_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64_32u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = if args.align >= 4 {
        format!("({})", emit_read_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("({})", emit_read_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };

    let read = format!("(ulong)({})", &emit_read_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"));

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64_32s(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = if args.align >= 4 {
        format!("(long)({})", &emit_read_u32_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("(long)({})", &emit_read_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64_16u(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = if args.align >= 2 {
        format!("(ulong)({})", &emit_read_u16_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    } else {
        format!("(ulong)({})", &emit_read_u16(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load), "(ulong)(heap_u32)", "warp_idx"))
    };

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

// Functions for loading from memory

pub fn emit_memstore_i32(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if args.align >= 4 {
        ret_str += &format!("\t{};\n", &emit_write_u32_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            &stored_val,
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            &stored_val,
                            "warp_idx"));
    };

    ret_str
}


pub fn emit_memstore8_i32(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\t{};\n", &emit_write_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                        "(ulong)(heap_u32)",
                        &format!("(char)({})", stored_val),
                        "warp_idx"));

    ret_str
}

pub fn emit_memstore8_i64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\t{};\n", &emit_write_u8(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                        "(ulong)(heap_u32)",
                        &format!("(char)({})", stored_val),
                        "warp_idx"));

    ret_str
}

pub fn emit_memstore16_i64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if args.align >= 2 {
        ret_str += &format!("\t{};\n", &emit_write_u16_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            &format!("(short)({})", stored_val),
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u16(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            &format!("(short)({})", stored_val),
                            "warp_idx"));
    }

    ret_str
}

pub fn emit_memstore16_i32(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if args.align >= 2 {
        ret_str += &format!("\t{};\n", &emit_write_u16_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            &format!("(short)({})", stored_val),
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u16(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            &format!("(short)({})", stored_val),
                            "warp_idx"));
    }

    ret_str
}

pub fn emit_memstore_i64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if args.align >= 8 {
        ret_str += &format!("\t{};\n", &emit_write_u64_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            &stored_val,
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            &stored_val,
                            "warp_idx"));
    }

    ret_str
}

pub fn emit_memstore_f64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::f64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);
    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\tulong temp = 0;\n");
    ret_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n", stored_val);
    if args.align >= 8 {
        ret_str += &format!("\t\t{};\n", &emit_write_u64_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            "temp",
                            "warp_idx"));
    } else {
        ret_str += &format!("\t\t{};\n", &emit_write_u64(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            "temp",
                            "warp_idx"));
    }
    ret_str += &format!("\t}}\n");


    ret_str
}

pub fn emit_memstore_f32(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::f32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);
    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\tuint temp = 0;\n");
    ret_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(float));\n", stored_val);
    if args.align >= 4 {
        ret_str += &format!("\t\t{};\n", &emit_write_u32_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            "temp",
                            "warp_idx"));
    } else {
        ret_str += &format!("\t\t{};\n", &emit_write_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            "temp",
                            "warp_idx"));
    }
    ret_str += &format!("\t}}\n");


    ret_str
}

pub fn emit_memstore32_i64(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, args: &MemArg, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if args.align >= 4 {
        ret_str += &format!("\t{};\n", &emit_write_u32_aligned(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            &format!("(int)({})", stored_val),
                            "warp_idx"));
    } else {
        ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)((global char*)heap_u32+{}+(int)({}))", args.offset, i_load),
                            "(ulong)(heap_u32)",
                            &format!("(int)({})", stored_val),
                            "warp_idx"));
    }

    ret_str
}

/*
 * This function is essentially a no-op, since we pre-allocate the heaps for all procs!
 * All we do is update the metadata saying that the heap has grown by N pages
 */
pub fn emit_mem_grow(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _arg: &MemoryArg, _debug: bool) -> String {
    // arg is the index of the memory space, however we are assuming that there is only 1 so it doesn't matter
    let num_pages_to_grow_by = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let mut ret_str = String::from("");
    
    // if curr_size + num_pages_to_grow_by <= max size (in terms of pages)
    ret_str += &format!("\t{}\n",
                        format!("if (*current_mem_size + (int)({}) <= *max_mem_size) {{", num_pages_to_grow_by));
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

pub fn emit_mem_size(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _arg: &MemoryArg, _debug: bool) -> String {
    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    format!("\t{} = {};\n", result_register, "*current_mem_size")
}
