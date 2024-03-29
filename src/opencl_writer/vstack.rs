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

use crate::opencl_writer::emit_read_u32_aligned;
use crate::opencl_writer::emit_read_u32_fast;
use crate::opencl_writer::emit_read_u64_aligned;
use crate::opencl_writer::emit_write_u32_aligned;
use crate::opencl_writer::emit_write_u32_fast;
use crate::opencl_writer::emit_write_u64_aligned;
use crate::opencl_writer::get_func_result;
use crate::opencl_writer::OpenCLCWriter;
use crate::opencl_writer::WASI_SNAPSHOT_PREVIEW1;

use wast::core::*;
use wast::token::Index;
use wast::token::Index::*;

use crate::opencl_writer::format_fn_name;
use core::ops::Range;
use std::cmp::Ord;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

#[derive(Debug, PartialEq, Clone)]
pub enum StackType {
    i32,
    i64,
    f32,
    f64,
    u128,
}

#[derive(Debug, PartialEq, Clone)]
enum ControlStackVStackTypes {
    Block,
    If,
    Loop,
}

#[derive(Debug, Clone)]
pub struct StackSnapshot {
    i32_idx: usize,
    i64_idx: usize,
    f32_idx: usize,
    f64_idx: usize,
    u128_idx: usize,
    // Virtual stack snapshots are used for tracking the stacks of blocks that don't save/restore ctx
    is_virtual: bool,
    // Track stack types as well
    stack_types: Vec<StackType>,
}

impl StackSnapshot {
    pub fn from_current_ctx(ctx: &StackCtx, is_virtual: bool) -> StackSnapshot {
        StackSnapshot {
            i32_idx: ctx.i32_idx,
            i64_idx: ctx.i64_idx,
            f32_idx: ctx.f32_idx,
            f64_idx: ctx.f64_idx,
            u128_idx: ctx.u128_idx,
            is_virtual: is_virtual,
            stack_types: ctx.total_stack_types.clone(),
        }
    }
}

/*
 * The stacks are generated during the first compilation pass
 * Indicies are tracked during execution
 */
#[derive(Debug)]
pub struct StackCtx {
    // Useful in debugging compiler crashes during vstack pop
    _curr_fn_name: String,
    // The last element of this Vec points at the current stack frame we are in
    // 0 -> the function level stack frame, all subsequent frames are loops
    stack_frame_idx: u32,
    stack_frame_stack: Vec<u32>,
    // The context map of restore points
    restore_context_map: HashMap<u32, HashSet<String>>,
    // The context map of save points
    save_context_map: HashMap<u32, HashSet<String>>,
    // Track each intermediate type separately
    i32_stack: Vec<String>,
    i32_idx: usize,
    i64_stack: Vec<String>,
    i64_idx: usize,
    f32_stack: Vec<String>,
    f32_idx: usize,
    f64_stack: Vec<String>,
    f64_idx: usize,
    u128_stack: Vec<String>,
    u128_idx: usize,
    // Local & Parameter information
    stack_frame_offset: u32,
    // offsets of intermediates from the stack frame offset
    intermediate_offsets: HashMap<String, u32>,
    local_offsets: HashMap<String, u32>,
    local_types: HashMap<String, StackType>,
    moved_locals: HashSet<String>,
    demoted_intermediates: HashSet<String>,
    local_cache_size: u32,
    param_offset: i32,
    total_stack_types: Vec<StackType>,
    // Tracking loop sp state changes for unwinding the stack during br statements
    // We store the *sp modification in this stack, then pop this off when emitting end statements
    control_stack: Vec<u32>,
    control_stack_snapshots: Vec<StackSnapshot>,
    // store which locals are parameters (for emitting fastcalls)
    is_param: HashMap<String, bool>,
    tainted_loops: Vec<bool>,
    if_else_branches: Vec<bool>,
    num_fn_calls: u32,
    num_hypercalls: u32,
    called_fastcalls: HashSet<String>,
    max_emitted_context: u32,
    // Track how many optimized loops we have currently open
    // This is used to track when we should emit stack saves/restores
    opt_loop_tracking: Vec<bool>,
    // Track if we encounter any hcalls / call_indirects / non-opt calls
    // If we don't, then we don't have to emit any context saving/restoring at all
    fastcall_opt_possible: bool,
    // Track the memory interleaving for emitting optimized context saving ops
    interleave: u32,
    // Track which indirect calls should be optimized (and only emit fastcalls)
    indirect_call_map: HashMap<u32, bool>,
}

type ControlStackType = Vec<(
    String,
    Option<StackType>,
    ControlStackVStackTypes,
    Vec<StackType>,
    u32,
    u32,
    u32,
    u32,
    u32,
)>;

impl<'a> StackCtx {
    /*
     * Parse a function and generate a stack context for it.
     * We can statically determine the maximum required amount of intermediate values
     */
    pub fn initialize_context(
        writer_ctx: &OpenCLCWriter,
        instructions: &Box<[Instruction<'a>]>,
        local_param_types: &HashMap<String, ValType>,
        local_offsets: &HashMap<String, u32>,
        is_param: &HashMap<String, bool>,
        fastcalls: HashSet<String>,
        param_offset: i32,
        indirect_call_mapping: &HashMap<u32, &Index>,
        curr_fn_name: String,
        reduction_size: &mut u32,
        local_work_group: usize,
        interleave: u32,
        is_gpu: bool,
        unsafe_writes: bool,
    ) -> StackCtx {
        let mut indirect_call_map: HashMap<u32, bool> = HashMap::new();
        let mut indirect_call_count = 0;

        let mut stack_sizes: Vec<StackType> = vec![];
        let mut is_fastcall = true;

        // Track which loops we can optimize for later
        // (is_loop, tainted), is_loop needed since we are also tracking blocks
        // We also track the current i32, i64, f32, f64 values and reset them at the end of blocks
        let mut control_stack: ControlStackType = vec![];
        let mut tainted_loops: Vec<bool> = vec![];
        let mut open_loop_stack: Vec<usize> = vec![];
        let mut loop_idx: usize = 0;

        // Track which if blocks have "else" blocks
        // control_stack tracks the operators
        let mut if_else_branches: Vec<bool> = vec![];
        let mut if_else_idx_stack: Vec<usize> = vec![];
        let mut if_idx: usize = 0;
        /*
         * Needed to avoid edge case where we have an empty infinite loop
         */
        let mut empty_loop = false;

        /*
         * Liveness analysis for locals:
         *
         * We track reads/writes to locals for minimizing the size of context save/restore operations.
         *
         * We count the following as save points for writes:
         * 1) Func calls pre-call (indirect, hyper, regular - non fastcall)
         * 2) Loop start (pre-label)
         * 3) branches targeting loop starts
         *  3.1) Note: We don't actually have to handle this case -
         *       the locals we save here = the locals we reload at loop start post label.
         *       All other locals are saved before the loop starts
         *
         * We count the following as restore points for reads:
         * 1) Func calls pre-call (indirect, hyper, regular - non fastcall)
         * 2) Loop start (post-label)
         * 3) Loop end
         * 4) Fn start (this isn't a real case, it just simplifies our liveness analysis code)
         *
         *
         * As we encounter Loops/Calls we push the locals for each save/restore point we encounter
         */
        let mut read_locals: HashSet<String> = HashSet::new();
        let mut write_locals: HashSet<String> = HashSet::new();

        /*
         * We track the locals by stack frame, starting with the top level of the function
         *
         * For each stack frame we keep 2 vectors of lists of locals (save & restore points)
         *
         * We save the current locals at the start of the next stack frame, and restore locals at end statements
         *
         * Other save points include function calls & branches targeting
         */
        let mut restore_context_map: HashMap<u32, HashSet<String>> = HashMap::new();
        let mut save_context_map: HashMap<u32, HashSet<String>> = HashMap::new();

        // A mapping of the range of nested blocks for a loop
        let mut loop_nested_context_map: HashMap<u32, u32> = HashMap::new();
        // Map each nested loop to its idx in the tainted loop tracking structure
        let mut context_map_tainted_loop_map: HashMap<u32, u32> = HashMap::new();

        // Track the active stack frames
        let mut curr_ctx = vec![];
        let mut curr_ctx_idx = 1;

        // push fn_start as the default stack frame
        curr_ctx.push(0);

        // Track the locals that have been read from in a stack frame
        let mut read_locals_stack: Vec<HashSet<String>> = vec![];
        let mut write_locals_stack: Vec<HashSet<String>> = vec![];

        // Track all possible loop nestings
        // We need to do this to nest restore points in loops properly
        let mut nested_loops: Vec<Vec<u32>> = vec![];

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

        let mut current_u128_count: u32 = 0;
        let mut max_u128_count: u32 = 0;

        let mut called_fastcalls: HashSet<String> = HashSet::new();

        // loop_idx tracks how many actively open loops there are, so we taint just those
        fn taint_open_loops(tainted_loops: &mut Vec<bool>, open_loop_stack: Vec<usize>) -> () {
            for loop_idx in open_loop_stack {
                tainted_loops[loop_idx] = true;
            }
        }

        fn update_counter(curr_value: &mut u32, max_value: &mut u32) -> () {
            *curr_value += 1;
            if *curr_value > *max_value {
                *max_value = *curr_value;
            }
        }

        fn update_by_valtype(
            valtype: &ValType,
            curr_value_i32: &mut u32,
            max_value_i32: &mut u32,
            curr_value_i64: &mut u32,
            max_value_i64: &mut u32,
            curr_value_f32: &mut u32,
            max_value_f32: &mut u32,
            curr_value_f64: &mut u32,
            max_value_f64: &mut u32,
            curr_value_u128: &mut u32,
            max_value_u128: &mut u32,
        ) -> () {
            match valtype {
                ValType::I32 => {
                    update_counter(curr_value_i32, max_value_i32);
                }
                ValType::F32 => {
                    update_counter(curr_value_f32, max_value_f32);
                }
                ValType::I64 => {
                    update_counter(curr_value_i64, max_value_i64);
                }
                ValType::F64 => {
                    update_counter(curr_value_f64, max_value_f64);
                }
                ValType::V128 => {
                    update_counter(curr_value_u128, max_value_u128);
                }
                _ => panic!("vstack update by valtype error"),
            }
        }

        // for each instr,
        for instruction in instructions.iter() {
            //dbg!(&instruction);
            match instruction {
                Instruction::Nop => {
                    // No-op
                }
                Instruction::Drop => {
                    stack_sizes.pop().unwrap();
                }
                Instruction::I32Store(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 2;
                }
                Instruction::I32Store8(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 2;
                }
                Instruction::I64Store8(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_i64_count -= 1;
                }
                Instruction::I64Store16(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_i64_count -= 1;
                }
                Instruction::I32Store16(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 2;
                }
                Instruction::I64Store32(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_i64_count -= 1;
                }
                Instruction::V128Load(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::u128);
                    // no-op
                    current_i32_count -= 1;
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                }
                Instruction::V128Load64Zero(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::u128);
                    // no-op
                    current_i32_count -= 1;
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                }
                Instruction::I32Load(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                }
                Instruction::I32Load8u(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                }
                Instruction::I64Load16u(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I64Load16s(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I32Load16u(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                }
                Instruction::I32Load16s(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                }
                Instruction::I32Load8s(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                }
                Instruction::I64Load8u(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I64Load8s(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I64Load32u(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I64Load32s(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I64Load(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::F64Load(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_i32_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                }
                Instruction::F32Load(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_i32_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                }
                Instruction::I64Store(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_i64_count -= 1;
                }
                Instruction::F64Store(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_f64_count -= 1;
                }
                Instruction::F32Store(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_f32_count -= 1;
                }
                Instruction::V128Store(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 1;
                    current_u128_count -= 1;
                }
                Instruction::MemoryFill(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 3;
                }
                Instruction::MemoryCopy(_memarg) => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.pop();
                    current_i32_count -= 3;
                }
                /*
                 * As of right now we only support i32 globals anyways...
                 * TODO: for future support of globals, check for other types here
                 */
                Instruction::GlobalGet(_idx) => {
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                    stack_sizes.push(StackType::i32);
                }
                Instruction::GlobalSet(_idx) => {
                    current_i32_count -= 1;
                    stack_sizes.pop().unwrap();
                }
                Instruction::I32Const(_val) => {
                    stack_sizes.push(StackType::i32);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64Const(_val) => {
                    stack_sizes.push(StackType::i64);
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::F32Const(_val) => {
                    stack_sizes.push(StackType::f32);
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                }
                Instruction::F64Const(_val) => {
                    stack_sizes.push(StackType::f64);
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                }
                Instruction::V128Const(_) => {
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                }
                Instruction::LocalGet(idx) => match idx {
                    Index::Id(id) => {
                        stack_sizes.push(StackCtx::convert_wast_types(
                            local_param_types.get(&id.name().to_string()).unwrap(),
                        ));
                        read_locals.insert(id.name().to_string());
                        update_by_valtype(
                            local_param_types.get(&id.name().to_string()).unwrap(),
                            &mut current_i32_count,
                            &mut max_i32_count,
                            &mut current_i64_count,
                            &mut max_i64_count,
                            &mut current_f32_count,
                            &mut max_f32_count,
                            &mut current_f64_count,
                            &mut max_f64_count,
                            &mut current_u128_count,
                            &mut max_u128_count,
                        );
                    }
                    Index::Num(value, _) => {
                        let id = match is_param.get(&format!("l{}", value)) {
                            Some(false) => {
                                format!("l{}", value)
                            }
                            Some(true) => format!("p{}", value),
                            _ => format!("p{}", value),
                        };
                        stack_sizes.push(StackCtx::convert_wast_types(
                            local_param_types.get(&id).unwrap(),
                        ));
                        read_locals.insert(id.to_string());
                        update_by_valtype(
                            local_param_types.get(&id).unwrap(),
                            &mut current_i32_count,
                            &mut max_i32_count,
                            &mut current_i64_count,
                            &mut max_i64_count,
                            &mut current_f32_count,
                            &mut max_f32_count,
                            &mut current_f64_count,
                            &mut max_f64_count,
                            &mut current_u128_count,
                            &mut max_u128_count,
                        );
                    }
                },
                Instruction::LocalSet(idx) => {
                    stack_sizes.pop().unwrap();
                    match idx {
                        Index::Id(id) => {
                            write_locals.insert(id.name().to_string());
                            match local_param_types.get(&id.name().to_string()).unwrap() {
                                ValType::I32 => {
                                    current_i32_count -= 1;
                                }
                                ValType::F32 => {
                                    current_f32_count -= 1;
                                }
                                ValType::I64 => {
                                    current_i64_count -= 1;
                                }
                                ValType::F64 => {
                                    current_f64_count -= 1;
                                }
                                ValType::V128 => {
                                    current_u128_count -= 1;
                                }
                                _ => panic!("Unknown local size found (vstack init)"),
                            }
                        }
                        Index::Num(value, _) => {
                            let id = match is_param.get(&format!("l{}", value)) {
                                Some(false) => {
                                    format!("l{}", value)
                                }
                                Some(true) => format!("p{}", value),
                                _ => format!("p{}", value),
                            };
                            write_locals.insert(id.clone());
                            match local_param_types.get(&id).unwrap() {
                                ValType::I32 => {
                                    current_i32_count -= 1;
                                }
                                ValType::F32 => {
                                    current_f32_count -= 1;
                                }
                                ValType::I64 => {
                                    current_i64_count -= 1;
                                }
                                ValType::F64 => {
                                    current_f64_count -= 1;
                                }
                                ValType::V128 => {
                                    current_u128_count -= 1;
                                }
                                _ => panic!("Unknown local size found (vstack init)"),
                            }
                        }
                    }
                }
                Instruction::LocalTee(idx) => {
                    // LocalTee just peaks the last item on the stack and sets a local value
                    // No stack allocs occur
                    let id: String = match idx {
                        Index::Id(id) => id.name().to_string(),
                        Index::Num(val, _) => match is_param.get(&format!("l{}", val)) {
                            Some(false) => {
                                format!("l{}", val)
                            }
                            Some(true) => format!("p{}", val),
                            _ => format!("p{}", val),
                        },
                    };
                    write_locals.insert(id.clone());
                }
                Instruction::I32Add => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I32Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I64Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I32Sub => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I64Add => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::F64Max => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                }
                Instruction::F64Min => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                }
                Instruction::F32Max => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                }
                Instruction::F32Min => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                }
                Instruction::F64Add => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                }
                Instruction::F64Sub => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                }
                Instruction::F32Add => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                }
                Instruction::F32Sub => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                }
                Instruction::F32Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                }
                Instruction::F64Div => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                }
                Instruction::F32Div => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1;
                }
                Instruction::F64Mul => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                }
                Instruction::I32TruncSatF64S | Instruction::I32TruncSatF64U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                    current_f64_count -= 1;
                }
                Instruction::I64TruncSatF64S | Instruction::I64TruncSatF64U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                    current_f64_count -= 1;
                }
                Instruction::F32Trunc => {}
                Instruction::F64Trunc => {}
                Instruction::F64Neg => {}
                Instruction::F32Neg => {}
                Instruction::F64Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F32Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F64Lt => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F64Gt => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F32Gt => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F32Lt => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F64Le => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F64Ge => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F32Le => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F32Ge => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64LtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F64Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F32Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I32TruncF64U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64TruncF64U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_f64_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I32TruncF64S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_f64_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64TruncF64S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_f64_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I64Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64DivU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I32Eqz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    // no-op
                }
                Instruction::I64Eqz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i64_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I32And => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I64And => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I32Ne => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I32LtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I32LtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I64LtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I32GtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I64GtU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64GtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I32GtS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I32LeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I32LeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I64LeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64LeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I32GeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I32GeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I64GeU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64GeS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I32Xor => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i32_count -= 1;
                }
                Instruction::I32WrapI64 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    current_i64_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I32Extend8S | Instruction::I32Extend16S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                }
                Instruction::I64Extend8S | Instruction::I64Extend16S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                }
                Instruction::I64ExtendI32S | Instruction::I64ExtendI32U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    current_i32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::Call(idx) => {
                    let id = match idx {
                        Index::Id(id) => format_fn_name(id.name()),
                        Index::Num(val, _) => format!("func_{}", val),
                    };

                    if writer_ctx.imports_map.contains_key(&id) {
                        match writer_ctx.imports_map.get(&id) {
                            Some((wasi_api, Some(wasi_fn_name), _)) => {
                                match (wasi_api, WASI_SNAPSHOT_PREVIEW1.get(wasi_fn_name)) {
                                    // ignore WASI API scoping for now
                                    (_, Some(true)) => {
                                        // Taint loops that perform hypercalls
                                        match wasi_fn_name {
                                            &"proc_exit" |
                                            &"fd_write" if unsafe_writes => {
                                                // some hypercalls emit traps/proc_exit instead, and are "safe"
                                            }
                                            _ => {
                                                taint_open_loops(&mut tainted_loops, open_loop_stack.clone());
                                                is_fastcall = false;
                                                num_hypercalls += 1;
                                            }
                                        }

                                        match wasi_fn_name {
                                            &"fd_write"               => {
                                                current_i32_count -= 3;
                                            },
                                            &"proc_exit"              => {
                                                current_i32_count -= 1;
                                            },
                                            &"args_sizes_get"         => {
                                                current_i32_count -= 1;
                                            },
                                            &"args_get"               => {
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
                                            &"poll_oneoff"            => {
                                                current_i32_count -= 3;
                                            },
                                            &"fd_filestat_get"        => {
                                                current_i32_count -= 1;
                                            },
                                            &"fd_read"                => {
                                                current_i32_count -= 3;
                                            },
                                            &"fd_seek"                => {
                                                current_i32_count -= 2;
                                                current_i64_count -= 1;
                                            },
                                            &"path_filestat_get"      => {
                                                current_i32_count -= 4;
                                            },
                                            &"path_open"              => {
                                                current_i32_count -= 6;
                                                current_i64_count -= 2;
                                            },
                                            &"fd_close"               => {
                                                // No-op
                                            },
                                            &"fd_fdstat_get"          => {
                                                current_i32_count -= 1;
                                            },
                                            &"fd_fdstat_set_flags"    => {
                                                current_i32_count -= 1;
                                            },
                                            &"fd_readdir"             => {
                                                current_i32_count -= 3;
                                                current_i64_count -= 1;
                                            },
                                            &"path_create_directory"  => {
                                                current_i32_count -= 2;
                                            },
                                            &"path_remove_directory"  => {
                                                current_i32_count -= 2;
                                            },
                                            &"path_unlink_file"  => {
                                                current_i32_count -= 2;
                                            },
                                            &"path_rename"  => {
                                                current_i32_count -= 5;
                                            },
                                            &"path_symlink"  => {
                                                current_i32_count -= 4;
                                            },
                                            &"vectorvisor_barrier"    => {
                                                // no-op
                                            },
                                            &"serverless_invoke"      => {
                                                current_i32_count -= 1;
                                            },
                                            &"serverless_response"    => {
                                                current_i32_count -= 2;
                                            },
                                            &"clock_time_get"         => {
                                                current_i32_count -= 1;
                                                current_i64_count -= 1;
                                                stack_sizes.pop();
                                                stack_sizes.pop();
                                                stack_sizes.pop();
                                                stack_sizes.push(StackType::i32);
                                            },
                                            &"sched_yield"            => {
                                                // sched_yield is special cased, and not treated as an actual call
                                                num_hypercalls -= 1;
                                                stack_sizes.push(StackType::i32);
                                                update_counter(&mut current_i32_count, &mut max_i32_count);
                                            },
                                            _ => panic!("Unidentified WASI fn name: {:?} (vstack)", wasi_fn_name),
                                        }
                                    },
                                    _ => panic!("WASI import not found, this probably means the hypercall is not yet implemented: {:?} (vstack)", wasi_fn_name)
                                }
                            }
                            _ => panic!(
                                "Unsupported hypercall found {:?} (vstack)",
                                writer_ctx.imports_map.get(&id)
                            ),
                        }
                    } else {
                        // Check the function name to see if it is a valid fastcall
                        // We only taint non-fastcalls
                        if !fastcalls.contains(&id) {
                            taint_open_loops(&mut tainted_loops, open_loop_stack.clone());
                            is_fastcall = false;
                            // Track how many regular function calls we perform
                            num_fn_calls += 1;
                        } else {
                            called_fastcalls.insert(id.to_string());
                        }

                        match writer_ctx.func_map.get(&id) {
                            Some(_) => {
                                let func_type_signature = &writer_ctx.func_map.get(&id).unwrap().ty;
                                match &func_type_signature.inline {
                                    // if we can find the type signature
                                    Some(res) => {
                                        for (_, _, ty) in res.params.iter() {
                                            stack_sizes.pop();
                                            match ty {
                                                ValType::I32 => {
                                                    current_i32_count -= 1;
                                                }
                                                ValType::I64 => {
                                                    current_i64_count -= 1;
                                                }
                                                ValType::F32 => {
                                                    current_f32_count -= 1;
                                                }
                                                ValType::F64 => {
                                                    current_f64_count -= 1;
                                                }
                                                ValType::V128 => {
                                                    current_u128_count -= 1;
                                                }
                                                _ => panic!(
                                                    "vstack missing valtype check in func call"
                                                ),
                                            }
                                        }

                                        // push the results back
                                        for ty in res.results.iter() {
                                            stack_sizes.push(StackCtx::convert_wast_types(&ty));
                                            update_by_valtype(
                                                ty,
                                                &mut current_i32_count,
                                                &mut max_i32_count,
                                                &mut current_i64_count,
                                                &mut max_i64_count,
                                                &mut current_f32_count,
                                                &mut max_f32_count,
                                                &mut current_f64_count,
                                                &mut max_f64_count,
                                                &mut current_u128_count,
                                                &mut max_u128_count,
                                            );
                                        }
                                    }
                                    // if we cannot find the type signature, we need to look it up to check for the param offset
                                    None => {
                                        let fn_type_id = match func_type_signature.index {
                                            Some(Index::Id(id)) => id.name().to_string(),
                                            Some(Index::Num(n, _)) => format!("t{}", n),
                                            None => format!(""),
                                        };

                                        let function_type = writer_ctx.types.get(&fn_type_id);
                                        match function_type {
                                            Some(TypeDef::Func(ft)) => {
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
                                                        ValType::V128 => {
                                                            current_u128_count -= 1;
                                                        }
                                                        _ => panic!("vstack missing valtype check in func call")
                                                    }
                                                    stack_sizes.pop();
                                                }
                                                // push the results back
                                                for ty in ft.results.iter() {
                                                    stack_sizes.push(StackCtx::convert_wast_types(&ty));
                                                    update_by_valtype(ty,
                                                                        &mut current_i32_count, &mut max_i32_count,
                                                                        &mut current_i64_count, &mut max_i64_count,
                                                                        &mut current_f32_count, &mut max_f32_count,
                                                                        &mut current_f64_count, &mut max_f64_count,
                                                                        &mut current_u128_count, &mut max_u128_count);
                                                }
                                            },
                                            None => (),
                                            _ => panic!("Non-function type referenced from function (vstack)")
                                        };
                                    }
                                }
                            }
                            // we have an import that isn't a system call...
                            None => {
                                panic!("Unknown import (vstack) id: {:?}", id);
                            }
                        }
                    }
                }
                Instruction::CallIndirect(call_indirect) => {
                    let mut matching_types = 0;
                    // Check for types
                    match (
                        call_indirect.ty.index.as_ref(),
                        call_indirect.ty.inline.as_ref(),
                    ) {
                        (Some(index), _) => {
                            // if we have an index, we need to look it up in the global structure
                            let type_index = match index {
                                Num(n, _) => format!("t{}", n),
                                Id(i) => i.name().to_string(),
                            };

                            let indirect_func_type = match writer_ctx.types.get(&type_index).unwrap() {
                                TypeDef::Func(ft) => ft,
                                _ => panic!("Indirect call cannot have a type of something other than a func"),
                            };

                            // Track how many targetable indirect function calls match the given type
                            let mut matching_types = 0;
                            let mut fastcall_opt = 0;

                            // We only need to call functions with matching type signatures, the rest would trap
                            for func_id in indirect_call_mapping.values() {
                                let f_name = match func_id {
                                    Index::Id(id) => format_fn_name(id.name()),
                                    Index::Num(val, _) => format!("func_{}", val),
                                };
                                let func_type_signature =
                                    &writer_ctx.func_map.get(&f_name).unwrap().ty;

                                let func_type_index = match func_type_signature.index {
                                    Some(Index::Id(id)) => id.name().to_string(),
                                    Some(Index::Num(val, _)) => format!("t{}", val),
                                    None => panic!("Only type indicies supported for call_indirect in vstack pass"),
                                };

                                if func_type_index == type_index {
                                    matching_types += 1;
                                    if fastcalls.contains(&f_name) && f_name != curr_fn_name {
                                        fastcall_opt += 1;
                                    }
                                }
                            }

                            // Track the number of function call stubs to generate
                            // We only generate stubs for non-fastcalls
                            if fastcall_opt == matching_types {
                                //num_fn_calls += matching_types - fastcall_opt;
                                // no-op in this case
                                indirect_call_map.insert(indirect_call_count, true);
                            } else if matching_types > 0 {
                                num_fn_calls += matching_types;
                                taint_open_loops(&mut tainted_loops, open_loop_stack.clone());
                                is_fastcall = false;
                                indirect_call_map.insert(indirect_call_count, false);
                                assert!(!fastcalls.contains(&curr_fn_name), "Incorrect fastcall opt found, cannot emit call_indirect slowpath: {:?}", &curr_fn_name);
                            } else {
                                // indirect call with 0 functions
                                indirect_call_map.insert(indirect_call_count, true);
                            }

                            indirect_call_count += 1;

                            // Pop the CallIndirect index
                            stack_sizes.pop();
                            // Then, pop off the parameters
                            for (_, _, param_type) in indirect_func_type.params.iter() {
                                stack_sizes.pop();
                                match param_type {
                                    ValType::I32 => {
                                        current_i32_count -= 1;
                                    }
                                    ValType::I64 => {
                                        current_i64_count -= 1;
                                    }
                                    ValType::F32 => {
                                        current_f32_count -= 1;
                                    }
                                    ValType::F64 => {
                                        current_f64_count -= 1;
                                    }
                                    ValType::V128 => {
                                        current_u128_count -= 1;
                                    }
                                    _ => {
                                        panic!("vstack missing valtype check in indirect func call")
                                    }
                                }
                            }

                            // Next, push the result(s) back
                            for return_type in indirect_func_type.results.iter() {
                                stack_sizes.push(StackCtx::convert_wast_types(&return_type));
                                update_by_valtype(
                                    return_type,
                                    &mut current_i32_count,
                                    &mut max_i32_count,
                                    &mut current_i64_count,
                                    &mut max_i64_count,
                                    &mut current_f32_count,
                                    &mut max_f32_count,
                                    &mut current_f64_count,
                                    &mut max_f64_count,
                                    &mut current_u128_count,
                                    &mut max_u128_count,
                                );
                            }
                        }
                        (_, Some(_inline)) => {
                            panic!("Inline types for call_indirect not implemented yet (vstack)")
                        }
                        _ => (),
                    };
                }
                Instruction::I32Eq => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                }
                Instruction::I32Or => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                }
                Instruction::I32ShrU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 2;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64ShrU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 2;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I32ShrS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                }
                Instruction::I32Shl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                }
                Instruction::I64Shl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I32DivU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                }
                Instruction::I32DivS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                }
                Instruction::I64DivS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I32RemU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                }
                Instruction::I64RemU => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I32RemS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                }
                Instruction::I64RemS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I64ShrS => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I64Xor => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I64Or => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I32Rotl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_i32_count -= 1;
                }
                Instruction::I64Rotl => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I64Sub => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_i64_count -= 1;
                }
                Instruction::I64ReinterpretF64 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_f64_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::F64Nearest => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                }
                Instruction::F32Nearest => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                }
                Instruction::F64Copysign => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);
                    current_f64_count -= 1;
                }
                Instruction::F64Copysign => {
                    stack_sizes.pop();
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);
                    current_f32_count -= 1
                }
                Instruction::F64ReinterpretI64 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_i64_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                }
                Instruction::F32ReinterpretI32 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);

                    current_i32_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                }
                Instruction::I32ReinterpretF32 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_f32_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F64Ceil => {
                    // No-op
                }
                Instruction::F32Ceil => {
                    // No-op
                }
                Instruction::F64Floor => {
                    // No-op
                }
                Instruction::F32Floor => {
                    // No-op
                }
                Instruction::F64PromoteF32 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_f32_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                }
                Instruction::F32DemoteF64 => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);

                    current_f64_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                }
                Instruction::I32TruncF32U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_f32_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::I64TruncF32U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_f32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I64TruncF32S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);

                    current_f32_count -= 1;
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                }
                Instruction::I32TruncF32S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);

                    current_f32_count -= 1;
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::F64ConvertI32S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_i32_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                }
                Instruction::F64ConvertI32U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_i32_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                }
                Instruction::F32ConvertI32U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);

                    current_i32_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                }
                Instruction::F32ConvertI64U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);

                    current_i64_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                }
                Instruction::F32ConvertI32S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f32);

                    current_i32_count -= 1;
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                }
                Instruction::F64ConvertI64U => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_i64_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                }
                Instruction::F64ConvertI64S => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::f64);

                    current_i64_count -= 1;
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                }
                Instruction::F64Sqrt | Instruction::F32Sqrt => {
                    // no-op
                }
                Instruction::F32Abs | Instruction::F64Abs => {
                    // no-op
                }
                Instruction::I32Clz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                }
                Instruction::I32Popcnt => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                }
                Instruction::I64Clz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    // no-op
                }
                Instruction::I32Ctz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i32);
                    // no-op
                }
                Instruction::I64Ctz => {
                    stack_sizes.pop();
                    stack_sizes.push(StackType::i64);
                    // no-op
                }
                Instruction::If(b) => {
                    if_else_branches.push(false);
                    if_idx += 1;
                    if_else_idx_stack.push(if_idx);
                    let block_type = get_func_result(&writer_ctx, &b.ty);
                    match block_type.clone() {
                        Some(stack_size) => {
                            update_by_valtype(
                                &StackCtx::convert_stacktypes_valtype(&stack_size.clone()),
                                &mut current_i32_count,
                                &mut max_i32_count,
                                &mut current_i64_count,
                                &mut max_i64_count,
                                &mut current_f32_count,
                                &mut max_f32_count,
                                &mut current_f64_count,
                                &mut max_f64_count,
                                &mut current_u128_count,
                                &mut max_u128_count,
                            );
                        }
                        None => (),
                    };

                    let label: String = match b.label {
                        Some(id) => id.name().to_string().clone(),
                        // we never use If names anyways, no need to track the value
                        _ => format!("if{}", if_idx),
                    };

                    control_stack.push((
                        label,
                        block_type,
                        ControlStackVStackTypes::If,
                        stack_sizes.clone(),
                        current_i32_count.clone(),
                        current_i64_count.clone(),
                        current_f32_count.clone(),
                        current_f64_count.clone(),
                        current_u128_count.clone(),
                    ));
                }
                Instruction::Else(_) => {
                    /*
                     * If we encounter an else:
                     * if_else_idx_stack always points at the most recent if block
                     */
                    if_else_branches[if_else_idx_stack.last().unwrap() - 1] = true;
                }
                /*
                 * Track block & loop starts/ends to minimize intermediate value req
                 */
                Instruction::Block(b) => {
                    // Get the type of the block
                    let block_type = get_func_result(&writer_ctx, &b.ty);
                    match block_type.clone() {
                        Some(stack_size) => {
                            update_by_valtype(
                                &StackCtx::convert_stacktypes_valtype(&stack_size.clone()),
                                &mut current_i32_count,
                                &mut max_i32_count,
                                &mut current_i64_count,
                                &mut max_i64_count,
                                &mut current_f32_count,
                                &mut max_f32_count,
                                &mut current_f64_count,
                                &mut max_f64_count,
                                &mut current_u128_count,
                                &mut max_u128_count,
                            );
                        }
                        None => (),
                    };

                    let label: String = match b.label {
                        Some(id) => id.name().to_string().clone(),
                        // we never use block names anyways, no need to track the value
                        _ => format!("b{}", 0),
                    };

                    control_stack.push((
                        label,
                        block_type,
                        ControlStackVStackTypes::Block,
                        stack_sizes.clone(),
                        current_i32_count.clone(),
                        current_i64_count.clone(),
                        current_f32_count.clone(),
                        current_f64_count.clone(),
                        current_u128_count.clone(),
                    ));

                    // push the stack frame
                    read_locals_stack.push(read_locals.clone());
                    write_locals_stack.push(write_locals.clone());
                    read_locals.clear();
                    write_locals.clear();

                    curr_ctx.push(curr_ctx_idx);
                    curr_ctx_idx += 1;
                }
                Instruction::Loop(b) => {
                    context_map_tainted_loop_map
                        .insert(curr_ctx_idx, tainted_loops.len().try_into().unwrap());
                    tainted_loops.push(false);
                    open_loop_stack.push(loop_idx);
                    loop_idx += 1;
                    empty_loop = true;
                    let block_type = get_func_result(&writer_ctx, &b.ty);
                    match block_type.clone() {
                        Some(stack_size) => {
                            update_by_valtype(
                                &StackCtx::convert_stacktypes_valtype(&stack_size.clone()),
                                &mut current_i32_count,
                                &mut max_i32_count,
                                &mut current_i64_count,
                                &mut max_i64_count,
                                &mut current_f32_count,
                                &mut max_f32_count,
                                &mut current_f64_count,
                                &mut max_f64_count,
                                &mut current_u128_count,
                                &mut max_u128_count,
                            );
                        }
                        None => (),
                    };

                    let label: String = match b.label {
                        Some(id) => id.name().to_string().clone(),
                        _ => format!("l{}", loop_idx),
                    };

                    control_stack.push((
                        label,
                        block_type,
                        ControlStackVStackTypes::Loop,
                        stack_sizes.clone(),
                        current_i32_count.clone(),
                        current_i64_count.clone(),
                        current_f32_count.clone(),
                        current_f64_count.clone(),
                        current_u128_count.clone(),
                    ));

                    read_locals_stack.push(read_locals.clone());
                    write_locals_stack.push(write_locals.clone());
                    read_locals.clear();
                    write_locals.clear();

                    curr_ctx.push(curr_ctx_idx);
                    curr_ctx_idx += 1;

                    nested_loops.push(curr_ctx.clone());

                    // We need to continue here to avoid resetting the empty_loop counter
                    continue;
                }
                Instruction::End(_id) => {
                    // As we close loops, keep track so we don't taint them
                    let (
                        _label,
                        t,
                        control_stack_op,
                        stack_restore,
                        old_i32,
                        old_i64,
                        old_f32,
                        old_f64,
                        old_u128,
                    ) = control_stack.pop().unwrap();

                    match control_stack_op {
                        ControlStackVStackTypes::Block => {
                            // pop the stack frame
                            let idx = curr_ctx.pop().unwrap();
                            // save the read locals for this stack frame
                            restore_context_map.insert(idx, read_locals.clone());
                            save_context_map.insert(idx, write_locals.clone());
                            read_locals = read_locals_stack.pop().unwrap();
                            write_locals = write_locals_stack.pop().unwrap();
                        }
                        ControlStackVStackTypes::Loop => {
                            // pop the stack frame
                            let idx = curr_ctx.pop().unwrap();
                            // save the read locals for this stack frame
                            restore_context_map.insert(idx, read_locals.clone());
                            save_context_map.insert(idx, write_locals.clone());
                            loop_nested_context_map.insert(idx, curr_ctx_idx);
                            read_locals = read_locals_stack.pop().unwrap();
                            write_locals = write_locals_stack.pop().unwrap();

                            open_loop_stack.pop().unwrap();
                        }
                        ControlStackVStackTypes::If => {
                            if_else_idx_stack.pop().unwrap();
                        }
                    }

                    // restore the previous stack frame
                    stack_sizes = stack_restore;
                    current_i32_count = old_i32;
                    current_i64_count = old_i64;
                    current_f32_count = old_f32;
                    current_f64_count = old_f64;
                    current_u128_count = old_u128;

                    // We have to push the result value of the block (if we have one)
                    match t {
                        Some(stack_type) => {
                            stack_sizes.push(stack_type.clone());
                        }
                        None => (),
                    }
                }
                Instruction::Select(_) => {
                    let _c = stack_sizes.pop().unwrap(); // c
                    let arg1 = stack_sizes.pop().unwrap();
                    let arg2 = stack_sizes.pop().unwrap();
                    if arg1 != arg2 {
                        panic!("Select must operate on two args of the same type (vstack): c: {:?}, {:?}, {:?} in {:?}", _c, arg1, arg2, curr_fn_name);
                    }
                    current_i32_count -= 1;
                    // depending on the arg1, arg2 vals we pop different types
                    match arg1 {
                        StackType::i32 => {
                            current_i32_count -= 1;
                            stack_sizes.push(StackType::i32);
                        }
                        StackType::i64 => {
                            current_i64_count -= 1;
                            stack_sizes.push(StackType::i64);
                        }
                        StackType::f32 => {
                            current_f32_count -= 1;
                            stack_sizes.push(StackType::f32);
                        }
                        StackType::f64 => {
                            current_f64_count -= 1;
                            stack_sizes.push(StackType::f64);
                        }
                        StackType::u128 => {
                            current_u128_count -= 1;
                            stack_sizes.push(StackType::u128);
                        }
                    }
                }
                Instruction::MemoryGrow(_arg) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::i32);
                    // no-op
                }
                Instruction::MemorySize(_arg) => {
                    stack_sizes.push(StackType::i32);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                }
                Instruction::Return => {}
                Instruction::Br(idx) => {
                    if empty_loop {
                        taint_open_loops(&mut tainted_loops, open_loop_stack.clone());
                    }
                }
                Instruction::BrIf(idx) => {
                    if empty_loop {
                        taint_open_loops(&mut tainted_loops, open_loop_stack.clone());
                    }
                    stack_sizes.pop().unwrap();
                    current_i32_count -= 1;
                }
                Instruction::BrTable(table_idxs) => {
                    if empty_loop {
                        taint_open_loops(&mut tainted_loops, open_loop_stack.clone());
                    }
                    stack_sizes.pop().unwrap();
                    current_i32_count -= 1;
                }
                Instruction::Unreachable => {
                    if !is_gpu && !fastcalls.contains(&curr_fn_name) {
                        // This happens because unreachable is emitted as a hypercall in this case
                        num_hypercalls += 1;
                    }
                }
                Instruction::I8x16ExtractLaneU(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::i32);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                    current_u128_count -= 1;
                }
                Instruction::I8x16ExtractLaneS(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::i32);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                    current_u128_count -= 1;
                }
                Instruction::I16x8ExtractLaneU(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::i32);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                    current_u128_count -= 1;
                }
                Instruction::I16x8ExtractLaneS(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::i32);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                    current_u128_count -= 1;
                }
                Instruction::I16x8MaxU => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I8x16GtS | Instruction::I8x16GtU => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I16x8GtS | Instruction::I16x8GtU => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I8x16MaxU => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I8x16NarrowI16x8S => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I8x16NarrowI16x8U => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I64x2Mul
                | Instruction::I64x2Add
                | Instruction::I64x2Sub
                | Instruction::I64x2Eq
                | Instruction::I64x2GeS
                | Instruction::I64x2GtS
                | Instruction::I64x2LtS
                | Instruction::I64x2LeS
                | Instruction::I64x2Ne => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I32x4Splat => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_i32_count -= 1;
                }
                Instruction::I32x4Eq => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I8x16Splat => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_i32_count -= 1;
                }
                Instruction::I16x8Splat => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_i32_count -= 1;
                }
                Instruction::F32x4Splat => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_f32_count -= 1;
                }
                Instruction::F64x2Splat => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_f64_count -= 1;
                }
                Instruction::I64x2Splat => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_i64_count -= 1;
                }
                Instruction::I8x16LeS
                | Instruction::I8x16LeU
                | Instruction::I8x16LtS
                | Instruction::I8x16LtU
                | Instruction::I64x2Shl
                | Instruction::I64x2ShrU
                | Instruction::I64x2ShrS
                | Instruction::I8x16Shl
                | Instruction::I8x16ShrU
                | Instruction::I8x16ShrS => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_i32_count -= 1;
                }
                Instruction::I8x16Add | Instruction::I8x16Sub => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I8x16Eq => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I8x16Ne => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I16x8ShrU | Instruction::I16x8ShrS => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_i32_count -= 1;
                }
                Instruction::I16x8Mul => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I16x8Add
                | Instruction::I16x8GeS
                | Instruction::I16x8LtS
                | Instruction::I16x8LtU
                | Instruction::I16x8LeS
                | Instruction::I16x8LeU
                | Instruction::I16x8GeU
                | Instruction::I16x8Sub => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I16x8Shl => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_i32_count -= 1;
                }
                Instruction::I16x8Ne => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I16x8Eq => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::F32x4Mul => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::F32x4Add => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::F32x4Ne => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I32x4Shl => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_i32_count -= 1;
                }
                Instruction::I32x4ShrS | Instruction::I32x4ShrU => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_i32_count -= 1;
                }
                Instruction::I32x4GeS
                | Instruction::I32x4GeU
                | Instruction::I32x4GtS
                | Instruction::I32x4LtS
                | Instruction::I32x4LtU
                | Instruction::I32x4LeS
                | Instruction::I32x4LeU
                | Instruction::I32x4GtU
                | Instruction::I32x4MaxU => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I32x4Mul => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I32x4Add => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I32x4Sub => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::I32x4Ne => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::V128Xor => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::V128And => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::V128Or => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::V128Not => {
                    // no-op
                }
                Instruction::I8x16Shuffle(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_u128_count -= 1;
                }
                Instruction::V128Load8x8U(_)
                | Instruction::V128Load8x8S(_)
                | Instruction::V128Load16x4U(_)
                | Instruction::V128Load16x4S(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_i32_count -= 1;
                }
                Instruction::V128Load8Lane(_)
                | Instruction::V128Load16Lane(_)
                | Instruction::V128Load32Lane(_)
                | Instruction::V128Load64Lane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_i32_count -= 1;
                }
                Instruction::V128Store32Lane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_i32_count -= 1;
                }
                Instruction::V128Store64Lane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_i32_count -= 1;
                }
                Instruction::V128Load8Splat(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_i32_count -= 1;
                }
                Instruction::V128Load16Splat(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_i32_count -= 1;
                }
                Instruction::V128Load32Splat(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_i32_count -= 1;
                }
                Instruction::V128Load64Splat(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    update_counter(&mut current_u128_count, &mut max_u128_count);
                    current_i32_count -= 1;
                }
                Instruction::I32x4ExtractLane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::i32);
                    update_counter(&mut current_i32_count, &mut max_i32_count);
                    current_u128_count -= 1;
                }
                Instruction::I64x2ExtractLane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::i64);
                    update_counter(&mut current_i64_count, &mut max_i64_count);
                    current_u128_count -= 1;
                }
                Instruction::F32x4ExtractLane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::f32);
                    update_counter(&mut current_f32_count, &mut max_f32_count);
                    current_u128_count -= 1;
                }
                Instruction::F64x2ExtractLane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::f64);
                    update_counter(&mut current_f64_count, &mut max_f64_count);
                    current_u128_count -= 1;
                }
                Instruction::I32x4ReplaceLane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_i32_count -= 1;
                }
                Instruction::I64x2ReplaceLane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_i64_count -= 1;
                }
                Instruction::F32x4ReplaceLane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_f32_count -= 1;
                }
                Instruction::F64x2ReplaceLane(_) => {
                    stack_sizes.pop().unwrap();
                    stack_sizes.pop().unwrap();
                    stack_sizes.push(StackType::u128);
                    current_f64_count -= 1;
                }
                _ => panic!(
                    "Instruction {:?} not yet implemented (vstack-pass)",
                    instruction
                ),
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

        let mut u128_stack = vec![];
        for idx in 0..max_u128_count {
            u128_stack.push(format!("u128_{}", idx));
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
        let (mut max_offset, mut max_offset_type_size): (u32, u32) =
            match cloned_local_offsets.pop() {
                Some((name, offset)) => {
                    (
                        offset,
                        writer_ctx.get_size_valtype(&local_param_types.get(&name).unwrap()),
                    )
                    //(offset, 2)
                }
                None => (0, 0),
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
                    max_offset_type_size =
                        writer_ctx.get_size_valtype(&local_param_types.get(name).unwrap());
                }
            }
        }

        // For each intermediate, map it to an offset
        let mut intermediate_offsets: HashMap<String, u32> = HashMap::new();
        let mut curr_offset = 0;
        for idx in i32_stack.clone() {
            intermediate_offsets.insert(idx, curr_offset);
            curr_offset += 2;
        }
        for idx in i64_stack.clone() {
            intermediate_offsets.insert(idx, curr_offset);
            curr_offset += 2;
        }
        for idx in f32_stack.clone() {
            intermediate_offsets.insert(idx, curr_offset);
            curr_offset += 2;
        }
        for idx in f64_stack.clone() {
            intermediate_offsets.insert(idx, curr_offset);
            curr_offset += 2;
        }
        for idx in u128_stack.clone() {
            intermediate_offsets.insert(idx, curr_offset);
            curr_offset += 4;
        }

        // For each loop that we can't optimize, we need to generate a function call stub
        num_fn_calls += tainted_loops
            .iter()
            .filter(|x| **x == true)
            .collect::<Vec<&bool>>()
            .len() as u32;

        // Get the size of the local_cache
        // This array goes on the stack between the locals and intermediate values
        // This is an artifact from trying out a bitvector (increases compile times substantially)
        //let round_up_to_next_multiple8: i32 = (((local_offsets.len() as i32 + 1) + 7) & (-8 as i32)) / 8;
        let round_up_to_next_multiple8: i32 = 0;

        // For the top level stack frame
        let idx = curr_ctx.pop().unwrap();
        // save the read locals for this stack frame
        restore_context_map.insert(idx, read_locals.clone());
        save_context_map.insert(idx, write_locals.clone());

        // Now demote local values to shared memory (L1 cache) based on partitioning information
        // reduction_size
        let mut moved_locals: HashSet<String> = HashSet::new();

        // Set the reduction size to be half of reduction_size, so we telescope across
        // functions in the partition. (i.e. 64, 32, 16, 8, 4, 4 versus just 128 in one function)
        let mut local_reduction_size = *reduction_size;

        let mut demoted_intermediates: HashSet<String> = HashSet::new();
        if local_reduction_size > 0 && local_work_group != 999999 {
            // Alloc some smem bytes for intermediate vals. We only demote i32/i64 vals to
            // avoid changes elsewhere in the compiler (mostly memcpy for floats)
            let mut intermediate_reduction_size = *reduction_size;

            *reduction_size -= intermediate_reduction_size;

            // I32
            let mut idx = 0;
            for val in i32_stack.clone() {
                if intermediate_reduction_size > 0 {
                    let old_offset: u32 = *intermediate_offsets.get(&val).unwrap();
                    i32_stack[idx] = format!("{}[thread_idx]", val.clone());
                    demoted_intermediates.insert(i32_stack[idx].clone());
                    intermediate_offsets.insert(i32_stack[idx].clone(), old_offset);
                    intermediate_reduction_size -= 4;
                } else {
                    break;
                }
                idx += 1;
            }

            // I64
            idx = 0;
            for val in i64_stack.clone() {
                if intermediate_reduction_size > 0 {
                    let old_offset: u32 = *intermediate_offsets.get(&val).unwrap();
                    i64_stack[idx] = format!("{}[thread_idx]", val.clone());
                    demoted_intermediates.insert(i64_stack[idx].clone());
                    intermediate_offsets.insert(i64_stack[idx].clone(), old_offset);
                    intermediate_reduction_size -= 8;
                } else {
                    break;
                }
                idx += 1;
            }

            // add remains of int reduction size back to locals
            *reduction_size += intermediate_reduction_size;

            // I32
            for (local, l_type) in local_types_converted.clone().iter() {
                match l_type {
                    StackType::i32 => {
                        if !is_param.get(local).unwrap() && local_reduction_size > 4 {
                            moved_locals.insert(local.to_string());
                            local_reduction_size -= 4;
                        }
                    }
                    _ => (),
                }
            }

            // I64
            for (local, l_type) in local_types_converted.clone().iter() {
                match l_type {
                    StackType::i64 => {
                        if !is_param.get(local).unwrap() && local_reduction_size > 8 {
                            moved_locals.insert(local.to_string());
                            local_reduction_size -= 8;
                        }
                    }
                    _ => (),
                }
            }

            // F32
            for (local, l_type) in local_types_converted.clone().iter() {
                match l_type {
                    StackType::f32 => {
                        if !is_param.get(local).unwrap() && local_reduction_size > 4 {
                            moved_locals.insert(local.to_string());
                            local_reduction_size -= 4;
                        }
                    }
                    _ => (),
                }
            }

            // F64
            for (local, l_type) in local_types_converted.clone().iter() {
                match l_type {
                    StackType::f64 => {
                        if !is_param.get(local).unwrap() && local_reduction_size > 8 {
                            moved_locals.insert(local.to_string());
                            local_reduction_size -= 8;
                        }
                    }
                    _ => (),
                }
            }
        }

        if *reduction_size == 4 {
            *reduction_size = 0;
        }

        for (key, val) in loop_nested_context_map {
            // For each nested loop, if it is not tainted, we want to add all of the nested stack
            // frame local contexts to it
            let tainted_loops_idx = *context_map_tainted_loop_map.get(&key).unwrap() as usize;
            if !tainted_loops[tainted_loops_idx] {
                let mut new_read_hs: HashSet<String> = HashSet::new();
                //let mut new_write_hs: HashSet<String> = HashSet::new();
                // process stack frames in the range: [key, val)
                // i.e. the blocks/loops that come after the topmost loop
                for idx in key..val {
                    let read_set: HashSet<String> = restore_context_map.get(&idx).unwrap().clone();
                    //let write_set: HashSet<String> = save_context_map.get(&idx).unwrap().clone();
                    new_read_hs.extend(read_set);
                    //new_write_hs.extend(write_set);
                }
                restore_context_map.insert(key, new_read_hs);
                //save_context_map.insert(key, new_write_hs);
            }
        }

        StackCtx {
            _curr_fn_name: curr_fn_name,
            stack_frame_idx: 1,
            stack_frame_stack: vec![0],
            restore_context_map: restore_context_map,
            save_context_map: save_context_map,
            i32_stack: i32_stack,
            i32_idx: 0,
            i64_stack: i64_stack,
            i64_idx: 0,
            f32_stack: f32_stack,
            f32_idx: 0,
            f64_stack: f64_stack,
            f64_idx: 0,
            u128_stack: u128_stack,
            u128_idx: 0,
            // the offset used to store intermediate variables
            stack_frame_offset: round_up_to_next_multiple8 as u32
                + max_offset
                + max_offset_type_size,
            local_cache_size: round_up_to_next_multiple8 as u32,
            intermediate_offsets: intermediate_offsets,
            local_offsets: local_offsets.clone(),
            local_types: local_types_converted,
            moved_locals: moved_locals,
            demoted_intermediates: demoted_intermediates,
            param_offset: param_offset,
            total_stack_types: vec![],
            control_stack: vec![],
            control_stack_snapshots: vec![],
            tainted_loops: tainted_loops,
            if_else_branches: if_else_branches,
            num_fn_calls: num_fn_calls,
            num_hypercalls: num_hypercalls,
            is_param: is_param.clone(),
            called_fastcalls: called_fastcalls,
            max_emitted_context: 0,
            opt_loop_tracking: vec![],
            fastcall_opt_possible: is_fastcall,
            interleave: interleave,
            indirect_call_map: indirect_call_map,
        }
    }

    pub fn check_optimized_indirect_call(&self, idx: u32) -> bool {
        *self.indirect_call_map.get(&idx).unwrap_or(&false)
    }

    pub fn is_local_local(&self, local: String) -> bool {
        self.moved_locals.contains(&local)
    }

    /*
     * Check to see if we can optimize the currently selected loop
     */
    pub fn is_loop_tainted(&mut self, loop_idx: usize) -> bool {
        self.tainted_loops[loop_idx]
    }

    /*
     * Track when we enter an optimized loop, to avoid emitting extra context saves
     */
    pub fn open_opt_loop(&mut self) -> () {
        self.opt_loop_tracking.push(true);
    }

    pub fn close_opt_loop(&mut self) -> () {
        self.opt_loop_tracking.pop();
    }

    /*
     * Check if an If block has a matching else
     */
    pub fn if_has_else(&mut self, if_idx: usize) -> bool {
        self.if_else_branches[if_idx]
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
            }
            None => {
                // Else, push the total stack inc
                self.control_stack.push(stack_inc);
            }
        }
    }

    pub fn vstack_pop_stack_info(&mut self) -> () {
        self.control_stack.pop();
    }

    pub fn vstack_push_stack_frame(&mut self, is_virtual: bool, is_if_frame: bool) -> () {
        self.control_stack_snapshots
            .push(StackSnapshot::from_current_ctx(self, is_virtual));

        // save the context idx values so we can return to them later
        if !is_if_frame {
            self.stack_frame_stack.push(self.stack_frame_idx);
            self.stack_frame_idx += 1;
        }
    }

    pub fn vstack_pop_stack_frame(&mut self, is_if_frame: bool) -> () {
        let stack_frame_unwind = self.control_stack_snapshots.pop().unwrap();
        self.i32_idx = stack_frame_unwind.i32_idx;
        self.i64_idx = stack_frame_unwind.i64_idx;
        self.f32_idx = stack_frame_unwind.f32_idx;
        self.f64_idx = stack_frame_unwind.f64_idx;
        self.u128_idx = stack_frame_unwind.u128_idx;
        self.total_stack_types = stack_frame_unwind.stack_types;

        // restore_context tracking
        if !is_if_frame {
            self.stack_frame_stack.pop().unwrap();
        }
    }

    /*
     * This function is used to check for edge cases in the 'end' instruction w/Loops.
     * Ex:
     * loop (result i32)
     *  br 0
     * end
     *
     * There is no hanging value for end here, so we have to check for this
     */
    pub fn vstack_check_for_hanging_value(&mut self, t: StackType) -> bool {
        // Examine the most recently pushed stack frame
        let stack_frame = match self.control_stack_snapshots.last() {
            Some(sf) => sf,
            None => {
                // For non-tainted loops or those instead fastcalls,
                // We just check to see if there is at least val of the requested type on the stack
                if self.i32_idx > 0 {
                    return true;
                } else {
                    return false;
                }
            }
        };

        match t {
            StackType::i32 => {
                // if self.i32_idx > stack_frame.i32_idx, then we have at least 1 hanging value
                if stack_frame.i32_idx < self.i32_idx {
                    true
                } else {
                    false
                }
            }
            StackType::i64 => {
                if stack_frame.i64_idx < self.i64_idx {
                    true
                } else {
                    false
                }
            }
            StackType::f32 => {
                if stack_frame.f32_idx < self.f32_idx {
                    true
                } else {
                    false
                }
            }
            StackType::f64 => {
                if stack_frame.f64_idx < self.f64_idx {
                    true
                } else {
                    false
                }
            }
            StackType::u128 => {
                if stack_frame.u128_idx < self.u128_idx {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn convert_wast_types(ty: &ValType) -> StackType {
        match ty {
            ValType::I32 => StackType::i32,
            ValType::F32 => StackType::f32,
            ValType::I64 => StackType::i64,
            ValType::F64 => StackType::f64,
            ValType::V128 => StackType::u128,
            _ => panic!("Unknown stack type (vstack)"),
        }
    }

    pub fn convert_stacktypes_valtype(ty: &StackType) -> ValType {
        match ty {
            StackType::i32 => ValType::I32,
            StackType::i64 => ValType::I64,
            StackType::f32 => ValType::F32,
            StackType::f64 => ValType::F64,
            StackType::u128 => ValType::V128,
            _ => panic!("Unknown stack type (convert_stacktypes_valtype)"),
        }
    }

    pub fn emit_cache_array(&self, is_fastcall: bool) -> (String, u32) {
        let mut max_offset: u32 = 0;
        for (_local, offset) in self.local_offsets.iter() {
            if *offset > max_offset {
                max_offset = *offset;
            }
        }
        if self.local_offsets.len() > 0 && !is_fastcall {
            // we want this array to function as a bit-indexed array to save space
            //let round_up_to_next_multiple8: i32 = ((max_offset as i32 + 1) + 7) & (-8 as i32);
            //(format!("\tuchar local_cache[{}] = {{ 0 }};\n", round_up_to_next_multiple8 / 8), (round_up_to_next_multiple8 / 8).try_into().unwrap())
            (
                format!("\tbool local_cache[{}] = {{ 0 }};\n", max_offset + 1),
                0,
            )
        } else {
            // emit a single element array to deal with compiler errors
            (format!("\tbool local_cache[{}] = {{ 0 }};\n", 1), 0)
        }
    }

    pub fn emit_fastcall_header(&self) -> String {
        let mut ret_str = String::from("(");

        let mut params = self
            .local_types
            .iter()
            .collect::<Vec<(&String, &StackType)>>();
        params.sort_by(|(a, _), (b, _)| {
            self.local_offsets
                .get(&a.to_string())
                .unwrap()
                .cmp(self.local_offsets.get(&b.to_string()).unwrap())
        });

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
                    }
                    StackType::i64 => {
                        ret_str += &format!("ulong {}, ", local_name);
                    }
                    StackType::f32 => {
                        ret_str += &format!("float {}, ", local_name);
                    }
                    StackType::f64 => {
                        ret_str += &format!("double {}, ", local_name);
                    }
                    StackType::u128 => {
                        ret_str += &format!("ulong2 {}, ", local_name);
                    }
                }
            }
        }

        // now add the other info we need for fastcalls
        ret_str += &format!("global uint *heap_u32, global uint *current_mem_size, global uint *max_mem_size, global uint *globals_buffer, uint warp_idx, uint thread_idx, uint read_idx, global ulong *overhead_tracker, local uchar *scratch_space) {{\n");

        ret_str
    }

    pub fn emit_intermediates(&self, is_fastcall: bool, local_work_group: usize) -> String {
        let mut ret_str = String::from("");

        // emit the locals and parameters
        for (local_name, local_type) in self.local_types.iter() {
            let param_found = match self.is_param.get(local_name) {
                Some(false) => false,
                Some(true) => true,
                _ => panic!("Local offset name not found (vstack)"),
            };
            if !is_fastcall || !param_found {
                if self.moved_locals.contains(local_name) && local_work_group != 999999 {
                    match local_type {
                        StackType::i32 => {
                            ret_str +=
                                &format!("\t__local uint {}[{}];\n", local_name, local_work_group);
                            ret_str += &format!("\t{}[thread_idx] = 0;\n", local_name);
                        }
                        StackType::i64 => {
                            ret_str +=
                                &format!("\t__local ulong {}[{}];\n", local_name, local_work_group);
                            ret_str += &format!("\t{}[thread_idx] = 0;\n", local_name);
                        }
                        StackType::f32 => {
                            ret_str +=
                                &format!("\t__local float {}[{}];\n", local_name, local_work_group);
                            ret_str += &format!("\t{}[thread_idx] = 0.0f;\n", local_name);
                        }
                        StackType::f64 => {
                            ret_str += &format!(
                                "\t__local double {}[{}];\n",
                                local_name, local_work_group
                            );
                            ret_str += &format!("\t{}[thread_idx] = 0.0;\n", local_name);
                        }
                        StackType::u128 => {
                            panic!("Storing v128 types in smem not yet supported!");
                        }
                    }
                } else {
                    match local_type {
                        StackType::i32 => {
                            ret_str += &format!("\tuint {} = 0;\n", local_name);
                        }
                        StackType::i64 => {
                            ret_str += &format!("\tulong {} = 0;\n", local_name);
                        }
                        StackType::f32 => {
                            ret_str += &format!("\tfloat {} = 0.0;\n", local_name);
                        }
                        StackType::f64 => {
                            ret_str += &format!("\tdouble {} = 0.0;\n", local_name);
                        }
                        StackType::u128 => {
                            ret_str += &format!("\tulong2 {} = (ulong2)(0);\n", local_name);
                        }
                    }
                }
            }
        }

        let mut idx = 0;
        for intermediate in &self.i32_stack.clone() {
            if self.demoted_intermediates.contains(intermediate) {
                ret_str += &format!(
                    "\t__local uint {}[{}];\n",
                    intermediate.replace("[thread_idx]", ""),
                    local_work_group
                );
            } else {
                ret_str += &format!("\tuint {} = 0;\n", intermediate);
            }
        }

        idx = 0;
        for intermediate in &self.i64_stack.clone() {
            if self.demoted_intermediates.contains(intermediate) {
                ret_str += &format!(
                    "\t__local ulong {}[{}];\n",
                    intermediate.replace("[thread_idx]", ""),
                    local_work_group
                );
            } else {
                ret_str += &format!("\tulong {} = 0;\n", intermediate);
            }
        }

        for intermediate in &self.f32_stack {
            ret_str += &format!("\tfloat {} = 0.0;\n", intermediate);
        }

        for intermediate in &self.f64_stack {
            ret_str += &format!("\tdouble {} = 0.0;\n", intermediate);
        }

        for intermediate in &self.u128_stack {
            ret_str += &format!("\tulong2 {} = (ulong2)(0);\n", intermediate);
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
            let offset: i32 =
                *self.local_offsets.get(local_name).unwrap() as i32 + self.param_offset;
            // only load parameters, locals are already zeroed out.
            if offset < 0 {
                let param_lookup_u32 = emit_read_u32_aligned(
                    &format!(
                        "(ulong)(stack_u32+{}+{})",
                        offset,
                        &emit_read_u32_aligned(
                            "(ulong)(stack_frames+*sfp)",
                            "(ulong)stack_frames",
                            "warp_idx"
                        )
                    ),
                    "(ulong)stack_u32",
                    "warp_idx",
                );
                let param_lookup_u64 = emit_read_u64_aligned(
                    &format!(
                        "(ulong)(stack_u32+{}+{})",
                        offset,
                        &emit_read_u32_aligned(
                            "(ulong)(stack_frames+*sfp)",
                            "(ulong)stack_frames",
                            "warp_idx"
                        )
                    ),
                    "(ulong)stack_u32",
                    "warp_idx",
                );
                let param_lookup_u64_upper = emit_read_u64_aligned(
                    &format!(
                        "(ulong)(stack_u32+{}+{})",
                        offset + 2,
                        &emit_read_u32_aligned(
                            "(ulong)(stack_frames+*sfp)",
                            "(ulong)stack_frames",
                            "warp_idx"
                        )
                    ),
                    "(ulong)stack_u32",
                    "warp_idx",
                );
                match local_type {
                    StackType::i32 => {
                        ret_str += &format!("\t\t{} = {};\n", local_name, param_lookup_u32);
                        if debug_call_print {
                            ret_str += &format!(
                                "\t\tprintf(\"param {}, value: %d\\n\", {});\n",
                                local_name, local_name
                            );
                        }
                    }
                    StackType::i64 => {
                        ret_str += &format!("\t\t{} = {};\n", local_name, param_lookup_u64);
                        if debug_call_print {
                            ret_str += &format!(
                                "\t\tprintf(\"param {}, value: %d\\n\", {});\n",
                                local_name, local_name
                            );
                        }
                    }
                    StackType::f32 => {
                        ret_str += &format!("\t\t{{\n");
                        ret_str += &format!("\t\t\tuint temp = {};\n", param_lookup_u32);
                        ret_str += &format!(
                            "\t\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(uint));\n",
                            local_name
                        );
                        ret_str += &format!("\t\t}}\n");
                        if debug_call_print {
                            ret_str += &format!(
                                "\t\tprintf(\"param {}, value: %f\\n\", {});\n",
                                local_name, local_name
                            );
                        }
                    }
                    StackType::f64 => {
                        ret_str += &format!("\t\t{{\n");
                        ret_str += &format!("\t\t\tulong temp = {};\n", param_lookup_u64);
                        ret_str += &format!(
                            "\t\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(ulong));\n",
                            local_name
                        );
                        ret_str += &format!("\t\t}}\n");
                        if debug_call_print {
                            ret_str += &format!(
                                "\t\tprintf(\"param {}, value: %f\\n\", {});\n",
                                local_name, local_name
                            );
                        }
                    }
                    StackType::u128 => {
                        ret_str += &format!("\t\t{}.x = {};\n", local_name, param_lookup_u64);
                        ret_str += &format!("\t\t{}.y = {};\n", local_name, param_lookup_u64_upper);
                        if debug_call_print {
                            ret_str += &format!(
                                "\t\tprintf(\"param {}.x, value: %ld\\n\", {}.x);\n",
                                local_name, local_name
                            );
                            ret_str += &format!(
                                "\t\tprintf(\"param {}.y, value: %ld\\n\", {}.y);\n",
                                local_name, local_name
                            );
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
        let panic_msg = &format!("vstack_pop err: {:?}", self);
        let get_panic_msg = &format!("vstack_pop get err: {:?}", self);

        self.total_stack_types.pop().expect(panic_msg);
        match t {
            StackType::i32 => {
                if self.i32_idx == 0 {
                    panic!("vstack_pop failed to pop i32 register: {:?}", self);
                }
                self.i32_idx -= 1;
                format!("{}", self.i32_stack.get(self.i32_idx).expect(get_panic_msg))
            }
            StackType::i64 => {
                if self.i64_idx == 0 {
                    panic!("vstack_pop failed to pop i64 register: {:?}", self);
                }
                self.i64_idx -= 1;
                format!("{}", self.i64_stack.get(self.i64_idx).expect(get_panic_msg))
            }
            StackType::f32 => {
                if self.f32_idx == 0 {
                    panic!("vstack_pop failed to pop f32 register: {:?}", self);
                }
                self.f32_idx -= 1;
                format!("{}", self.f32_stack.get(self.f32_idx).expect(get_panic_msg))
            }
            StackType::f64 => {
                if self.f64_idx == 0 {
                    panic!("vstack_pop failed to pop f64 register: {:?}", self);
                }
                self.f64_idx -= 1;
                format!("{}", self.f64_stack.get(self.f64_idx).expect(get_panic_msg))
            }
            StackType::u128 => {
                if self.u128_idx == 0 {
                    panic!("vstack_pop failed to pop u128 register: {:?}", self);
                }
                self.u128_idx -= 1;
                format!(
                    "{}",
                    self.u128_stack.get(self.u128_idx).expect(get_panic_msg)
                )
            }
        }
    }

    /*
     * Save the result of a computation to an intermediate value
     */
    pub fn vstack_alloc(&mut self, t: StackType) -> String {
        self.total_stack_types.push(t.clone());
        let panic_msg = &format!("vstack alloc err: {:?}", self);
        let ret = match t {
            StackType::i32 => {
                let alloc_val = self.i32_stack.get(self.i32_idx).expect(panic_msg);
                self.i32_idx += 1;
                format!("{}", alloc_val)
            }
            StackType::i64 => {
                let alloc_val = self.i64_stack.get(self.i64_idx).expect(panic_msg);
                self.i64_idx += 1;
                format!("{}", alloc_val)
            }
            StackType::f32 => {
                let alloc_val = self.f32_stack.get(self.f32_idx).expect(panic_msg);
                self.f32_idx += 1;
                format!("{}", alloc_val)
            }
            StackType::f64 => {
                let alloc_val = self.f64_stack.get(self.f64_idx).expect(panic_msg);
                self.f64_idx += 1;
                format!("{}", alloc_val)
            }
            StackType::u128 => {
                let alloc_val = self.u128_stack.get(self.u128_idx).expect(panic_msg);
                self.u128_idx += 1;
                format!("{}", alloc_val)
            }
        };

        // update max emitted context
        let curr_ctx: u32 =
            (self.i32_idx + self.i64_idx * 2 + self.f32_idx + self.f64_idx * 2 + self.u128_idx * 4)
                .try_into()
                .unwrap();
        if self.max_emitted_context < curr_ctx {
            self.max_emitted_context = curr_ctx as u32;
        }

        ret
    }

    /*
     * Peak the registers on the vstack, useful for unops
     */
    pub fn vstack_peak(&mut self, t: StackType, idx: usize) -> String {
        match t {
            StackType::i32 => {
                let alloc_val = self.i32_stack.get(self.i32_idx - 1 - idx).unwrap();
                format!("{}", alloc_val)
            }
            StackType::i64 => {
                let alloc_val = self.i64_stack.get(self.i64_idx - 1 - idx).unwrap();
                format!("{}", alloc_val)
            }
            StackType::f32 => {
                let alloc_val = self.f32_stack.get(self.f32_idx - 1 - idx).unwrap();
                format!("{}", alloc_val)
            }
            StackType::f64 => {
                let alloc_val = self.f64_stack.get(self.f64_idx - 1 - idx).unwrap();
                format!("{}", alloc_val)
            }
            StackType::u128 => {
                let alloc_val = self.u128_stack.get(self.u128_idx - 1 - idx).unwrap();
                format!("{}", alloc_val)
            }
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
            StackType::i32 => self.i32_idx == 0,
            StackType::i64 => self.i64_idx == 0,
            StackType::f32 => self.f32_idx == 0,
            StackType::f64 => self.f64_idx == 0,
            StackType::u128 => self.u128_idx == 0,
        }
    }

    // Get the Xth type from the top of the stack
    pub fn vstack_peak_type(&mut self, idx: usize) -> StackType {
        self.total_stack_types[self.total_stack_types.len() - 1 - idx].clone()
    }

    // get the size of the current stack frame
    pub fn stack_frame_size(&self) -> usize {
        self.i32_idx + (self.i64_idx * 2) + self.f32_idx + (self.f64_idx * 2) + (self.u128_idx * 4)
    }

    // Used to track which fastcalls we call (for use in register spill analysis)
    pub fn called_fastcalls(&self) -> HashSet<String> {
        self.called_fastcalls.clone()
    }

    // get the max possible size of a stack context for the current function
    pub fn max_stack_frame_size(&self) -> usize {
        (self.i32_stack.len() * 2)
            + (self.i64_stack.len() * 2)
            + (self.f32_stack.len() * 2)
            + (self.f64_stack.len() * 2)
            + (self.u128_stack.len() * 4)
    }

    pub fn get_max_emitted_context(&self) -> u32 {
        self.max_emitted_context
    }

    pub fn generate_intermediate_ranges(
        &self,
    ) -> (
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
    ) {
        // We want to find the most recent control stack snapshot that isn't virtual and generate that range
        // This is because we aren't saving the contexts of virtual snapshots
        let mut ctrl_stack_copy = self.control_stack_snapshots.clone();
        ctrl_stack_copy.reverse();

        /*
         * We either want the most recent non-virtual (Loop) snapshot delta or we want to save the
         * entire context.
         */

        for ctrl_stack in ctrl_stack_copy {
            if !ctrl_stack.is_virtual {
                let i32_range = ctrl_stack.i32_idx..self.i32_idx;
                let i64_range = ctrl_stack.i64_idx..self.i64_idx;
                let f32_range = ctrl_stack.f32_idx..self.f32_idx;
                let f64_range = ctrl_stack.f64_idx..self.f64_idx;
                let u128_range = ctrl_stack.u128_idx..self.u128_idx;

                return (i32_range, i64_range, f32_range, f64_range, u128_range);
            }
        }

        (
            0..self.i32_idx,
            0..self.i64_idx,
            0..self.f32_idx,
            0..self.f64_idx,
            0..self.u128_idx,
        )
    }

    pub fn emit_restore_local_cache(&self) -> String {
        let local_cache_start_offset = format!(
            "(ulong)(stack_u32+{}+{})",
            self.stack_frame_offset - self.local_cache_size,
            &emit_read_u32_aligned(
                "(ulong)(stack_frames+*sfp)",
                "(ulong)stack_frames",
                "warp_idx"
            )
        );

        format!(
            "\trestore_local_cache((uchar*)local_cache, {}, {}, (ulong)stack_u32, warp_idx);\n",
            self.local_cache_size, local_cache_start_offset
        )
    }

    /*
     * Generate the code to save the context of the current function
     * We can statically determine the minimum
     */

    pub fn save_context(&mut self, save_locals_only: bool, save_intermediate_only: bool) -> String {
        let mut ret_str = String::from("");

        // save the local_cache context
        /*
        let local_cache_start_offset = format!("(ulong)(stack_u32+{}+{})",
                                            self.stack_frame_offset - self.local_cache_size,
                                            &emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)stack_frames", "warp_idx"));
        ret_str += &format!("\tsave_local_cache((uchar*)local_cache, {}, {}, (ulong)stack_u32, warp_idx);\n", self.local_cache_size, local_cache_start_offset);
        */

        if self.fastcall_opt_possible {
            return ret_str;
        }

        let mut is_empty = true;

        ret_str += &format!("{{\n");
        ret_str += &format!("\tulong start = get_clock();\n");
        let sfp_val = emit_read_u32_aligned(
            "(ulong)(stack_frames+*sfp)",
            "(ulong)stack_frames",
            "warp_idx",
        );
        ret_str += &format!("\tuint sfp_ptr = {};\n", sfp_val);
        let sfp_ptr = "sfp_ptr";

        let map_idx = self.stack_frame_stack.last().unwrap();
        let locals_set = self.save_context_map.get(map_idx).unwrap().clone();

        // save the locals to the stack frame
        if !save_intermediate_only {
            for (tmp_local, ty) in self.local_types.iter() {
                let local: &mut String = &mut tmp_local.clone();
                let cache_idx: u32 = *self.local_offsets.get(local).unwrap();
                let offset: i32 =
                    *self.local_offsets.get(local).unwrap() as i32 + self.param_offset;

                if !locals_set.contains(local) {
                    continue;
                }

                is_empty = false;

                if self.moved_locals.contains(local) {
                    *local = format!("{}[thread_idx]", local);
                }

                match ty {
                    StackType::i32 if self.interleave == 1 => {
                        ret_str += &format!(
                            "\tif (local_cache[{}]) {};\n",
                            cache_idx,
                            &emit_write_u32_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset, &sfp_ptr,),
                                "(ulong)stack_u32",
                                &local,
                                "warp_idx"
                            )
                        );
                    }
                    StackType::i32 => {
                        ret_str += &format!(
                            "\tif (local_cache[{}]) {};\n",
                            cache_idx,
                            &emit_write_u32_fast(
                                &format!("(ulong)({}+{})*4", offset, &sfp_ptr,),
                                "(ulong)stack_base",
                                &local,
                            )
                        );
                    }
                    StackType::i64 => {
                        ret_str += &format!(
                            "\tif (local_cache[{}]) {};\n",
                            cache_idx,
                            &emit_write_u64_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset, &sfp_ptr,),
                                "(ulong)stack_u32",
                                &local,
                                "warp_idx"
                            )
                        );
                    }
                    StackType::f32 if self.interleave == 1 => {
                        ret_str += &format!("\tif (local_cache[{}]) {{\n", cache_idx);
                        ret_str += &format!("\t\tuint temp = 0;\n");
                        ret_str += &format!("\t\tfloat tempaddr = {};\n", local);
                        ret_str += &format!(
                            "\t\t___private_memcpy_nonmmu(&temp, &tempaddr, sizeof(float));\n"
                        );
                        ret_str += &format!(
                            "\t\t{};\n",
                            &emit_write_u32_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset, &sfp_ptr,),
                                "(ulong)stack_u32",
                                "temp",
                                "warp_idx"
                            )
                        );
                        ret_str += &format!("\t}}\n");
                    }
                    StackType::f32 => {
                        ret_str += &format!("\tif (local_cache[{}]) {{\n", cache_idx);
                        ret_str += &format!("\t\tuint temp = 0;\n");
                        ret_str += &format!("\t\tfloat tempaddr = {};\n", local);
                        ret_str += &format!(
                            "\t\t___private_memcpy_nonmmu(&temp, &tempaddr, sizeof(float));\n"
                        );
                        ret_str += &format!(
                            "\t\t{};\n",
                            &emit_write_u32_fast(
                                &format!("(ulong)({}+{})*4", offset, &sfp_ptr,),
                                "(ulong)stack_base",
                                "temp",
                            )
                        );
                        ret_str += &format!("\t}}\n");
                    }
                    StackType::f64 => {
                        ret_str += &format!("\tif (local_cache[{}]) {{\n", cache_idx);
                        ret_str += &format!("\t\tulong temp = 0;\n");
                        ret_str += &format!("\t\tdouble tempaddr = {};\n", local);
                        ret_str += &format!(
                            "\t\t___private_memcpy_nonmmu(&temp, &tempaddr, sizeof(double));\n"
                        );
                        ret_str += &format!(
                            "\t\t{};\n",
                            &emit_write_u64_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset, &sfp_ptr,),
                                "(ulong)stack_u32",
                                "temp",
                                "warp_idx"
                            )
                        );
                        ret_str += &format!("\t}}\n");
                    }
                    StackType::u128 => {
                        ret_str += &format!("\tif (local_cache[{}]) {{\n", cache_idx);
                        ret_str += &format!("\t\tulong temp_lower = {}.x;\n", local);
                        ret_str += &format!("\t\tulong temp_upper = {}.y;\n", local);
                        // write lower
                        ret_str += &format!(
                            "\t\t{};\n",
                            &emit_write_u64_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset, &sfp_ptr,),
                                "(ulong)stack_u32",
                                "temp_lower",
                                "warp_idx"
                            )
                        );
                        // write upper
                        ret_str += &format!(
                            "\t\t{};\n",
                            &emit_write_u64_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset + 2, &sfp_ptr,),
                                "(ulong)stack_u32",
                                "temp_upper",
                                "warp_idx"
                            )
                        );
                        ret_str += &format!("\t}}\n");
                    }
                }
                ret_str += &format!("\tlocal_cache[{}] = 0;\n", cache_idx);
            }
        }

        // Now go through and save the intermediate values
        if !save_locals_only {
            // We only want to save the intermediates on our current stack frame
            let (i32_range, i64_range, f32_range, f64_range, u128_range) =
                self.generate_intermediate_ranges();

            /*
             * The ctx saving structure is:
             *         (fn start, sfp pointer)
             * |  <parameters>  | <locals> | <intermediates> ....
             *
             */
            for idx in i32_range {
                is_empty = false;
                let i_name = self.i32_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();
                if self.interleave == 1 {
                    ret_str += &format!(
                        "\t{};\n",
                        &emit_write_u32_aligned(
                            &format!(
                                "(ulong)(stack_u32+{}+{}+{})",
                                &sfp_ptr, self.stack_frame_offset, i_name_offset
                            ),
                            "(ulong)stack_u32",
                            &i_name,
                            "warp_idx"
                        )
                    );
                } else {
                    ret_str += &format!(
                        "\t{};\n",
                        &emit_write_u32_fast(
                            &format!(
                                "(ulong)({}+{}+{})*4",
                                &sfp_ptr, self.stack_frame_offset, i_name_offset
                            ),
                            "(ulong)stack_base",
                            &i_name,
                        )
                    );
                }
            }

            for idx in i64_range {
                is_empty = false;
                let i_name = self.i64_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();
                ret_str += &format!(
                    "\t{};\n",
                    &emit_write_u64_aligned(
                        &format!(
                            "(ulong)(stack_u32+{}+{}+{})",
                            &sfp_ptr, self.stack_frame_offset, i_name_offset
                        ),
                        "(ulong)stack_u32",
                        &i_name,
                        "warp_idx"
                    )
                );
            }

            for idx in f32_range {
                is_empty = false;
                let i_name = self.f32_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();

                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tuint temp = 0;\n");
                ret_str += &format!(
                    "\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(float));\n",
                    &i_name
                );
                if self.interleave == 1 {
                    ret_str += &format!(
                        "\t\t{};\n",
                        &emit_write_u32_aligned(
                            &format!(
                                "(ulong)(stack_u32+{}+{}+{})",
                                &sfp_ptr, self.stack_frame_offset, i_name_offset
                            ),
                            "(ulong)stack_u32",
                            "temp",
                            "warp_idx"
                        )
                    );
                } else {
                    ret_str += &format!(
                        "\t\t{};\n",
                        &emit_write_u32_fast(
                            &format!(
                                "(ulong)({}+{}+{})*4",
                                &sfp_ptr, self.stack_frame_offset, i_name_offset
                            ),
                            "(ulong)stack_base",
                            "temp",
                        )
                    );
                }
                ret_str += &format!("\t}}\n");
            }

            for idx in f64_range {
                is_empty = false;
                let i_name = self.f64_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();

                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tulong temp = 0;\n");
                ret_str += &format!(
                    "\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n",
                    &i_name
                );
                ret_str += &format!(
                    "\t\t{};\n",
                    &emit_write_u64_aligned(
                        &format!(
                            "(ulong)(stack_u32+{}+{}+{})",
                            &sfp_ptr, self.stack_frame_offset, i_name_offset
                        ),
                        "(ulong)stack_u32",
                        "temp",
                        "warp_idx"
                    )
                );
                ret_str += &format!("\t}}\n");
            }

            for idx in u128_range {
                is_empty = false;
                let i_name = self.u128_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();

                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tulong temp_lower = {}.x;\n", i_name);
                ret_str += &format!("\t\tulong temp_upper = {}.y;\n", i_name);
                // lower
                ret_str += &format!(
                    "\t\t{};\n",
                    &emit_write_u64_aligned(
                        &format!(
                            "(ulong)(stack_u32+{}+{}+{})",
                            &sfp_ptr, self.stack_frame_offset, i_name_offset
                        ),
                        "(ulong)stack_u32",
                        "temp_lower",
                        "warp_idx"
                    )
                );
                // upper
                ret_str += &format!(
                    "\t\t{};\n",
                    &emit_write_u64_aligned(
                        &format!(
                            "(ulong)(stack_u32+{}+{}+{})",
                            sfp_ptr,
                            self.stack_frame_offset + 4,
                            i_name_offset
                        ),
                        "(ulong)stack_u32",
                        "temp_upper",
                        "warp_idx"
                    )
                );
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
        ret_str += &format!("\tulong end = get_clock();\n");
        ret_str += &format!("\t*overhead_tracker += end - start;\n");
        //ret_str += &format!("\tprintf(\"overhead_tracker: %lu, diff: %lu, end: %lu, start: %lu\\n\", *overhead_tracker, end - start, end, start);\n");
        ret_str += &format!("}}\n");

        if is_empty {
            ret_str = "".to_string();
        }

        ret_str
    }

    /*
     * Generate the code to restore the context of the current function
     */
    pub fn restore_context(
        &mut self,
        restore_locals_only: bool,
        restore_intermediates_only: bool,
    ) -> String {
        let mut ret_str = String::from("");
        let mut is_empty = true;

        if self.fastcall_opt_possible {
            return ret_str;
        }

        //if self.opt_loop_tracking.len() > 0 || self.encountered_first_restore_point {
        if self.opt_loop_tracking.len() > 0 {
            return ret_str;
        }

        ret_str += &format!("{{\n");
        ret_str += &format!("\tulong start = get_clock();\n");
        let sfp_val = emit_read_u32_aligned(
            "(ulong)(stack_frames+*sfp)",
            "(ulong)stack_frames",
            "warp_idx",
        );
        ret_str += &format!("\tuint sfp_ptr = {};\n", sfp_val);
        let sfp_ptr = "sfp_ptr";

        // First, load all locals from memory
        if !restore_intermediates_only {
            let map_idx = self.stack_frame_stack.last().unwrap();
            let locals_set = self.restore_context_map.get(map_idx).unwrap().clone();

            // ret_str += &self.emit_restore_local_cache();

            for (tmp_local, ty) in self.local_types.iter() {
                let local: &mut String = &mut tmp_local.clone();
                let offset: i32 =
                    *self.local_offsets.get(local).unwrap() as i32 + self.param_offset;
                let cache_idx: u32 = *self.local_offsets.get(local).unwrap();

                if !locals_set.contains(local) {
                    continue;
                }

                is_empty = false;

                if self.moved_locals.contains(local) {
                    *local = format!("{}[thread_idx]", local);
                }

                match ty {
                    // Only load locals if they *haven't* been written to
                    StackType::i32 if self.interleave == 1 => {
                        ret_str += &format!(
                            "\tif (!local_cache[{}]) {} = {};\n",
                            cache_idx,
                            local,
                            &emit_read_u32_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset, &sfp_ptr,),
                                "(ulong)stack_u32",
                                "warp_idx"
                            )
                        );
                    }
                    StackType::i32 => {
                        ret_str += &format!(
                            "\tif (!local_cache[{}]) {} = {};\n",
                            cache_idx,
                            local,
                            &emit_read_u32_fast(
                                &format!("(ulong)({}+{})*4", offset, &sfp_ptr,),
                                "(ulong)stack_base",
                            )
                        );
                    }
                    StackType::i64 => {
                        ret_str += &format!(
                            "\tif (!local_cache[{}]) {} = {};\n",
                            cache_idx,
                            local,
                            &emit_read_u64_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset, &sfp_ptr),
                                "(ulong)stack_u32",
                                "warp_idx"
                            )
                        );
                    }
                    StackType::f32 if self.interleave == 1 => {
                        ret_str += &format!("\tif (!local_cache[{}]) {{\n", cache_idx);
                        ret_str += &format!(
                            "\t\tuint temp = {};\n",
                            &emit_read_u32_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset, &sfp_ptr,),
                                "(ulong)stack_u32",
                                "warp_idx"
                            )
                        );
                        ret_str += &format!("\t\tfloat tempaddr = 0.0f;\n");
                        ret_str += &format!(
                            "\t\t___private_memcpy_nonmmu(&tempaddr, &temp, sizeof(float));\n"
                        );
                        ret_str += &format!("\t\t{} = tempaddr;\n", local);
                        ret_str += &format!("\t}}\n");
                    }
                    StackType::f32 => {
                        ret_str += &format!("\tif (!local_cache[{}]) {{\n", cache_idx);
                        ret_str += &format!(
                            "\t\tuint temp = {};\n",
                            &emit_read_u32_fast(
                                &format!("(ulong)({}+{})*4", offset, &sfp_ptr,),
                                "(ulong)stack_base",
                            )
                        );
                        ret_str += &format!("\t\tfloat tempaddr = 0.0f;\n");
                        ret_str += &format!(
                            "\t\t___private_memcpy_nonmmu(&tempaddr, &temp, sizeof(float));\n"
                        );
                        ret_str += &format!("\t\t{} = tempaddr;\n", local);
                        ret_str += &format!("\t}}\n");
                    }
                    StackType::f64 => {
                        ret_str += &format!("\tif (!local_cache[{}]) {{\n", cache_idx);
                        ret_str += &format!(
                            "\t\tulong temp = {};\n",
                            &emit_read_u64_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset, &sfp_ptr,),
                                "(ulong)stack_u32",
                                "warp_idx"
                            )
                        );
                        ret_str += &format!("\t\tdouble tempaddr = 0.0;\n");
                        ret_str += &format!(
                            "\t\t___private_memcpy_nonmmu(&tempaddr, &temp, sizeof(double));\n"
                        );
                        ret_str += &format!("\t\t{} = tempaddr;\n", local);
                        ret_str += &format!("\t}}\n");
                    }
                    StackType::u128 => {
                        ret_str += &format!("\tif (!local_cache[{}]) {{\n", cache_idx);
                        // restore lower
                        ret_str += &format!(
                            "\t\t{}.x = {};\n",
                            local,
                            &emit_read_u64_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset, &sfp_ptr,),
                                "(ulong)stack_u32",
                                "warp_idx"
                            )
                        );
                        // restore upper
                        ret_str += &format!(
                            "\t\t{}.y = {};\n",
                            local,
                            &emit_read_u64_aligned(
                                &format!("(ulong)(stack_u32+{}+{})", offset + 4, &sfp_ptr,),
                                "(ulong)stack_u32",
                                "warp_idx"
                            )
                        );
                        ret_str += &format!("\t}}\n");
                    }
                }
            }
        }

        if !restore_locals_only {
            // Now restore the intermediate values

            let (i32_range, i64_range, f32_range, f64_range, u128_range) =
                self.generate_intermediate_ranges();

            for idx in i32_range {
                let i_name = self.i32_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();
                is_empty = false;
                if self.interleave == 1 {
                    ret_str += &format!(
                        "\t{} = {};\n",
                        &i_name,
                        &emit_read_u32_aligned(
                            &format!(
                                "(ulong)(stack_u32+{}+{}+{})",
                                &sfp_ptr, self.stack_frame_offset, i_name_offset
                            ),
                            "(ulong)stack_u32",
                            "warp_idx"
                        )
                    );
                } else {
                    ret_str += &format!(
                        "\t{} = {};\n",
                        &i_name,
                        &emit_read_u32_fast(
                            &format!(
                                "(ulong)({}+{}+{})*4",
                                &sfp_ptr, self.stack_frame_offset, i_name_offset
                            ),
                            "(ulong)stack_base",
                        )
                    );
                }
            }

            for idx in i64_range {
                let i_name = self.i64_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();
                is_empty = false;

                ret_str += &format!(
                    "\t{} = {};\n",
                    &i_name,
                    &emit_read_u64_aligned(
                        &format!(
                            "(ulong)(stack_u32+{}+{}+{})",
                            &sfp_ptr, self.stack_frame_offset, i_name_offset
                        ),
                        "(ulong)stack_u32",
                        "warp_idx"
                    )
                );
            }

            for idx in f32_range {
                let i_name = self.f32_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();
                is_empty = false;

                ret_str += &format!("\t{{\n");
                if self.interleave == 1 {
                    ret_str += &format!(
                        "\t\tuint temp = {};\n",
                        &emit_read_u32_aligned(
                            &format!(
                                "(ulong)(stack_u32+{}+{}+{})",
                                &sfp_ptr, self.stack_frame_offset, i_name_offset
                            ),
                            "(ulong)stack_u32",
                            "warp_idx"
                        )
                    );
                } else {
                    ret_str += &format!(
                        "\t\tuint temp = {};\n",
                        &emit_read_u32_fast(
                            &format!(
                                "(ulong)({}+{}+{})*4",
                                &sfp_ptr, self.stack_frame_offset, i_name_offset
                            ),
                            "(ulong)stack_base",
                        )
                    );
                }
                ret_str += &format!(
                    "\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(float));\n",
                    &i_name
                );
                ret_str += &format!("\t}}\n");
            }

            for idx in f64_range {
                let i_name = self.f64_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();
                is_empty = false;

                ret_str += &format!("\t{{\n");
                ret_str += &format!(
                    "\t\tulong temp = {};\n",
                    &emit_read_u64_aligned(
                        &format!(
                            "(ulong)(stack_u32+{}+{}+{})",
                            &sfp_ptr, self.stack_frame_offset, i_name_offset
                        ),
                        "(ulong)stack_u32",
                        "warp_idx"
                    )
                );
                ret_str += &format!(
                    "\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(double));\n",
                    &i_name
                );
                ret_str += &format!("\t}}\n");
            }

            for idx in u128_range {
                let i_name = self.f64_stack.get(idx).unwrap();
                let i_name_offset = self.intermediate_offsets.get(i_name).unwrap();
                is_empty = false;

                // lower
                ret_str += &format!(
                    "\t{}.x = {};\n",
                    &i_name,
                    &emit_read_u64_aligned(
                        &format!(
                            "(ulong)(stack_u32+{}+{}+{})",
                            &sfp_ptr, self.stack_frame_offset, i_name_offset
                        ),
                        "(ulong)stack_u32",
                        "warp_idx"
                    )
                );
                // upper
                ret_str += &format!(
                    "\t{}.y = {};\n",
                    &i_name,
                    &emit_read_u64_aligned(
                        &format!(
                            "(ulong)(stack_u32+{}+{}+{})",
                            &sfp_ptr,
                            self.stack_frame_offset + 4,
                            i_name_offset
                        ),
                        "(ulong)stack_u32",
                        "warp_idx"
                    )
                );
            }
        }

        ret_str += &format!("\tulong end = get_clock();\n");
        ret_str += &format!("\t*overhead_tracker += end - start;\n");
        ret_str += &format!("}}\n");

        if is_empty {
            ret_str = "".to_string();
        }

        ret_str
    }

    /*
     * This function is used to restore the context of a block/loop with a return value
     */
    pub fn restore_context_with_result_val(
        &mut self,
        restore_locals_only: bool,
        restore_intermediates_only: bool,
        ret_val_type: Option<StackType>,
    ) -> String {
        // Temporarily account for an extra value at the top of the stack
        match ret_val_type {
            Some(StackType::i32) => self.i32_idx -= 1,
            Some(StackType::i64) => self.i64_idx -= 1,
            Some(StackType::f32) => self.f32_idx -= 1,
            Some(StackType::f64) => self.f64_idx -= 1,
            Some(StackType::u128) => self.u128_idx -= 1,
            None => (),
        }

        let result = self.restore_context(restore_locals_only, restore_intermediates_only);
        // restore prev val
        match ret_val_type {
            Some(StackType::i32) => self.i32_idx += 1,
            Some(StackType::i64) => self.i64_idx += 1,
            Some(StackType::f32) => self.f32_idx += 1,
            Some(StackType::f64) => self.f64_idx += 1,
            Some(StackType::u128) => self.u128_idx += 1,
            None => (),
        }

        result
    }
}
