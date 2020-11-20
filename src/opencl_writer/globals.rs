use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;

use std::collections::HashMap;

pub fn emit_global_get(writer: &opencl_writer::OpenCLCWriter,
                       global_id: &str,
                       global_mappings: &HashMap<&str, (u32, u32)>,
                       debug: bool) -> String {
    let mut ret_str = String::from("");
    let (offset, size) = global_mappings.get(global_id).unwrap();
    match size {
        1 => {
            if debug {
                ret_str += &format!("\t{};\n",
                emit_write_u32("(ulong)(stack_u32+*sp)",
                               "(ulong)(stack_u32)",
                               &emit_read_u32(&format!("(ulong)((char*)globals_buffer+{})", offset*4),
                                                       "(ulong)(globals_buffer)",
                                                       "warp_idx"),
                               "warp_idx"));
            } else {
                ret_str += &format!("\t{};\n",
                emit_write_u32("(ulong)(stack_u32+*sp)",
                               "(ulong)(stack_u32)",
                               &emit_read_u32(&format!("(ulong)((global char*)globals_buffer+{})", offset*4),
                                                       "(ulong)(globals_buffer)",
                                                       "warp_idx"),
                               "warp_idx"));
            }


            ret_str += &String::from("\t*sp += 1;\n");
        },
        2 => {
            if debug {
                ret_str += &format!("\t{};\n",
                emit_write_u64("(ulong)(stack_u32+*sp)",
                               "(ulong)(stack_u32)",
                               &emit_read_u64(&format!("(ulong)((char*)globals_buffer+{})", offset*4),
                                                       "(ulong)(globals_buffer)",
                                                       "warp_idx"),
                               "warp_idx"));
            } else {
                ret_str += &format!("\t{};\n",
                emit_write_u64("(ulong)(stack_u32+*sp)",
                               "(ulong)(stack_u32)",
                               &emit_read_u64(&format!("(ulong)((global char*)globals_buffer+{})", offset*4),
                                                       "(ulong)(globals_buffer)",
                                                       "warp_idx"),
                               "warp_idx"));
            }

            ret_str += &String::from("\t*sp += 2;\n");
        },
        _ => {
            panic!("Unimplemented size for emit_global_get")
        }
    }

    ret_str
}

pub fn emit_global_set(writer: &opencl_writer::OpenCLCWriter,
                       global_id: &str,
                       global_mappings: &HashMap<&str, (u32, u32)>,
                       debug: bool) -> String {
    let mut ret_str = String::from("");
    let (offset, size) = global_mappings.get(global_id).unwrap();
    match size {
        1 => {
            if debug {
                ret_str += &format!("\t{};\n",
                &emit_write_u32(&format!("(ulong)((char*)globals_buffer+{})", offset*4),
                                         "(ulong)(globals_buffer)",
                                         // read the last value off the stack
                                         &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"),
                                         "warp_idx"));
            } else {
                ret_str += &format!("\t{};\n",
                &emit_write_u32(&format!("(ulong)((global char*)globals_buffer+{})", offset*4),
                                         "(ulong)(globals_buffer)",
                                         // read the last value off the stack
                                         &emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"),
                                         "warp_idx"));
            }

            // pop the value off of the stack
            ret_str += &format!("\t{}\n",
                                "*sp -= 1;");
        },
        2 => {
            if debug {
                ret_str += &format!("\t{};\n",
                &emit_write_u64(&format!("(ulong)((char*)globals_buffer+{})", offset*4),
                                "(ulong)(globals_buffer)",
                                // read the last value off the stack
                                &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                "warp_idx"));
            } else {
                ret_str += &format!("\t{};\n",
                &emit_write_u64(&format!("(ulong)((global char*)globals_buffer+{})", offset*4),
                                "(ulong)(globals_buffer)",
                                // read the last value off the stack
                                &emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                "warp_idx"));
            }

            // pop the value off of the stack
            ret_str += &format!("\t{}\n",
                                "*sp -= 2;");
        },
        _ => {
            panic!("Unimplemented size for emit_global_set")
        }
    }

    ret_str
}