/*
 * This file contains the implementation of the virtual stack
 * 
 * The virtual stack converts all stack reads into intermediate values to minimize writes to global memory.
 * The secondary benefit is we perform far fewer MMU translations for value lookups/stores
 * 
 * 
 * In the vstack model, all parameters, locals are also intermediates as well
 * 
 */
use crate::opencl_writer::OpenCLCWriter;
use crate::opencl_writer::WASI_SNAPSHOT_PREVIEW1;
use crate::opencl_writer::emit_read_u32;
use crate::opencl_writer::emit_read_u64;
use crate::opencl_writer::emit_write_u32;
use crate::opencl_writer::emit_write_u64;

use wast::Instruction;
use wast::ValType;
use std::collections::HashMap;

#[derive(Debug)]
pub enum StackType {
    i32,
    i64,
    f32,
    f64
}

/*
 * The stacks are generated during the first compilation pass
 * Indicies are tracked during execution
 */
#[derive(Debug)]
pub struct StackCtx {
    // Track each intermediate type separately
    i32_stack: Vec<String>,
    i32_idx: usize,
    i64_stack: Vec<String>,
    i64_idx: usize,
    f32_stack: Vec<String>,
    f32_idx: usize,
    f64_stack: Vec<String>,
    f64_idx: usize,
    // Local & Parameter information
    stack_frame_offset: u32,
    local_offsets: HashMap<String, u32>,
    local_types: HashMap<String, StackType>,
    param_offset: i32,
    // Track the whole stack too
    // Note: this is updated in vstack_pop/vstack_alloc and not initialize_context
    pub stack_sizes: Vec<u32>,
}

impl<'a> StackCtx {
    /*
     * Parse a function and generate a stack context for it.
     * We can statically determine the maximum required amount of intermediate values
     */
    pub fn initialize_context(writer_ctx: &OpenCLCWriter, instructions: &Box<[Instruction<'a>]>, local_param_types: &HashMap<String, ValType>, local_offsets: &HashMap<String, u32>, is_param: &HashMap<String, bool>, param_offset: i32) -> StackCtx {
        let mut stack_sizes = vec![];
        
        // We treat local & parameter intermediates the same, as they are already named at this point


        // Track how many of each intermediate we are going to need!
        let mut current_i32_count: u32 = 0;
        let mut max_i32_count: u32 = 0;

        let mut current_i64_count: u32 = 0;
        let mut max_i64_count: u32 = 0;

        let mut current_f32_count: u32 = 0;
        let mut max_f32_count: u32 = 0;

        let mut current_f64_count: u32 = 0;
        let mut max_f64_count: u32 = 0;

        fn update_counter(curr_value: &mut u32, max_value: &mut u32) -> () {
            *curr_value += 1;
            if *curr_value > *max_value {
                *max_value = *curr_value;
            }
        }

        fn update_by_valtype(valtype: &ValType,
                             curr_value_i32: &mut u32, max_value_i32: &mut u32,
                             curr_value_i64: &mut u32, max_value_i64: &mut u32,
                             curr_value_f32: &mut u32, max_value_f32: &mut u32,
                             curr_value_f64: &mut u32, max_value_f64: &mut u32) -> () {
            match valtype {
                wast::ValType::I32 => {
                    update_counter(curr_value_i32, max_value_i32);
                },
                wast::ValType::F32 => {
                    update_counter(curr_value_f32, max_value_f32);
                },
                wast::ValType::I64 => {
                    update_counter(curr_value_i64, max_value_i64);
                },
                wast::ValType::F64 => {
                    update_counter(curr_value_f64, max_value_f64);
                },
                _ => panic!("vstack update by valtype error"),
            }
        };

        // for each instr, 
        for instruction in instructions.iter() {
            println!("{:?}", instruction);
            match instruction {
                wast::Instruction::Drop => {
                    stack_sizes.pop().unwrap();
                }
                wast::Instruction::I32Store(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                },
                wast::Instruction::I32Store8(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                },
                wast::Instruction::I64Store8(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                },
                wast::Instruction::I64Store16(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                },
                wast::Instruction::I32Store16(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                },
                wast::Instruction::I64Store32(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                },
                wast::Instruction::I32Load(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                },
                wast::Instruction::I32Load8u(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                },
                wast::Instruction::I64Load16u(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(2);
                },
                wast::Instruction::I32Load16u(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                },
                wast::Instruction::I32Load16s(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                },
                wast::Instruction::I32Load8s(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                },
                wast::Instruction::I64Load8u(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(2);
                },
                wast::Instruction::I64Load32u(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(2);
                },
                wast::Instruction::I64Load(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(2);
                },
                wast::Instruction::F64Load(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(2);
                },
                wast::Instruction::I64Store(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                },
                wast::Instruction::F64Store(memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                },
                /*
                 * As of right now we only support i32 globals anyways...
                 * TODO: for future support of globals, check for other types here
                 */
                wast::Instruction::GlobalGet(idx) => {
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::GlobalSet(idx) => {
                    current_i32_count -= 1;
                },
                wast::Instruction::I32Const(val) => {
                    stack_sizes.push(1);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64Const(val) => {
                    stack_sizes.push(2);
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::F32Const(val) => {
                    stack_sizes.push(1);
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                },
                wast::Instruction::F64Const(val) => {
                    stack_sizes.push(2);
                    update_counter(&mut current_f64_count, &mut max_f64_count);

                },
                wast::Instruction::LocalGet(idx) => {
                    match idx {
                        wast::Index::Id(id) => {
                            update_by_valtype(local_param_types.get(&id.name().to_string()).unwrap(),
                                                &mut current_i32_count, &mut max_i32_count,
                                                &mut current_i64_count, &mut max_i64_count,
                                                &mut current_f32_count, &mut max_f32_count,
                                                &mut current_f64_count, &mut max_f64_count);
                        },
                        wast::Index::Num(value, _) => {
                            let id = match is_param.get(&format!("l{}", value)) {
                                Some(false) => {
                                    format!("l{}", value)
                                },
                                Some(true) => format!("p{}", value),
                                _ => format!("p{}", value),
                            };
                            update_by_valtype(local_param_types.get(&id).unwrap(),
                                                &mut current_i32_count, &mut max_i32_count,
                                                &mut current_i64_count, &mut max_i64_count,
                                                &mut current_f32_count, &mut max_f32_count,
                                                &mut current_f64_count, &mut max_f64_count);
                        },
                    }
                },
                wast::Instruction::LocalSet(idx) => {
                    match idx {
                        wast::Index::Id(id) => {
                            match local_param_types.get(&id.name().to_string()).unwrap() {
                                ValType::I32 => {
                                    current_i32_count -= 1;
                                },
                                ValType::F32 => {
                                    current_f32_count -= 1;
                                },
                                ValType::I64 => {
                                    current_i64_count -= 1;
                                },
                                ValType::F64 => {
                                    current_f64_count -= 1;
                                },
                                _ => panic!("Unknown local size found (vstack init)"),
                            }
                        },
                        wast::Index::Num(value, _) => {
                            let id = match is_param.get(&format!("l{}", value)) {
                                Some(false) => {
                                    format!("l{}", value)
                                },
                                Some(true) => format!("p{}", value),
                                _ => format!("p{}", value),
                            };
                            match local_param_types.get(&id).unwrap() {
                                ValType::I32 => {
                                    current_i32_count -= 1;
                                },
                                ValType::F32 => {
                                    current_f32_count -= 1;
                                },
                                ValType::I64 => {
                                    current_i64_count -= 1;
                                },
                                ValType::F64 => {
                                    current_f64_count -= 1;
                                },
                                _ => panic!("Unknown local size found (vstack init)"),
                            }
                        },
                    }
                },
                wast::Instruction::LocalTee(idx) => {
                    match idx {
                        wast::Index::Id(id) => {
                            update_by_valtype(local_param_types.get(&id.name().to_string()).unwrap(),
                                                &mut current_i32_count, &mut max_i32_count,
                                                &mut current_i64_count, &mut max_i64_count,
                                                &mut current_f32_count, &mut max_f32_count,
                                                &mut current_f64_count, &mut max_f64_count);
                        },
                        wast::Index::Num(value, _) => {
                            let id = match is_param.get(&format!("l{}", value)) {
                                Some(false) => {
                                    format!("l{}", value)
                                },
                                Some(true) => format!("p{}", value),
                                _ => format!("p{}", value),
                            };
                            update_by_valtype(local_param_types.get(&id).unwrap(),
                                                &mut current_i32_count, &mut max_i32_count,
                                                &mut current_i64_count, &mut max_i64_count,
                                                &mut current_f32_count, &mut max_f32_count,
                                                &mut current_f64_count, &mut max_f64_count);
                        },
                    }
                },
                wast::Instruction::I32Add => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 1;
                },
                wast::Instruction::I32Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 1;
                },
                wast::Instruction::I64Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    current_i64_count -= 1;
                },
                wast::Instruction::I32Sub => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 1;
                },
                wast::Instruction::I64Add => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    current_i64_count -= 1;
                },
                wast::Instruction::F64Add => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    current_f64_count -= 1;
                },
                wast::Instruction::F64Div => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    current_f64_count -= 1;
                },
                wast::Instruction::F64Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    current_f64_count -= 1;
                },
                wast::Instruction::F64Neg => {
                },
                wast::Instruction::F64Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64Lt => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64Le => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64LtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32TruncF64U => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_f64_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64DivU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32Eqz => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    // no-op
                },
                wast::Instruction::I64Eqz => {
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i64_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32And => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64And => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I32Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32LtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32LtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64LtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32GtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64GtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64GtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32GtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32LeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32LeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64LeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64LeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32GeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32GeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64GeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64GeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32Xor => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32WrapI64 => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    current_i64_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64ExtendI32S => {
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64ExtendI32U => {
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::Call(idx) => {
                    let id = match idx {
                        wast::Index::Id(id) => id.name(),
                        _ => panic!("Unable to get Id for function call: {:?}", idx),
                    };

                    if writer_ctx.imports_map.contains_key(id) {
                        match writer_ctx.imports_map.get(id) {
                            Some((wasi_api, Some(wasi_fn_name), _)) => {
                                match (wasi_api, WASI_SNAPSHOT_PREVIEW1.get(wasi_fn_name)) {
                                    // ignore WASI API scoping for now
                                    (_, Some(true)) => {
                                        match wasi_fn_name {
                                            &"fd_write"               => {

                                            },
                                            &"proc_exit"              => {

                                            },
                                            &"environ_sizes_get"      => {

                                            },
                                            &"environ_get"            => {

                                            },
                                            &"fd_prestat_get"         => {

                                            },
                                            &"fd_prestat_dir_name"    => {

                                            },
                                            &"random_get"             => {

                                            },
                                            &"serverless_invoke"      => {

                                            },
                                            &"serverless_response"    => {

                                            },
                                            _ => panic!("Unidentified WASI fn name: {:?} (vstack)", wasi_fn_name),
                                        }
                                    },
                                    _ => panic!("WASI import not found, this probably means the hypercall is not yet implemented: {:?} (vstack)", wasi_fn_name)
                                }
                            },
                            _ => panic!("Unsupported hypercall found {:?} (vstack)", writer_ctx.imports_map.get(id))
                        }
                    } else {
                        // else, this is a normal function call
                        // if self.func_map.get(id) is none, we have an import
                        // right now we only support WASI imports
                        match writer_ctx.func_map.get(id) {
                            Some(_) => {
                                let func_type_signature = &writer_ctx.func_map.get(id).unwrap().ty;

                                let params = &func_type_signature.inline.as_ref().unwrap().params;
                                let results = &func_type_signature.inline.as_ref().unwrap().results;

                                for (_, _, ty) in params.iter() {
                                    match ty {
                                        ValType::I32 => {
                                            current_i32_count -= 1;
                                        },
                                        ValType::I64 => {
                                            current_i64_count -= 1;
                                        },
                                        ValType::F32 => {
                                            current_f32_count -= 1;
                                        },
                                        ValType::F64 => {
                                            current_f64_count -= 1;
                                        },
                                        _ => panic!("vstack missing valtype check in func call")
                                    }
                                }

                                // push the results back
                                for ty in results.iter() {
                                    update_by_valtype(ty,
                                                        &mut current_i32_count, &mut max_i32_count,
                                                        &mut current_i64_count, &mut max_i64_count,
                                                        &mut current_f32_count, &mut max_f32_count,
                                                        &mut current_f64_count, &mut max_f64_count);
                                }
                            },
                            // we have an import that isn't a system call...
                            None => {
                                panic!("Unknown import (vstack)");
                            }
                        }
                    }
                },
                wast::Instruction::CallIndirect(call_indirect) => {
                    // Check for types
                },
                wast::Instruction::I32Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);

                },
                wast::Instruction::I32Or => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32ShrU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64ShrU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I32ShrS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32Shl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64Shl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I32DivU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32DivS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64DivS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I32RemU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64RemU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I32RemS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64RemS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64ShrS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64Xor => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64Or => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I32Rotl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(1);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64Rotl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64Sub => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64ReinterpretF64 => {
                    current_f64_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::F64ReinterpretI64 => {
                    current_i64_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::F32ReinterpretI32 => {
                    current_i32_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                },
                wast::Instruction::I32ReinterpretF32 => {
                    current_f32_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64ConvertI32S => {
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i32_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::F64ConvertI32U => {
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i32_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::F64ConvertI64U => {
                    stack_sizes.pop();
                    stack_sizes.push(2);

                    current_i64_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::I32Clz => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    // no-op
                },
                wast::Instruction::I32Popcnt => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    // no-op
                },
                wast::Instruction::I64Clz => {
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    // no-op
                },
                wast::Instruction::I32Ctz => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                    // no-op
                },
                wast::Instruction::I64Ctz => {
                    stack_sizes.pop();
                    stack_sizes.push(2);
                    // no-op
                },
                /*
                 * Track block & loop starts/ends to minimize intermediate value req
                 */
                wast::Instruction::Block(b) => {
                },
                wast::Instruction::Loop(b) => {

                }
                wast::Instruction::End(id) => {
                },
                wast::Instruction::Select(_) => {

                },
                wast::Instruction::MemoryGrow(arg) => {
                    stack_sizes.pop();
                    stack_sizes.push(1);
                },
                wast::Instruction::MemorySize(arg) => {
                    stack_sizes.push(1);
                },
                wast::Instruction::Return => {

                },
                wast::Instruction::Br(idx) => {

                },
                wast::Instruction::BrIf(idx) => {

                },
                wast::Instruction::BrTable(table_idxs) => {

                },
                wast::Instruction::Unreachable => {
                },
                _ => panic!("Instruction {:?} not yet implemented (vstack-pass)", instruction)
            }
        }

        println!("max_i32_count: {}", max_i32_count);
        println!("max_i64_count: {}", max_i64_count);
        println!("max_f32_count: {}", max_f32_count);
        println!("max_f64_count: {}", max_f64_count);

        let mut i32_stack = vec![];
        for idx in 0..max_i32_count {
            i32_stack.push(format!("i32_{}", idx));
        }

        let mut i64_stack = vec![];
        for idx in 0..max_i64_count {
            i64_stack.push(format!("i64_{}", idx));
        }

        let mut f32_stack = vec![];
        for idx in 0..max_f32_count {
            f32_stack.push(format!("f32_{}", idx));
        }

        let mut f64_stack = vec![];
        for idx in 0..max_f64_count {
            f64_stack.push(format!("f64_{}", idx));
        }

        let mut local_types_converted = HashMap::new();
        for (name, ty) in local_param_types.iter() {
            local_types_converted.insert(name.clone(), StackCtx::convert_wast_types(ty));
        }

        let mut max_offset: u32 = 0;
        let mut max_offset_type_size = 0;
        for (name, offsets) in local_offsets {
            let param_found = match is_param.get(name) {
                Some(false) => false,
                Some(true) => true,
                _ => panic!("Local offset name not found (vstack)"),
            };

            if !param_found {
                if *offsets > max_offset {
                    max_offset = *offsets;
                    max_offset_type_size = writer_ctx.get_size_valtype(&local_param_types.get(name).unwrap());
                }
            }
        }

        StackCtx {
            i32_stack: i32_stack,
            i32_idx: 0,
            i64_stack: i64_stack,
            i64_idx: 0,
            f32_stack: f32_stack,
            f32_idx: 0,
            f64_stack: f64_stack,
            f64_idx: 0,
            stack_frame_offset: max_offset + max_offset_type_size,
            local_offsets: local_offsets.clone(),
            local_types: local_types_converted,
            param_offset: param_offset,
            // this is intentionally empty, because we actually track this during the *main* compilation pass
            stack_sizes: vec![]
        }
    }

    pub fn convert_wast_types(ty: &wast::ValType) -> StackType {
        match ty {
            wast::ValType::I32 => StackType::i32,
            wast::ValType::F32 => StackType::f32,
            wast::ValType::I64 => StackType::i64,
            wast::ValType::F64 => StackType::f64,
            _ => panic!("Unknown stack type (vstack)"),
        }
    }

    pub fn emit_intermediates(&self) -> String {
        let mut ret_str = String::from("");

        // emit the locals and parameters
        for (local_name, local_type) in self.local_types.iter() {
            match local_type {
                StackType::i32 => {
                    ret_str += &format!("\tuint {} = 0;\n", local_name);
                },
                StackType::i64 => {
                    ret_str += &format!("\tulong {} = 0;\n", local_name);
                },
                StackType::f32 => {
                    ret_str += &format!("\tfloat {} = 0.0;\n", local_name);
                },
                StackType::f64 => {
                    ret_str += &format!("\tdouble {} = 0.0;\n", local_name);
                }
            }
        }

        for intermediate in &self.i32_stack {
            ret_str += &format!("\tuint {} = 0;\n", intermediate);
        }

        for intermediate in &self.i64_stack {
            ret_str += &format!("\tulong {} = 0;\n", intermediate);
        }

        for intermediate in &self.f32_stack {
            ret_str += &format!("\tfloat {} = 0.0;\n", intermediate);
        }

        for intermediate in &self.f64_stack {
            ret_str += &format!("\tdouble {} = 0.0;\n", intermediate);
        }

        ret_str
    }

    /*
     * This initialization happens if !*is_calling == False (function being called for the first time)
     * In that case we read the value of parameters from the stack context. Locals are 0s by default.
     */
    pub fn emit_load_params(&self) -> String {
        let mut ret_str = String::from("");

        // Read each parameter into intermediate value
        for (local_name, local_type) in self.local_types.iter() {
            let offset: i32 = *self.local_offsets.get(local_name).unwrap() as i32 + self.param_offset;
            // only load parameters, locals are already zeroed out.
            if offset < 0 {
                let param_lookup_u32 = emit_read_u32(&format!("(ulong)(stack_u32+{}+{})",
                                                          offset, 
                                                          &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                          "(ulong)stack_u32",
                                                          "warp_idx");
                let param_lookup_u64 = emit_read_u64(&format!("(ulong)(stack_u32+{}+{})",
                                                          offset, 
                                                          &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                          "(ulong)stack_u32",
                                                          "warp_idx");
                match local_type {
                    StackType::i32 => {
                        ret_str += &format!("\t\t{} = {};\n", local_name, param_lookup_u32);
                    },
                    StackType::i64 => {
                        ret_str += &format!("\t\t{} = {};\n", local_name, param_lookup_u64);
                    },
                    StackType::f32 => {
                        ret_str += &format!("\t\t{} = {};\n", local_name, param_lookup_u32);
                    },
                    StackType::f64 => {
                        ret_str += &format!("\t\t{} = {};\n", local_name, param_lookup_u64);
                    }
                }
            }
        }
        ret_str
    }

    /*
     * Get the most recent intermediate value from the stack
     */
    pub fn vstack_pop(&mut self, t: StackType) -> String {
        match t {
            StackType::i32 => {
                self.i32_idx -= 1;
                format!("{}", self.i32_stack.get(self.i32_idx).unwrap())
            },
            StackType::i64 => {
                self.i64_idx -= 1;
                format!("{}", self.i64_stack.get(self.i64_idx).unwrap())
            },
            StackType::f32 => {
                self.f32_idx -= 1;
                format!("{}", self.f32_stack.get(self.f32_idx).unwrap())

            },
            StackType::f64 => {
                self.f64_idx -= 1;
                format!("{}", self.f64_stack.get(self.f64_idx).unwrap())
            },
        }
    }

    /*
     * Save the result of a computation to an intermediate value
     */
    pub fn vstack_alloc(&mut self, t: StackType) -> String {
        match t {
            StackType::i32 => {
                let alloc_val = self.i32_stack.get(self.i32_idx).unwrap();
                self.i32_idx += 1;
                format!("{}", alloc_val)
            },
            StackType::i64 => {
                let alloc_val = self.i64_stack.get(self.i64_idx).unwrap();
                self.i64_idx += 1;
                format!("{}", alloc_val)
            },
            StackType::f32 => {
                let alloc_val = self.f32_stack.get(self.f32_idx).unwrap();
                self.f32_idx += 1;
                format!("{}", alloc_val)
            },
            StackType::f64 => {
                let alloc_val = self.f64_stack.get(self.f64_idx).unwrap();
                self.f64_idx += 1;
                format!("{}", alloc_val)
            },
        }
    }

    /*
     * Peak the last register on the vstack, useful for unops
     */
    pub fn vstack_peak(&mut self, t: StackType) -> String {
        match t {
            StackType::i32 => {
                let alloc_val = self.i32_stack.get(self.i32_idx-1).unwrap();
                format!("{}", alloc_val)
            },
            StackType::i64 => {
                let alloc_val = self.i64_stack.get(self.i64_idx-1).unwrap();
                format!("{}", alloc_val)
            },
            StackType::f32 => {
                let alloc_val = self.f32_stack.get(self.f32_idx-1).unwrap();
                format!("{}", alloc_val)
            },
            StackType::f64 => {
                let alloc_val = self.f64_stack.get(self.f64_idx-1).unwrap();
                format!("{}", alloc_val)
            },
        }
    }

    pub fn stack_frame_size(&self) -> usize {
        self.i32_stack.len() + (self.i64_stack.len()*2) + self.f32_stack.len() + (self.f64_stack.len()*2)
    }

    /*
     * Generate the code to save the context of the current function
     * We can statically determine the minimum
     */
    pub fn save_context(&self) -> String {
        let mut ret_str = String::from("");

        // First, save the locals to the stack frame
        // TODO: track writes to locals at runtime to minimize gmem writes?
        for (local, ty) in self.local_types.iter() {
            let offset: i32 = *self.local_offsets.get(local).unwrap() as i32 + self.param_offset;
            match ty {
                StackType::i32 => {
                    ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    &local,
                                                                    "warp_idx"));
                },
                StackType::i64 => {
                    ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    &local,
                                                                    "warp_idx"));
                },
                StackType::f32 => {
                    ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    &local,
                                                                    "warp_idx"));
                },
                StackType::f64 => {
                    ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    &local,
                                                                    "warp_idx"));
                }
            }
        }

        // Now go through and save the intermediate values
        let mut intermediate_offset = 0;
        for idx in 0..self.i32_idx {
            ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)(stack_u32+{}+{}+{})",
                                                            self.stack_frame_offset, intermediate_offset,
                                                            &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                            "(ulong)stack_u32",
                                                            &self.i32_stack.get(idx).unwrap(),
                                                            "warp_idx"));
            intermediate_offset += 1;
        }

        for idx in 0..self.i64_idx {
            ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)(stack_u32+{}+{}+{})",
                                                            self.stack_frame_offset, intermediate_offset,
                                                            &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                            "(ulong)stack_u32",
                                                            &self.i64_stack.get(idx).unwrap(),
                                                            "warp_idx"));
            intermediate_offset += 2;
        }

        for idx in 0..self.f32_idx {
            ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)(stack_u32+{}+{}+{})",
                                                            self.stack_frame_offset, intermediate_offset,
                                                            &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                            "(ulong)stack_u32",
                                                            &self.f32_stack.get(idx).unwrap(),
                                                            "warp_idx"));
            intermediate_offset += 1;
        }

        for idx in 0..self.f64_idx {
            ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)(stack_u32+{}+{}+{})",
                                                            self.stack_frame_offset, intermediate_offset,
                                                            &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                            "(ulong)stack_u32",
                                                            &self.f64_stack.get(idx).unwrap(),
                                                            "warp_idx"));
            intermediate_offset += 2;
        }

        ret_str
    }

    /*
     * Generate the code to restore the context of the current function
     */
    pub fn restore_context(&self) -> String {
        let mut ret_str = String::from("");

        // First, load all locals from memory
        for (local, ty) in self.local_types.iter() {
            let offset: i32 = *self.local_offsets.get(local).unwrap() as i32 + self.param_offset;
            match ty {
                StackType::i32 => {
                    ret_str += &format!("\t{} = {};\n", local, &emit_read_u32(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    "warp_idx"));
                },
                StackType::i64 => {
                    ret_str += &format!("\t{} = {};\n", local, &emit_read_u64(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    "warp_idx"));
                },
                StackType::f32 => {
                    ret_str += &format!("\t{} = {};\n", local, &emit_read_u32(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    "warp_idx"));
                },
                StackType::f64 => {
                    ret_str += &format!("\t{} = {};\n", local, &emit_read_u64(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    "warp_idx"));
                }
            }
        }

        // Now restore the intermediate values
        let mut intermediate_offset = 0;
        for idx in 0..self.i32_idx {
            ret_str += &format!("\t{} = {};\n", &self.i32_stack.get(idx).unwrap(),
                                                &emit_read_u32(&format!("(ulong)(stack_u32+{}+{}+{})",
                                                            self.stack_frame_offset, intermediate_offset,
                                                            &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                            "(ulong)stack_u32",
                                                            "warp_idx"));
            intermediate_offset += 1;
        }

        for idx in 0..self.i64_idx {
            ret_str += &format!("\t{} = {};\n", &self.i64_stack.get(idx).unwrap(), 
                                                &emit_read_u64(&format!("(ulong)(stack_u32+{}+{}+{})",
                                                            self.stack_frame_offset, intermediate_offset,
                                                            &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                            "(ulong)stack_u32",
                                                            "warp_idx"));
            intermediate_offset += 2;
        }

        for idx in 0..self.f32_idx {
            ret_str += &format!("\t{} = {};\n", &self.f32_stack.get(idx).unwrap(),
                                                &emit_read_u32(&format!("(ulong)(stack_u32+{}+{}+{})",
                                                            self.stack_frame_offset, intermediate_offset,
                                                            &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                            "(ulong)stack_u32",
                                                            "warp_idx"));
            intermediate_offset += 1;
        }

        for idx in 0..self.f64_idx {
            ret_str += &format!("\t{} = {};\n", &self.f64_stack.get(idx).unwrap(), 
                                                &emit_read_u64(&format!("(ulong)(stack_u32+{}+{}+{})",
                                                            self.stack_frame_offset, intermediate_offset,
                                                            &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                            "(ulong)stack_u32",
                                                            "warp_idx"));
            intermediate_offset += 2;
        }

        ret_str
    }
}

