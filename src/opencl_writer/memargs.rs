use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

use crate::opencl_writer::mem_interleave::*;

use wast::core::*;

// Functions for loading from memory

pub fn emit_memload_i32_8u(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let read = format!(
        "(uchar)({})",
        emit_read_u8(
            &format!(
                "(ulong)((global char*)heap_u32+{}+(int)({}))",
                args.offset, i_load
            ),
            "(ulong)(heap_u32)",
            "warp_idx"
        )
    );

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i32_16u(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let read = if !writer.pretty_input_wasm || args.align < 2 {
        format!(
            "(uint)({})",
            emit_read_u16_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "(uint)({})",
            emit_read_u16_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{} = ({});\n", result_register, read);

    ret_str
}

pub fn emit_memload_i32_16s(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let read = if !writer.pretty_input_wasm || args.align < 2 {
        format!(
            "(int)({})",
            emit_read_u16_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "(int)({})",
            emit_read_u16_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{} = (uint)({});\n", result_register, read);

    ret_str
}

pub fn emit_memload_i32_8s(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let read = format!(
        "(char)({})",
        emit_read_u8(
            &format!(
                "(ulong)((global char*)heap_u32+{}+(int)({}))",
                args.offset, i_load
            ),
            "(ulong)(heap_u32)",
            "warp_idx"
        )
    );

    ret_str += &format!("\t{} = convert_int({});\n", result_register, read);

    ret_str
}

pub fn emit_memload_i32(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    let read = if !writer.pretty_input_wasm || args.align < 4 {
        format!(
            "({})",
            emit_read_u32_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "({})",
            emit_read_u32_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = if !writer.pretty_input_wasm || args.align < 8 {
        format!(
            "({})",
            emit_read_u64_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "({})",
            emit_read_u64_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_f64(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f64);

    let read = if !writer.pretty_input_wasm || args.align < 8 {
        format!(
            "({})",
            emit_read_u64_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "({})",
            emit_read_u64_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\tulong temp = {};\n", read);
    ret_str += &format!(
        "\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(double));\n",
        result_register
    );
    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_memload_f32(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::f32);

    let read = if !writer.pretty_input_wasm || args.align < 4 {
        format!(
            "({})",
            emit_read_u32_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "({})",
            emit_read_u32_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\tuint temp = {};\n", read);
    ret_str += &format!(
        "\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(float));\n",
        result_register
    );
    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_memload_i64_8u(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = format!(
        "(ulong)({})",
        &emit_read_u8(
            &format!(
                "(ulong)((global char*)heap_u32+{}+(int)({}))",
                args.offset, i_load
            ),
            "(ulong)(heap_u32)",
            "warp_idx"
        )
    );

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64_8s(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = format!(
        "(long)({})",
        &emit_read_u8(
            &format!(
                "(ulong)((global char*)heap_u32+{}+(int)({}))",
                args.offset, i_load
            ),
            "(ulong)(heap_u32)",
            "warp_idx"
        )
    );

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64_32u(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = if !writer.pretty_input_wasm || args.align < 4 {
        format!(
            "({})",
            emit_read_u32_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "({})",
            emit_read_u32_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64_32s(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = if !writer.pretty_input_wasm || args.align < 4 {
        format!(
            "(long)({})",
            &emit_read_u32_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "(long)({})",
            &emit_read_u32_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

pub fn emit_memload_i64_16s(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = if !writer.pretty_input_wasm || args.align < 2 {
        format!(
            "(long)({})",
            &emit_read_u16_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "(long)({})",
            &emit_read_u16_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}


pub fn emit_memload_i64_16u(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i64);

    let read = if !writer.pretty_input_wasm || args.align < 2 {
        format!(
            "(ulong)({})",
            &emit_read_u16_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "(ulong)({})",
            &emit_read_u16_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{} = {};\n", result_register, read);

    ret_str
}

// Functions for loading from memory

pub fn emit_memstore_i32(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if !writer.pretty_input_wasm || args.align < 4 {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u32_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &stored_val,
                "warp_idx"
            )
        );
    } else {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u32_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &stored_val,
                "warp_idx"
            )
        );
    }

    ret_str
}

pub fn emit_memstore8_i32(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!(
        "\t{};\n",
        &emit_write_u8(
            &format!(
                "(ulong)((global char*)heap_u32+{}+(int)({}))",
                args.offset, i_load
            ),
            "(ulong)(heap_u32)",
            &format!("(char)({})", stored_val),
            "warp_idx"
        )
    );

    ret_str
}

pub fn emit_memstore8_i64(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!(
        "\t{};\n",
        &emit_write_u8(
            &format!(
                "(ulong)((global char*)heap_u32+{}+(int)({}))",
                args.offset, i_load
            ),
            "(ulong)(heap_u32)",
            &format!("(char)({})", stored_val),
            "warp_idx"
        )
    );

    ret_str
}

pub fn emit_memstore16_i64(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if !writer.pretty_input_wasm || args.align < 2 {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u16_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &format!("(short)({})", stored_val),
                "warp_idx"
            )
        );
    } else {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u16_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &format!("(short)({})", stored_val),
                "warp_idx"
            )
        );
    }

    ret_str
}

pub fn emit_memstore16_i32(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if !writer.pretty_input_wasm || args.align < 2 {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u16_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &format!("(short)({})", stored_val),
                "warp_idx"
            )
        );
    } else {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u16_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &format!("(short)({})", stored_val),
                "warp_idx"
            )
        );
    }

    ret_str
}

pub fn emit_memstore_i64(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if !writer.pretty_input_wasm || args.align < 8 {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u64_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &stored_val,
                "warp_idx"
            )
        );
    } else {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u64_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &stored_val,
                "warp_idx"
            )
        );
    }

    ret_str
}

pub fn emit_memstore_f64(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::f64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);
    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\tulong temp = 0;\n");
    ret_str += &format!(
        "\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n",
        stored_val
    );

    if !writer.pretty_input_wasm || args.align < 8 {
        ret_str += &format!(
            "\t\t{};\n",
            &emit_write_u64_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "temp",
                "warp_idx"
            )
        );
    } else {
        ret_str += &format!(
            "\t\t{};\n",
            &emit_write_u64_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "temp",
                "warp_idx"
            )
        );
    }

    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_memstore_f32(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::f32);
    let i_load = stack_ctx.vstack_pop(StackType::i32);
    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\tuint temp = 0;\n");
    ret_str += &format!(
        "\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(float));\n",
        stored_val
    );

    if !writer.pretty_input_wasm || args.align < 4 {
        ret_str += &format!(
            "\t\t{};\n",
            &emit_write_u32_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "temp",
                "warp_idx"
            )
        );
    } else {
        ret_str += &format!(
            "\t\t{};\n",
            &emit_write_u32_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "temp",
                "warp_idx"
            )
        );
    }

    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_memstore32_i64(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::i64);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if !writer.pretty_input_wasm || args.align < 4 {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u32_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &format!("(int)({})", stored_val),
                "warp_idx"
            )
        );
    } else {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u32_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &format!("(int)({})", stored_val),
                "warp_idx"
            )
        );
    }

    ret_str
}

pub fn emit_memload_u128(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::u128);

    // We just use two 8-byte reads for 16-byte values
    let read_bottom = if !writer.pretty_input_wasm || args.align < 8 {
        format!(
            "({})",
            emit_read_u64_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "({})",
            emit_read_u64_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    let read_top = if !writer.pretty_input_wasm || args.align < 8 {
        format!(
            "({})",
            emit_read_u64_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset+8, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "({})",
            emit_read_u64_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset+8, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\tulong2 *temp = &{};\n", result_register);
    ret_str += &format!("\t\t(*temp).x = {};\n", read_bottom);
    ret_str += &format!("\t\t(*temp).y = {};\n", read_top);
    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_memload_u128_zero_64(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let i_load = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::u128);

    // We just use two 8-byte reads for 16-byte values
    let read_bottom = if !writer.pretty_input_wasm || args.align < 8 {
        format!(
            "({})",
            emit_read_u64_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    } else {
        format!(
            "({})",
            emit_read_u64_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                "warp_idx"
            )
        )
    };

    ret_str += &format!("\t{{\n");
    ret_str += &format!("\t\tulong2 *temp = &{};\n", result_register);
    ret_str += &format!("\t\t(*temp).x = {};\n", read_bottom);
    ret_str += &format!("\t\t(*temp).y = {};\n", 0);
    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_memstore_u128(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    args: &MemArg,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let stored_val = stack_ctx.vstack_pop(StackType::u128);
    let i_load = stack_ctx.vstack_pop(StackType::i32);

    if !writer.pretty_input_wasm || args.align < 8 {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u64_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &format!("(*(ulong2*)(&{})).x", stored_val),
                "warp_idx"
            )
        );
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u64_aligned_checked(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset+8, i_load
                ),
                "(ulong)(heap_u32)",
                &format!("(*(ulong2*)(&{})).y", stored_val),
                "warp_idx"
            )
        );
    } else {
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u64_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset, i_load
                ),
                "(ulong)(heap_u32)",
                &format!("(*(ulong2*)(&{})).x", stored_val),
                "warp_idx"
            )
        );
        ret_str += &format!(
            "\t{};\n",
            &emit_write_u64_aligned(
                &format!(
                    "(ulong)((global char*)heap_u32+{}+(int)({}))",
                    args.offset+8, i_load
                ),
                "(ulong)(heap_u32)",
                &format!("(*(ulong2*)(&{})).y", stored_val),
                "warp_idx"
            )
        );
    }

    ret_str
}

/*
 * This function is essentially a no-op, since we pre-allocate the heaps for all procs!
 * All we do is update the metadata saying that the heap has grown by N pages
 */
pub fn emit_mem_grow(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _arg: &MemoryArg,
    _debug: bool,
) -> String {
    // arg is the index of the memory space, however we are assuming that there is only 1 so it doesn't matter
    let num_pages_to_grow_by = stack_ctx.vstack_pop(StackType::i32);
    let result_register = stack_ctx.vstack_alloc(StackType::i32);

    let mut ret_str = String::from("");

    // if curr_size + num_pages_to_grow_by <= max size (in terms of pages)
    ret_str += &format!(
        "\t{}\n",
        format!(
            "if (*current_mem_size + (int)({}) <= *max_mem_size) {{",
            num_pages_to_grow_by
        )
    );
    // the grow is successful and push curr_size, else push -1

    ret_str += &format!("\t\tulong temp = {};\n", num_pages_to_grow_by);

    ret_str += &format!("\t\t{} = {};\n", &result_register, "*current_mem_size");

    ret_str += &format!("\t\t*current_mem_size += temp;\n");

    ret_str += &format!("\t{}\n", "} else {");
    // the grow failed, push an error onto the stack
    ret_str += &format!("\t\t{} = {};\n", &result_register, "(uint)(-1)");

    ret_str += &format!("\t{}\n", "}");

    ret_str
}

pub fn emit_mem_size(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _arg: &MemoryArg,
    _debug: bool,
) -> String {
    let result_register = stack_ctx.vstack_alloc(StackType::i32);
    format!("\t{} = {};\n", result_register, "*current_mem_size")
}

pub fn emit_memcpy(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _arg: &MemoryCopy,
    _debug: bool,
) -> String {
    let n_bytes = stack_ctx.vstack_pop(StackType::i32);
    let src = stack_ctx.vstack_pop(StackType::i32);
    let dst = stack_ctx.vstack_pop(StackType::i32);
    emit_intra_vm_memcpy(&format!("((global char*)(heap_u32)+{})", src), "(global char*)(heap_u32)",
			 &format!("((global char*)(heap_u32)+{})", dst), "(global char*)(heap_u32)",
			 &n_bytes,
			 "warp_idx")
}

pub fn emit_memfill(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _arg: &MemoryArg,
    _debug: bool,
) -> String {
    let n_bytes = stack_ctx.vstack_pop(StackType::i32);
    let val = stack_ctx.vstack_pop(StackType::i32);
    let dst = stack_ctx.vstack_pop(StackType::i32);
    emit_intra_vm_memfill(&format!("((global char*)(heap_u32)+{})", dst), "(global char*)(heap_u32)",
			  &val,
			  &n_bytes,
			  "warp_idx")
}
