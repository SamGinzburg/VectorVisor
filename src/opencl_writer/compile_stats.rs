/*
 * Parse each function and count the number of:
 * 1) Instructions (total)
 * 2) Function calls
 * 3) (Possible) Fastcalls performed
 * 4) Indirect calls
 * 5) Blocks 
 * 6) Loops
 */

use std::convert::TryInto;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::opencl_writer::OpenCLCWriter;
use crate::opencl_writer::format_fn_name;

pub fn function_stats(writer_ctx: &OpenCLCWriter, curr_fn_name: String, func: &wast::Func, fastcalls: &HashSet<String>, func_map: &HashMap<String, wast::Func>, indirect_call_mapping: &HashMap<u32, &wast::Index>) -> (u32, u32, u32, u32, u32, u32, u32) {

    let mut total_instr_count: u32;
    let mut total_func_count: u32 = 0;
    let mut total_fastcall_count: u32 = 0;
    let mut total_indirect_count: u32 = 0;
    let mut total_block_count: u32 = 0;
    let mut total_loop_count: u32 = 0;
    let mut total_local_size: u32 = 0;

    match (&func.kind, &func.id, &func.ty) {
        (wast::FuncKind::Import(_), _, _) => {
            // In this case, we have an InlineImport of the form:
            // (func (type 3) (import "foo" "bar"))
            panic!("InlineImport functions not yet implemented");
        },
        (wast::FuncKind::Inline{locals, expression}, _, _) => {

            for local in locals {
                    total_local_size += match local.ty {
                        wast::ValType::I32 => 4,
                        wast::ValType::F32 => 4,
                        wast::ValType::I64 => 8,
                        wast::ValType::F64 => 8,
                        _ => panic!("Unknown stack type (compile_stats)"),
                    };
            }

            total_instr_count = expression.instrs.len().try_into().unwrap();
            for instr in expression.instrs.iter() {
                match instr {
                    wast::Instruction::Call(idx) => {
                        let id: &str = &match idx {
                            wast::Index::Id(id) => format_fn_name(id.name()),
                            wast::Index::Num(val, _) => format!("func_{}", val),
                        };

                        if fastcalls.contains(id) {
                            total_fastcall_count += 1;
                            // get the func
                            let func = func_map.get(id).unwrap();
                            // Look up the compile stats for the fastcall and add it to our own
                            let (nested_total_instr_count, nested_total_func_count, nested_total_fastcall_count, nested_total_indirect_count, nested_total_block_count, nested_total_loop_count, nested_total_local_size) = function_stats(writer_ctx, curr_fn_name.clone(), func, fastcalls, func_map, indirect_call_mapping);
                            total_instr_count += nested_total_instr_count;
                            total_func_count += nested_total_func_count;
                            total_indirect_count += nested_total_indirect_count;
                            total_loop_count += nested_total_loop_count;
                            total_block_count += nested_total_block_count;
                            total_fastcall_count += nested_total_fastcall_count;
                            total_local_size += nested_total_local_size;
                        } else {
                            total_func_count += 1;
                        }
                    },
                    wast::Instruction::CallIndirect(call_indirect) => {
                        // Check how many fastcalls we are emitting here

                        match (call_indirect.ty.index.as_ref(), call_indirect.ty.inline.as_ref()) {
                            (Some(index), _) => {
                                // if we have an index, we need to look it up in the global structure
                                let type_index = match index {
                                    wast::Index::Num(n, _) => format!("t{}", n),
                                    wast::Index::Id(i) => i.name().to_string(),
                                };
                    
                                let indirect_func_type = match writer_ctx.types.get(&type_index).unwrap() {
                                    wast::TypeDef::Func(ft) => ft,
                                    _ => panic!("Indirect call cannot have a type of something other than a func"),
                                };
    
                                // We only need to call functions with matching type signatures, the rest would trap
                                for func_id in indirect_call_mapping.values() {
                                    let f_name = match func_id {
                                        wast::Index::Id(id) => format_fn_name(id.name()),
                                        wast::Index::Num(val, _) => format!("func_{}", val),
                                    };
                                    let func_type_signature = &writer_ctx.func_map.get(&f_name).unwrap().ty;
    
                                    let _func_type_index = match func_type_signature.index {
                                        Some(wast::Index::Id(id)) => id.name().to_string(),
                                        Some(wast::Index::Num(val, _)) => format!("t{}", val),
                                        None => panic!("Only type indicies supported for call_indirect in vstack pass"),
                                    };
    
                                    /*
                                    // TODO: figure out a way to renable fastcalls within call_indirect
                                    if func_type_index == type_index &&
                                       fastcalls.contains(&f_name) &&
                                       f_name != curr_fn_name {
                                        total_fastcall_count += 1;
                                        // get the func
                                        let func = func_map.get(&f_name).unwrap();
                                        // Look up the compile stats for the fastcall and add it to our own
                                        let (nested_total_instr_count, nested_total_func_count, nested_total_fastcall_count, nested_total_indirect_count, nested_total_block_count, nested_total_loop_count) = function_stats(writer_ctx, f_name, func, fastcalls, func_map, indirect_call_mapping);
                                        total_instr_count += nested_total_instr_count;
                                        total_func_count += nested_total_func_count;
                                        total_indirect_count += nested_total_indirect_count;
                                        total_loop_count += nested_total_loop_count;
                                        total_block_count += nested_total_block_count;
                                        total_fastcall_count += nested_total_fastcall_count;
                                    }
                                    */
                                }
                            },
                            (_, Some(_inline)) => panic!("Inline types for call_indirect not implemented yet (vstack)"),
                            _ => (),
                        };
    
                        total_indirect_count += 1;
                    },
                    wast::Instruction::Block(_) => total_block_count += 1,
                    wast::Instruction::Loop(_) => total_loop_count += 1,
                    _ => (),
                }
            }
        },
        _ => panic!("Unknown function type in function_stats"),
    };

    (total_instr_count, total_func_count, total_fastcall_count, total_indirect_count, total_block_count, total_loop_count, total_local_size)
}
