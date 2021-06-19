use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

use std::collections::HashMap;

pub fn emit_global_get(writer: &opencl_writer::OpenCLCWriter,
                       stack_ctx: &mut StackCtx,
                       global_id: &str,
                       global_mappings: &HashMap<String, (u32, u32)>,
                       stack_sizes: &mut Vec<u32>,
                       debug: bool) -> String {
    let mut ret_str = String::from("");
    let (offset, size) = global_mappings.get(global_id).unwrap();

    match size {
        1 => {
            stack_sizes.push(1);
            let result_register = stack_ctx.vstack_alloc(StackType::i32);
            let global_read = emit_read_u32(&format!("(ulong)((global char*)globals_buffer+{})", offset*4),
                                            "(ulong)(globals_buffer)",
                                            "warp_idx");
            ret_str += &format!("\t{} = {};\n", result_register, global_read);
        },
        2 => {
            stack_sizes.push(2);
            let result_register = stack_ctx.vstack_alloc(StackType::i64);
            let global_read = emit_read_u64(&format!("(ulong)((global char*)globals_buffer+{})", offset*4),
                                            "(ulong)(globals_buffer)",
                                            "warp_idx");
            ret_str += &format!("\t{} = {};\n", result_register, global_read);
        },
        _ => {
            panic!("Unimplemented size for emit_global_get")
        }
    }

    ret_str
}

pub fn emit_global_set(writer: &opencl_writer::OpenCLCWriter,
                       stack_ctx: &mut StackCtx,
                       global_id: &str,
                       global_mappings: &HashMap<String, (u32, u32)>,
                       stack_sizes: &mut Vec<u32>,
                       debug: bool) -> String {
    let mut ret_str = String::from("");
    let (offset, size) = global_mappings.get(global_id).unwrap();
    stack_sizes.pop().unwrap();

    // TODO: provide better support for non-i32/i64 global types
    // so far not needed for any benchmarks
    match size {
        1 => {
            let reg = stack_ctx.vstack_pop(StackType::i32);

            ret_str += &format!("\t{};\n",
            &emit_write_u32(&format!("(ulong)((global char*)globals_buffer+{})", offset*4),
                                        "(ulong)(globals_buffer)",
                                        // read the last value off the stack
                                        &reg,
                                        "warp_idx"));
        },
        2 => {
            let reg = stack_ctx.vstack_pop(StackType::i64);

            ret_str += &format!("\t{};\n",
            &emit_write_u64(&format!("(ulong)((global char*)globals_buffer+{})", offset*4),
                            "(ulong)(globals_buffer)",
                            // read the last value off the stack
                            &reg,
                            "warp_idx"));
        },
        _ => {
            panic!("Unimplemented size for emit_global_set")
        }
    }

    ret_str
}