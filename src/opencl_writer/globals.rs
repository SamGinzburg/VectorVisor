use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

use std::collections::HashMap;

pub fn emit_global_get(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    global_id: &str,
    global_mappings: &HashMap<String, (u32, u32, StackType)>,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let (offset, _size, st) = global_mappings.get(global_id).unwrap();

    match st {
        StackType::i32 => {
            let result_register = stack_ctx.vstack_alloc(StackType::i32);
            let global_read = emit_read_u32(
                &format!("(ulong)((global char*)globals_buffer+{})", offset * 4),
                "(ulong)(globals_buffer)",
                "warp_idx",
            );
            ret_str += &format!("\t{} = {};\n", result_register, global_read);
        }
        StackType::i64 => {
            let result_register = stack_ctx.vstack_alloc(StackType::i64);
            let global_read = emit_read_u64(
                &format!("(ulong)((global char*)globals_buffer+{})", offset * 4),
                "(ulong)(globals_buffer)",
                "warp_idx",
            );
            ret_str += &format!("\t{} = {};\n", result_register, global_read);
        }
        _ => {
            panic!("Unimplemented type for emit_global_get")
        }
    }

    ret_str
}

pub fn emit_global_set(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    global_id: &str,
    global_mappings: &HashMap<String, (u32, u32, StackType)>,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");
    let (offset, _size, st) = global_mappings.get(global_id).unwrap();

    // TODO: provide better support for non-i32/i64 global types
    // so far not needed for any benchmarks
    match st {
        StackType::i32 => {
            let reg = stack_ctx.vstack_pop(StackType::i32);

            ret_str += &format!(
                "\t{};\n",
                &emit_write_u32(
                    &format!("(ulong)((global char*)globals_buffer+{})", offset * 4),
                    "(ulong)(globals_buffer)",
                    &reg,
                    "warp_idx"
                )
            );
        }
        StackType::i64 => {
            let reg = stack_ctx.vstack_pop(StackType::i64);

            ret_str += &format!(
                "\t{};\n",
                &emit_write_u64(
                    &format!("(ulong)((global char*)globals_buffer+{})", offset * 4),
                    "(ulong)(globals_buffer)",
                    &reg,
                    "warp_idx"
                )
            );
        }
        _ => {
            panic!("Unimplemented type for emit_global_set")
        }
    }

    ret_str
}
