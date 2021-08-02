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
use crate::opencl_writer::get_func_result;

use wast::Index::Id;
use wast::Index::Num;
use wast::Instruction;
use wast::ValType;

use std::collections::{HashMap, HashSet};
use std::cmp::Ord;
use core::ops::Range;

#[derive(Debug, PartialEq, Clone)]
pub enum StackType {
    i32,
    i64,
    f32,
    f64
}

#[derive(Debug, Clone)]
pub struct StackSnapshot {
    i32_idx: usize,
    i64_idx: usize,
    f32_idx: usize,
    f64_idx: usize
}

impl StackSnapshot {
    pub fn from_current_ctx(ctx: &StackCtx) -> StackSnapshot {
        StackSnapshot {
            i32_idx: ctx.i32_idx,
            i64_idx: ctx.i64_idx,
            f32_idx: ctx.f32_idx,
            f64_idx: ctx.f64_idx,
        }
    }
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
    // offsets of intermediates from the stack frame offset
    intermediate_offsets: HashMap<String, u32>,
    local_offsets: HashMap<String, u32>,
    local_types: HashMap<String, StackType>,
    param_offset: i32,
    total_stack_types: Vec<StackType>,
    // Tracking loop sp state changes for unwinding the stack during br statements
    // We store the *sp modification in this stack, then pop this off when emitting end statements
    control_stack: Vec<u32>,
    control_stack_snapshots: Vec<StackSnapshot>,
    // store which locals are parameters (for emitting fastcalls)
    is_param: HashMap<String, bool>,
    tainted_loops: Vec<bool>,
    num_fn_calls: u32,
    num_hypercalls: u32,
}

impl<'a> StackCtx {
    /*
     * Parse a function and generate a stack context for it.
     * We can statically determine the maximum required amount of intermediate values
     */
    pub fn initialize_context(writer_ctx: &OpenCLCWriter, instructions: &Box<[Instruction<'a>]>, local_param_types: &HashMap<String, ValType>, local_offsets: &HashMap<String, u32>, is_param: &HashMap<String, bool>, fastcalls: HashSet<String>, param_offset: i32, indirect_call_len: u32, is_gpu: bool) -> StackCtx {
        let mut stack_sizes: Vec<StackType> = vec![];
        
        // Track which loops we can optimize for later
        // (is_loop, tainted), is_loop needed since we are also tracking blocks
        let mut control_stack: Vec<(bool, Option<StackType>)> = vec![];
        let mut tainted_loops: Vec<bool> = vec![];
        let mut loop_idx: usize = 0;

        /*
         * Needed to avoid edge case where we have an empty infinite loop
         */
        let mut empty_loop = false;

        // Track # function calls & indirect function calls
        // We do this so we can avoid another compiler pass to generate call return stubs
        let mut num_hypercalls = 0;
        let mut num_fn_calls = 0;

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

        // loop_idx tracks how many actively open loops there are, so we taint just those
        fn taint_open_loops(tainted_loops: &mut Vec<bool>, loop_idx: usize) -> () {
            let loop_len = tainted_loops.len().clone();
            for idx in 0..loop_idx {
                tainted_loops[loop_len - 1 - idx] = true;
            }
        }

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
            match instruction {
                wast::Instruction::Drop => {
                    stack_sizes.pop().unwrap();
                }
                wast::Instruction::I32Store(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 2;
                },
                wast::Instruction::I32Store8(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 2;
                },
                wast::Instruction::I64Store8(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_i64_count -= 1;
                },
                wast::Instruction::I64Store16(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_i64_count -= 1;
                },
                wast::Instruction::I32Store16(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 2;
                },
                wast::Instruction::I64Store32(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_i64_count -= 1;
                },
                wast::Instruction::I32Load(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                },
                wast::Instruction::I32Load8u(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                },
                wast::Instruction::I64Load16u(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I32Load16u(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                },
                wast::Instruction::I32Load16s(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                },
                wast::Instruction::I32Load8s(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                },
                wast::Instruction::I64Load8u(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64Load32u(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64Load32s(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64Load(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::F64Load(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_i32_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::F32Load(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_i32_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                },
                wast::Instruction::I64Store(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_i64_count -= 1;
                },
                wast::Instruction::F64Store(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_f64_count -= 1;
                },
                wast::Instruction::F32Store(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_f32_count -= 1;
                },
                /*
                 * As of right now we only support i32 globals anyways...
                 * TODO: for future support of globals, check for other types here
                 */
                wast::Instruction::GlobalGet(_idx) => {
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                    stack_sizes.push(StackType::i32);
                },
                wast::Instruction::GlobalSet(_idx) => {
                    current_i32_count -= 1;
                    stack_sizes.pop().unwrap();
                },
                wast::Instruction::I32Const(_val) => {
                    stack_sizes.push(StackType::i32);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64Const(_val) => {
                    stack_sizes.push(StackType::i64);
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::F32Const(_val) => {
                    stack_sizes.push(StackType::f32);
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                },
                wast::Instruction::F64Const(_val) => {
                    stack_sizes.push(StackType::f64);
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::LocalGet(idx) => {
                    match idx {
                        wast::Index::Id(id) => {
                            stack_sizes.push(StackCtx::convert_wast_types(local_param_types.get(&id.name().to_string()).unwrap()));

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
                            stack_sizes.push(StackCtx::convert_wast_types(local_param_types.get(&id).unwrap()));

                            update_by_valtype(local_param_types.get(&id).unwrap(),
                                                &mut current_i32_count, &mut max_i32_count,
                                                &mut current_i64_count, &mut max_i64_count,
                                                &mut current_f32_count, &mut max_f32_count,
                                                &mut current_f64_count, &mut max_f64_count);
                        },
                    }
                },
                wast::Instruction::LocalSet(idx) => {
                    stack_sizes.pop().unwrap();
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
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I32Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I64Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I32Sub => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I64Add => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::F64Max => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                },
                wast::Instruction::F64Min => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                },
                wast::Instruction::F32Max => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                },
                wast::Instruction::F32Min => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                },
                wast::Instruction::F64Add => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                },
                wast::Instruction::F64Sub => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                },
                wast::Instruction::F32Add => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                },
                wast::Instruction::F32Sub => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                },
                wast::Instruction::F32Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                },
                wast::Instruction::F64Div => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                },
                wast::Instruction::F32Div => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                },
                wast::Instruction::F64Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                },
                wast::Instruction::F32Trunc => {
                },
                wast::Instruction::F64Trunc => {
                },
                wast::Instruction::F64Neg => {
                },
                wast::Instruction::F32Neg => {
                },
                wast::Instruction::F64Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F32Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64Lt => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64Gt => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F32Gt => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F32Lt => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64Le => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64Ge => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F32Le => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F32Ge => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64LtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F32Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32TruncF64U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64DivU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I32Eqz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    // no-op
                },
                wast::Instruction::I64Eqz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i64_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32And => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I64And => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I32Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I32LtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I32LtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I64LtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32GtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I64GtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64GtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32GtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I32LeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I32LeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I64LeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64LeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32GeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I32GeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I64GeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64GeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I32Xor => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                },
                wast::Instruction::I32WrapI64 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64ExtendI32S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64ExtendI32U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
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
                                        // Taint loops that perform hypercalls
                                        taint_open_loops(&mut tainted_loops, loop_idx);

                                        // Track how many hypercalls we perform
                                        num_hypercalls += 1;

                                        match wasi_fn_name {
                                            &"fd_write"               => {
                                                current_i32_count -= 3;
                                            },
                                            &"proc_exit"              => {
                                                current_i32_count -= 1;
                                            },
                                            &"environ_sizes_get"      => {
                                                current_i32_count -= 1;
                                            },
                                            &"environ_get"            => {
                                                current_i32_count -= 1;
                                            },
                                            &"fd_prestat_get"         => {
                                                current_i32_count -= 1;
                                            },
                                            &"fd_prestat_dir_name"    => {
                                                current_i32_count -= 2;
                                            },
                                            &"random_get"             => {
                                                current_i32_count -= 1;
                                            },
                                            &"serverless_invoke"      => {
                                                current_i32_count -= 1;
                                            },
                                            &"serverless_response"    => {
                                                current_i32_count -= 2;
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
                        // Check the function name to see if it is a valid fastcall
                        // We only taint non-fastcalls
                        if !fastcalls.contains(id) {
                            taint_open_loops(&mut tainted_loops, loop_idx);
                            // Track how many regular function calls we perform
                            num_fn_calls += 1;
                        }

                        match writer_ctx.func_map.get(id) {
                            Some(_) => {
                                let func_type_signature = &writer_ctx.func_map.get(id).unwrap().ty;

                                match &func_type_signature.inline {
                                    // if we can find the type signature
                                    Some(res) => {
                                        for (_, _, ty) in res.params.iter() {
                                            stack_sizes.pop();
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
                                        for ty in res.results.iter() {
                                            stack_sizes.push(StackCtx::convert_wast_types(&ty));
                                            update_by_valtype(ty,
                                                                &mut current_i32_count, &mut max_i32_count,
                                                                &mut current_i64_count, &mut max_i64_count,
                                                                &mut current_f32_count, &mut max_f32_count,
                                                                &mut current_f64_count, &mut max_f64_count);
                                        }
                                    },
                                    // if we cannot find the type signature, we need to look it up to check for the param offset
                                    None => {
                                        let fn_type_id = match func_type_signature.index {
                                            Some(wast::Index::Id(id)) => id.name().to_string(),
                                            Some(wast::Index::Num(n, _)) => format!("t{}", n),
                                            None => format!(""),
                                        };
                            
                                        let function_type = writer_ctx.types.get(&fn_type_id);
                                        match function_type {
                                            Some(wast::TypeDef::Func(ft)) => {
                                                for (_, _, ty) in ft.params.iter() {
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
                                                for ty in ft.results.iter() {
                                                    stack_sizes.push(StackCtx::convert_wast_types(&ty));
                                                    update_by_valtype(ty,
                                                                        &mut current_i32_count, &mut max_i32_count,
                                                                        &mut current_i64_count, &mut max_i64_count,
                                                                        &mut current_f32_count, &mut max_f32_count,
                                                                        &mut current_f64_count, &mut max_f64_count);
                                                }
                                            },
                                            None => (),
                                            _ => panic!("Non-function type referenced from function (vstack)")
                                        };
                                    },
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
                    // Taint open loops
                    taint_open_loops(&mut tainted_loops, loop_idx);

                    // Track the number of function call stubs to generate
                    num_fn_calls += indirect_call_len;

                    // Check for types
                    match (call_indirect.ty.index.as_ref(), call_indirect.ty.inline.as_ref()) {
                        (Some(index), _) => {
                            // if we have an index, we need to look it up in the global structure
                            let type_index = match index {
                                Num(n, _) => format!("t{}", n),
                                Id(i) => i.name().to_string(),
                            };
                
                            let func_type = match writer_ctx.types.get(&type_index).unwrap() {
                                wast::TypeDef::Func(ft) => ft,
                                _ => panic!("Indirect call cannot have a type of something other than a func"),
                            };

                            // First, pop off the parameters
                            for (_, _, param_type) in func_type.params.iter() {
                                match param_type {
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
                                    _ => panic!("vstack missing valtype check in indirect func call")
                                }
                            }
                
                            // Next, push the result(s) back
                            for return_type in func_type.results.iter() {
                                update_by_valtype(return_type,
                                    &mut current_i32_count, &mut max_i32_count,
                                    &mut current_i64_count, &mut max_i64_count,
                                    &mut current_f32_count, &mut max_f32_count,
                                    &mut current_f64_count, &mut max_f64_count);
                            }
                        },
                        (_, Some(_inline)) => panic!("Inline types for call_indirect not implemented yet (vstack)"),
                        _ => (),
                    };                
                },
                wast::Instruction::I32Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                },
                wast::Instruction::I32Or => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                },
                wast::Instruction::I32ShrU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64ShrU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I32ShrS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                },
                wast::Instruction::I32Shl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                },
                wast::Instruction::I64Shl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I32DivU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                },
                wast::Instruction::I32DivS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                },
                wast::Instruction::I64DivS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I32RemU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                },
                wast::Instruction::I64RemU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I32RemS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                },
                wast::Instruction::I64RemS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I64ShrS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I64Xor => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I64Or => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I32Rotl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                },
                wast::Instruction::I64Rotl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I64Sub => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                },
                wast::Instruction::I64ReinterpretF64 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_f64_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::F64ReinterpretI64 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_i64_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::F32ReinterpretI32 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);

                    current_i32_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                },
                wast::Instruction::I32ReinterpretF32 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_f32_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64Ceil => {
                    // No-op
                },
                wast::Instruction::F32Ceil => {
                    // No-op
                },
                wast::Instruction::F64Floor => {
                    // No-op
                },
                wast::Instruction::F32Floor => {
                    // No-op
                },
                wast::Instruction::F64PromoteF32 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_f32_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::F32DemoteF64 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);

                    current_f64_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                },
                wast::Instruction::I32TruncF32U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_f32_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::I64TruncF32U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_f32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I64TruncF32S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_f32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                },
                wast::Instruction::I32TruncF32S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_f32_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::F64ConvertI32S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_i32_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::F64ConvertI32U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_i32_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::F32ConvertI32U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);

                    current_i32_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                },
                wast::Instruction::F32ConvertI32S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);

                    current_i32_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                },
                wast::Instruction::F64ConvertI64U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_i64_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::F64ConvertI64S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_i64_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                },
                wast::Instruction::F32Abs => {
                    // no-op
                },
                wast::Instruction::F64Abs => {
                    // no-op
                },
                wast::Instruction::I32Clz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                },
                wast::Instruction::I32Popcnt => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                },
                wast::Instruction::I64Clz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    // no-op
                },
                wast::Instruction::I32Ctz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                },
                wast::Instruction::I64Ctz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    // no-op
                },
                /*
                 * Track block & loop starts/ends to minimize intermediate value req
                 */
                wast::Instruction::Block(b) => {
                    // Get the type of the block
                    let block_type = get_func_result(&writer_ctx, &b.ty);
                    match block_type.clone() {
                        Some(stack_size) => {
                            stack_sizes.push(stack_size.clone());
                            update_by_valtype(&StackCtx::convert_stacktypes_valtype(&stack_size.clone()),
                                              &mut current_i32_count, &mut max_i32_count,
                                              &mut current_i64_count, &mut max_i64_count,
                                              &mut current_f32_count, &mut max_f32_count,
                                              &mut current_f64_count, &mut max_f64_count);
                        },
                        None => (),
                    };
                    control_stack.push((false, block_type));
                },
                wast::Instruction::Loop(b) => {
                    tainted_loops.push(false);
                    loop_idx += 1;
                    empty_loop = true;
                    let block_type = get_func_result(&writer_ctx, &b.ty);
                    match block_type.clone() {
                        Some(stack_size) => {
                            stack_sizes.push(stack_size.clone());
                            update_by_valtype(&StackCtx::convert_stacktypes_valtype(&stack_size.clone()),
                                              &mut current_i32_count, &mut max_i32_count,
                                              &mut current_i64_count, &mut max_i64_count,
                                              &mut current_f32_count, &mut max_f32_count,
                                              &mut current_f64_count, &mut max_f64_count);
                        },
                        None => (),
                    };
                    control_stack.push((true, block_type));
                    // We need to continue here to avoid resetting the empty_loop counter
                    continue;
                }
                wast::Instruction::End(_id) => {
                    // As we close loops, keep track so we don't taint them
                    let (is_loop, t) = control_stack.pop().unwrap();
                    if is_loop {
                        loop_idx -= 1;
                    }
                    // We have to pop the result value of the block (if we have one)
                    match t {
                        Some(stack_type) => {
                            match stack_type {
                                StackType::i32 => {
                                    current_i32_count -= 1;
                                },
                                StackType::i64 => {
                                    current_i64_count -= 1;
                                },
                                StackType::f32 => {
                                    current_f32_count -= 1;
                                },
                                StackType::f64 => {
                                    current_f64_count -= 1;
                                },
                            }
                            stack_sizes.pop();
                        },
                        None => (),
                    }
                },
                wast::Instruction::Select(_) => {
                    let _c = stack_sizes.pop().unwrap(); // c
                    let arg1 = stack_sizes.pop().unwrap();
                    let arg2 = stack_sizes.pop().unwrap();
                    if arg1 != arg2 {
                        panic!("Select must operate on two args of the same type (vstack)");
                    }
                    current_i32_count -= 1;
                    // depending on the arg1, arg2 vals we pop different types
                    match arg1 {
                        StackType::i32 => {
                            current_i32_count -= 1;
                            stack_sizes.push(StackType::i32);
                        },
                        StackType::i64 => {
                            current_i64_count -= 1;
                            stack_sizes.push(StackType::i64);
                        },
                        StackType::f32 => {
                            current_f32_count -= 1;
                            stack_sizes.push(StackType::f32);
                        },
                        StackType::f64 => {
                            current_f64_count -= 1;
                            stack_sizes.push(StackType::f64);
                        },
                    }
                },
                wast::Instruction::MemoryGrow(_arg) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::i32);
                    // no-op
                },
                wast::Instruction::MemorySize(_arg) => {
                    stack_sizes.push(StackType::i32);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                },
                wast::Instruction::Return => {
                },
                wast::Instruction::Br(_idx) => {
                    if empty_loop {
                        taint_open_loops(&mut tainted_loops, loop_idx);
                    }
                },
                wast::Instruction::BrIf(_idx) => {
                    stack_sizes.pop().unwrap();
                    current_i32_count -= 1;
                },
                wast::Instruction::BrTable(_table_idxs) => {
                    stack_sizes.pop().unwrap();
                    current_i32_count -= 1;
                },
                wast::Instruction::Unreachable => {
                    if !is_gpu {
                        num_hypercalls += 1;
                    }
                },
                _ => panic!("Instruction {:?} not yet implemented (vstack-pass)", instruction)
            }
            if empty_loop {
                empty_loop = false;
            }
        }

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

        let mut cloned_local_offsets: Vec<(String, u32)> = vec![];

        for (name, offset) in local_offsets.clone().iter() {
            cloned_local_offsets.push((name.to_string(), *offset))
        }

        // pop the first entry, set it as the starting max
        let (mut max_offset, mut max_offset_type_size): (u32, u32) = match cloned_local_offsets.pop() {
            Some ((name, offset)) => {
                (offset, writer_ctx.get_size_valtype(&local_param_types.get(&name).unwrap()))
            },
            None => {
                (0, 0)
            }
        };

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

        // For each intermediate, map it to an offset
        let mut intermediate_offsets: HashMap<String, u32> = HashMap::new();
        let mut curr_offset = 0;
        for idx in i32_stack.clone() {
            intermediate_offsets.insert(idx, curr_offset);
            curr_offset += 1;
        }
        for idx in i64_stack.clone() {
            intermediate_offsets.insert(idx, curr_offset);
            curr_offset += 2;
        }
        for idx in f32_stack.clone() {
            intermediate_offsets.insert(idx, curr_offset);
            curr_offset += 1;
        }
        for idx in f64_stack.clone() {
            intermediate_offsets.insert(idx, curr_offset);
            curr_offset += 2;
        }

        // For each loop that we can't optimize, we need to generate a function call stub
        num_fn_calls += tainted_loops.iter().filter( |x| { **x == true }).collect::<Vec<&bool>>().len() as u32;

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
            intermediate_offsets: intermediate_offsets,
            local_offsets: local_offsets.clone(),
            local_types: local_types_converted,
            param_offset: param_offset,
            total_stack_types: vec![],
            control_stack: vec![],
            control_stack_snapshots: vec![],
            tainted_loops: tainted_loops,
            num_fn_calls: num_fn_calls,
            num_hypercalls: num_hypercalls,
            is_param: is_param.clone()
        }
    }

    /*
     * Check to see if we can optimize the currently selected loop
     */
    pub fn is_loop_tainted(&mut self, loop_idx: usize) -> bool {
        self.tainted_loops[loop_idx]
    }

    /*
     * Get the lengths of the reentry stubs from the vstack pass
     */
    pub fn get_reentry_stub_lengths(&self) -> (u32, u32) {
        (self.num_fn_calls, self.num_hypercalls)
    }

    /*
     * When entering a loop block, track how much we increase *sp by
     */
    pub fn vstack_push_stack_info(&mut self, stack_inc: u32) -> () {
        // Look at the previous control stack entry (if any)
        match self.control_stack.clone().last() {
            Some(e) => {
                // If there is a previous stack frame, compute the delta
                self.control_stack.push(stack_inc - e);
            },
            None => {
                // Else, push the total stack inc
                self.control_stack.push(stack_inc);
            }
        }
    }

    pub fn vstack_pop_stack_info(&mut self) -> () {
        self.control_stack.pop();
    }

    pub fn vstack_push_stack_frame(&mut self) -> () {
        self.control_stack_snapshots.push(StackSnapshot::from_current_ctx(self));
    }

    pub fn vstack_pop_stack_frame(&mut self) -> () {
        let stack_frame_unwind = self.control_stack_snapshots.pop().unwrap();
        self.i32_idx = stack_frame_unwind.i32_idx;
        self.i64_idx = stack_frame_unwind.i64_idx;
        self.f32_idx = stack_frame_unwind.f32_idx;
        self.f64_idx = stack_frame_unwind.f64_idx;
    }

    /*
     * When executing a br instruction that exits a loop
     */
    pub fn vstack_get_stack_delta(&mut self, nested_stack_count: u32) -> u32 {
        let mut ret_val: u32 = 0;
        let mut control_stack_copy = self.control_stack.clone();
        control_stack_copy.reverse();
        for (val, idx) in control_stack_copy.iter().zip(0..control_stack_copy.len() as u32) {
            if idx == nested_stack_count {
                break;
            }
            ret_val += val;
        }

        ret_val
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

    pub fn convert_stacktypes_valtype(ty: &StackType) -> wast::ValType {
        match ty {
            StackType::i32 => wast::ValType::I32,
            StackType::i64 => wast::ValType::I64,
            StackType::f32 => wast::ValType::F32,
            StackType::f64 => wast::ValType::F64,
            _ => panic!("Unknown stack type (convert_stacktypes_valtype)"),
        }
    }

    pub fn emit_cache_array(&self, is_fastcall: bool) -> String {
        let mut max_offset: u32 = 0;
        for (_local, offset) in self.local_offsets.iter() {
            if *offset > max_offset {
                max_offset = *offset;
            }
        }
        if self.local_offsets.len() > 0  && !is_fastcall {
            format!("\tuchar local_cache[{}] = {{ 0 }};\n", max_offset+1)
        } else {
            String::from("")
        }
    }

    pub fn emit_fastcall_header(&self) -> String {
        let mut ret_str = String::from("(");


        let mut params = self.local_types.iter().collect::<Vec<(&String, &StackType)>>();
        params.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (local_name, local_type) in params.clone() {
            let param_found = match self.is_param.get(local_name) {
                Some(false) => false,
                Some(true) => true,
                _ => panic!("Local offset name not found (vstack)"),
            };
            if param_found {
                match local_type {
                    StackType::i32 => {
                        ret_str += &format!("uint {}, ", local_name);
                    },
                    StackType::i64 => {
                        ret_str += &format!("ulong {}, ", local_name);
                    },
                    StackType::f32 => {
                        ret_str += &format!("float {}, ", local_name);
                    },
                    StackType::f64 => {
                        ret_str += &format!("double {}, ", local_name);
                    }
                }
            }
        }

        // now add the other info we need for fastcalls
        ret_str += &format!("global uint *heap_u32, global uint *current_mem_size, global uint *max_mem_size, global uint *globals_buffer, ulong warp_idx) {{\n");

        ret_str
    }

    pub fn emit_intermediates(&self, is_fastcall: bool) -> String {
        let mut ret_str = String::from("");

        // emit the locals and parameters
        for (local_name, local_type) in self.local_types.iter() {
            let param_found = match self.is_param.get(local_name) {
                Some(false) => false,
                Some(true) => true,
                _ => panic!("Local offset name not found (vstack)"),
            };
            if !is_fastcall || !param_found {
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
    pub fn emit_load_params(&self, debug_call_print: bool) -> String {
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
                        if debug_call_print {
                            ret_str += &format!("\t\tprintf(\"param {}, value: %d\\n\", {});\n", local_name, local_name);
                        }
                    },
                    StackType::i64 => {
                        ret_str += &format!("\t\t{} = {};\n", local_name, param_lookup_u64);
                        if debug_call_print {
                            ret_str += &format!("\t\tprintf(\"param {}, value: %d\\n\", {});\n", local_name, local_name);
                        }
                    },
                    StackType::f32 => {
                        ret_str += &format!("\t\t{{\n");
                        ret_str += &format!("\t\t\tuint temp = {};\n", param_lookup_u32);
                        ret_str += &format!("\t\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(uint));\n", local_name);
                        ret_str += &format!("\t\t}}\n");
                        if debug_call_print {
                            ret_str += &format!("\t\tprintf(\"param {}, value: %f\\n\", {});\n", local_name, local_name);
                        }
                    },
                    StackType::f64 => {
                        ret_str += &format!("\t\t{{\n");
                        ret_str += &format!("\t\t\tulong temp = {};\n", param_lookup_u64);
                        ret_str += &format!("\t\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(ulong));\n", local_name);
                        ret_str += &format!("\t\t}}\n");
                        if debug_call_print {
                            ret_str += &format!("\t\tprintf(\"param {}, value: %f\\n\", {});\n", local_name, local_name);
                        }
                    }
                }
            }
        }
        ret_str
    }

    /*
     * Needed for emit_fn_call / indirect calls
     * We need to pop the three most recent intermediates in order for proper function calls
     */
    pub fn vstack_pop_any(&mut self) -> (String, StackType) {
        let pop_type = self.vstack_peak_type(0);
        let intermediate_name = self.vstack_pop(pop_type.clone());
        (intermediate_name, pop_type)
    }

    /*
     * Get the most recent intermediate value from the stack
     */
    pub fn vstack_pop(&mut self, t: StackType) -> String {
        self.total_stack_types.pop().unwrap();
        match t {
            StackType::i32 => {
                if self.i32_idx == 0 {
                    panic!("vstack_pop failed to pop i32 register: {:?}", self);
                }
                self.i32_idx -= 1;
                format!("{}", self.i32_stack.get(self.i32_idx).unwrap())
            },
            StackType::i64 => {
                if self.i64_idx == 0 {
                    panic!("vstack_pop failed to pop i64 register: {:?}", self);
                }
                self.i64_idx -= 1;
                format!("{}", self.i64_stack.get(self.i64_idx).unwrap())
            },
            StackType::f32 => {
                if self.f32_idx == 0 {
                    panic!("vstack_pop failed to pop f32 register: {:?}", self);
                }
                self.f32_idx -= 1;
                format!("{}", self.f32_stack.get(self.f32_idx).unwrap())

            },
            StackType::f64 => {
                if self.f64_idx == 0 {
                    panic!("vstack_pop failed to pop f64 register: {:?}", self);
                }
                self.f64_idx -= 1;
                format!("{}", self.f64_stack.get(self.f64_idx).unwrap())
            },
        }
    }

    /*
     * Save the result of a computation to an intermediate value
     */
    pub fn vstack_alloc(&mut self, t: StackType) -> String {
        self.total_stack_types.push(t.clone());
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
     * Peak the registers on the vstack, useful for unops
     */
    pub fn vstack_peak(&mut self, t: StackType, idx: usize) -> String {
        match t {
            StackType::i32 => {
                let alloc_val = self.i32_stack.get(self.i32_idx-1-idx).unwrap();
                format!("{}", alloc_val)
            },
            StackType::i64 => {
                let alloc_val = self.i64_stack.get(self.i64_idx-1-idx).unwrap();
                format!("{}", alloc_val)
            },
            StackType::f32 => {
                let alloc_val = self.f32_stack.get(self.f32_idx-1-idx).unwrap();
                format!("{}", alloc_val)
            },
            StackType::f64 => {
                let alloc_val = self.f64_stack.get(self.f64_idx-1-idx).unwrap();
                format!("{}", alloc_val)
            },
        }
    }

    /*
     * Check if the vstack is empty, needed for function_unwind w/unreachable ending
     * The type of unreachable is weird:
     *   (func $__rust_start_panic (type $t5) (param $p0 i32) (result i32)
     *       unreachable
     *       unreachable) 
     * 
     * Is valid WASM code! So in function unwind we just check if the stack is empty
     * and return -1 if it is.
     */
    pub fn vstack_is_empty(&mut self, t: StackType) -> bool {
        match t {
            StackType::i32 => {
                self.i32_idx == 0
            },
            StackType::i64 => {
                self.i64_idx == 0
            },
            StackType::f32 => {
                self.f32_idx == 0
            },
            StackType::f64 => {
                self.f64_idx == 0
            },
        }
    }

    // Get the Xth type from the top of the stack
    pub fn vstack_peak_type(&mut self, idx: usize) -> StackType {
        self.total_stack_types[self.total_stack_types.len()-1-idx].clone()
    }

    // get the size of the current stack frame
    pub fn stack_frame_size(&self) -> usize {
        self.i32_idx + (self.i64_idx*2) + self.f32_idx + (self.f64_idx*2)
    }

    // get the max possible size of a stack context for the current function
    pub fn max_stack_frame_size(&self) -> usize {
        self.i32_stack.len() + (self.i64_stack.len()*2) + self.f32_stack.len() + (self.f64_stack.len()*2)
    }

    pub fn generate_intermediate_ranges(&self) -> (Range<usize>, Range<usize>, Range<usize>, Range<usize>) {
        match self.control_stack_snapshots.last() {
            Some(snap) => {

                let i32_range = snap.i32_idx..self.i32_idx;
                let i64_range = snap.i64_idx..self.i64_idx;
                let f32_range = snap.f32_idx..self.f32_idx;
                let f64_range = snap.f64_idx..self.f64_idx;

                (i32_range, i64_range, f32_range, f64_range)
            },
            // If no stack frames pushed, then we have the easy case
            None => (0..self.i32_idx, 0..self.i64_idx, 0..self.f32_idx, 0..self.f64_idx),
        }
    }


    /*
     * Generate the code to save the context of the current function
     * We can statically determine the minimum
     */
    pub fn save_context(&self, save_locals_only: bool) -> String {
        let mut ret_str = String::from("");

        // First, save the locals to the stack frame
        for (local, ty) in self.local_types.iter() {
            let cache_idx: u32 = *self.local_offsets.get(local).unwrap();
            let offset: i32 = *self.local_offsets.get(local).unwrap() as i32 + self.param_offset;
            match ty {
                StackType::i32 => {
                    ret_str += &format!("\tif (local_cache[{}]) {};\n", cache_idx, &emit_write_u32(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    &local,
                                                                    "warp_idx"));
                },
                StackType::i64 => {
                    ret_str += &format!("\tif (local_cache[{}]) {};\n", cache_idx, &emit_write_u64(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    &local,
                                                                    "warp_idx"));
                },
                StackType::f32 => {
                    ret_str += &format!("\tif (local_cache[{}]) {{\n", cache_idx);
                    ret_str += &format!("\t\tuint temp = 0;\n");
                    ret_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(float));\n", local);
                    ret_str += &format!("\t\t{};\n", &emit_write_u32(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    "temp",
                                                                    "warp_idx"));
                    ret_str += &format!("\t}}\n");
                },
                StackType::f64 => {
                    ret_str += &format!("\tif (local_cache[{}]) {{\n", cache_idx);
                    ret_str += &format!("\t\tulong temp = 0;\n");
                    ret_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n", local);
                    ret_str += &format!("\t\t{};\n", &emit_write_u64(&format!("(ulong)(stack_u32+{}+{})",
                                                                    offset, 
                                                                    &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                    "(ulong)stack_u32",
                                                                    "temp",
                                                                    "warp_idx"));
                    ret_str += &format!("\t}}\n");
                }
            }
            // Reset the local cache after saving locals to the stack, to avoid duplicate writes
            ret_str += &format!("\tlocal_cache[{}] = 0;\n", cache_idx);
        }

        // Now go through and save the intermediate values
        if !save_locals_only {
            // We only want to save the intermediates on our current stack frame
            let (i32_range, i64_range, f32_range, f64_range) = self.generate_intermediate_ranges();
            let sfp_val = emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx");

            /*
             * The ctx saving structure is:
             *         (fn start, sfp pointer)
             * |  <parameters>  | <locals> | <intermediates> ....
             * 
             */
            for idx in i32_range {
                let i_name = self.i32_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();
                ret_str += &format!("\t{};\n", &emit_write_u32(&format!("(ulong)(stack_u32+{}+{}+{})", sfp_val, self.stack_frame_offset, i_name_offset),
                                                                "(ulong)stack_u32",
                                                                &i_name,
                                                                "warp_idx"));
            }
    
            for idx in i64_range {
                let i_name = self.i64_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();
                ret_str += &format!("\t{};\n", &emit_write_u64(&format!("(ulong)(stack_u32+{}+{}+{})", sfp_val, self.stack_frame_offset, i_name_offset),
                                                                "(ulong)stack_u32",
                                                                &i_name,
                                                                "warp_idx"));
            }
    
            for idx in f32_range {
                let i_name = self.f32_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();

                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tuint temp = 0;\n");
                ret_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(float));\n", &i_name);
                ret_str += &format!("\t\t{};\n", &emit_write_u32(&format!("(ulong)(stack_u32+{}+{}+{})", sfp_val, self.stack_frame_offset, i_name_offset),
                                                                "(ulong)stack_u32",
                                                                "temp",
                                                                "warp_idx"));
                ret_str += &format!("\t}}\n");
            }
    
            for idx in f64_range {
                let i_name = self.f64_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();

                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tulong temp = 0;\n");
                ret_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n", &i_name);
                ret_str += &format!("\t\t{};\n", &emit_write_u64(&format!("(ulong)(stack_u32+{}+{}+{})", sfp_val, self.stack_frame_offset, i_name_offset),
                                                                "(ulong)stack_u32",
                                                                "temp",
                                                                "warp_idx"));
                ret_str += &format!("\t}}\n");
            }
        }

        // allocate space for saving the context on *real* stack
        // only do this if we have to save intermediate values at all

        /*
        if !save_locals_only {
            // Get the most recent control stack 
            ret_str += &format!("\t*sp += {};\n", intermediate_offset);
        }
        */

        ret_str
    }

    /*
     * Generate the code to restore the context of the current function
     */
    pub fn restore_context(&self, restore_locals_only: bool, restore_intermediates_only: bool) -> String {
        let mut ret_str = String::from("");

        // First, load all locals from memory
        if !restore_intermediates_only {
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
                        ret_str += &format!("\t{{\n");
                        ret_str += &format!("\t\tuint temp = {};\n", &emit_read_u32(&format!("(ulong)(stack_u32+{}+{})",
                                                                                        offset, 
                                                                                        &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                                        "(ulong)stack_u32",
                                                                                        "warp_idx"));
                        ret_str += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(float));\n", local);
                        ret_str += &format!("\t}}\n");
                    },
                    StackType::f64 => {
                        ret_str += &format!("\t{{\n");
                        ret_str += &format!("\t\tulong temp = {};\n", &emit_read_u64(&format!("(ulong)(stack_u32+{}+{})",
                                                                                        offset, 
                                                                                        &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx")),
                                                                                        "(ulong)stack_u32",
                                                                                        "warp_idx"));
                        ret_str += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(double));\n", local);
                        ret_str += &format!("\t}}\n");
                    }
                }
            }
        }
        
        if !restore_locals_only {
            // Now restore the intermediate values

            let (i32_range, i64_range, f32_range, f64_range) = self.generate_intermediate_ranges();
            let sfp_val = emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx");

            // Restore the stack context
            /*
            let mut stack_frame_size = 0;
            for _ in i32_range.clone() {
                stack_frame_size += 1;
            }
            for _ in i64_range.clone() {
                stack_frame_size += 2;
            }
            for _ in f32_range.clone() {
                stack_frame_size += 1
            }
            for _ in f64_range.clone() {
                stack_frame_size += 2;
            }
            ret_str += &format!("\t*sp -= {};\n", stack_frame_size);
            */

            for idx in i32_range {
                let i_name = self.i32_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();

                ret_str += &format!("\t{} = {};\n", &i_name,
                                                    &emit_read_u32(&format!("(ulong)(stack_u32+{}+{}+{})", sfp_val, self.stack_frame_offset, i_name_offset),
                                                                "(ulong)stack_u32",
                                                                "warp_idx"));
            }

            for idx in i64_range {
                let i_name = self.i64_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();

                ret_str += &format!("\t{} = {};\n", &i_name, 
                                                    &emit_read_u64(&format!("(ulong)(stack_u32+{}+{}+{})", sfp_val, self.stack_frame_offset, i_name_offset),
                                                                "(ulong)stack_u32",
                                                                "warp_idx"));
            }

            for idx in f32_range {
                let i_name = self.f32_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();

                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tuint temp = {};\n", &emit_read_u32(&format!("(ulong)(stack_u32+{}+{}+{})", sfp_val, self.stack_frame_offset, i_name_offset),
                                                                                "(ulong)stack_u32",
                                                                                "warp_idx"));
                ret_str += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(float));\n", &i_name);
                ret_str += &format!("\t}}\n");
            }

            for idx in f64_range {
                let i_name = self.f64_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();

                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tulong temp = {};\n", &emit_read_u64(&format!("(ulong)(stack_u32+{}+{}+{})", sfp_val, self.stack_frame_offset, i_name_offset),
                                                                                "(ulong)stack_u32",
                                                                                "warp_idx"));
                ret_str += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(double));\n", &i_name);
                ret_str += &format!("\t}}\n");
            }
        }

        ret_str
    }
}

