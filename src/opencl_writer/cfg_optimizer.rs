use crate::opencl_writer::compile_stats::*;
use crate::opencl_writer::OpenCLCWriter;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::convert::TryInto;
use crate::opencl_writer::format_fn_name;

/*
 * Get the names of:
 * - Called functions inside loops, Called functions
 */
pub fn get_called_funcs(writer_ctx: &OpenCLCWriter, indirect_call_mapping: &Vec<String>, func: &wast::Func, fastcalls: &HashSet<String>, func_map: &HashMap<String, wast::Func>, imports_map: &HashMap<String, (&str, Option<&str>, wast::ItemSig)>, visited_funcs: &mut HashSet<String>) -> (Vec<String>, Vec<String>) {
    let mut fn_call_in_loop: Vec<String> = vec![];
    let mut fn_call: Vec<String> = vec![];

    let mut control_stack: Vec<bool> = vec![];
    let mut nested_loop_count = 0;
    match (&func.kind, &func.id, &func.ty) {
        (wast::FuncKind::Import(_), _, _) => {
            // In this case, we have an InlineImport of the form:
            // (func (type 3) (import "foo" "bar"))
            panic!("InlineImport functions not yet implemented");
        },
        (wast::FuncKind::Inline{locals, expression}, _, _typeuse) => {
            for instr in expression.instrs.iter() {
                match instr {
                    wast::Instruction::Call(idx) => {
                        let id: &str = &match idx {
                            wast::Index::Id(id) => format_fn_name(id.name()),
                            wast::Index::Num(val, _) => format!("func_{}", val),
                        };

                        // Only count non-fastcalls and non-syscalls
                        if !fastcalls.contains(id) && !imports_map.contains_key(id) {
                            if nested_loop_count > 0 {
                                fn_call_in_loop.push(id.to_string());
                                // Also track nested function calls
                                if !visited_funcs.contains(id) {
                                    visited_funcs.insert(id.to_string());
                                    let (nested_fn_call_in_loop, nested_fn_calls) = get_called_funcs(writer_ctx, indirect_call_mapping, func_map.get(id).unwrap(), fastcalls, func_map, imports_map, visited_funcs);
                                    fn_call_in_loop.extend(nested_fn_call_in_loop);
                                    fn_call.extend(nested_fn_calls);    
                                }
                            } else {
                                fn_call.push(id.to_string());
                                if !visited_funcs.contains(id) {
                                    visited_funcs.insert(id.to_string());
                                    let (nested_fn_call_in_loop, nested_fn_calls) = get_called_funcs(writer_ctx, indirect_call_mapping, func_map.get(id).unwrap(), fastcalls, func_map, imports_map, visited_funcs);
                                    fn_call_in_loop.extend(nested_fn_call_in_loop);
                                    fn_call.extend(nested_fn_calls);    
                                }
                            }
                        }
                    },
                    wast::Instruction::Block(_) => {
                        // track blocks too, since they are part of the control stack
                        control_stack.push(false);
                    },
                    wast::Instruction::Loop(_) => {
                        // track if we are in a loop or not
                        nested_loop_count += 1;
                        control_stack.push(true);
                    },
                    wast::Instruction::If(_) => {
                        control_stack.push(false);
                    },
                    wast::Instruction::End(_) => {
                        if control_stack.pop().unwrap() {
                            nested_loop_count -= 1;
                        }
                    },
                    wast::Instruction::CallIndirect(call_indirect) => {
                        // Add possible indirect targets to the partition that match type
                        // signatures
                        match (call_indirect.ty.index.as_ref(), call_indirect.ty.inline.as_ref()) {
                            (Some(index), _) => {
                                let type_index = match index {
                                    wast::Index::Num(n, _) => format!("t{}", n),
                                    wast::Index::Id(i) => i.name().to_string(),
                                };

                                let indirect_func_type = match writer_ctx.types.get(&type_index).unwrap() {
                                    wast::TypeDef::Func(ft) => ft,
                                    _ => panic!("Indirect call cannot have a type of something other than a func"),
                                };

                                for f_name in indirect_call_mapping {
                                    let func_type_signature = &writer_ctx.func_map.get(&f_name as &str).unwrap().ty;

                                    let func_type_index = match func_type_signature.index {
                                        Some(wast::Index::Id(id)) => id.name().to_string(),
                                        Some(wast::Index::Num(val, _)) => format!("t{}", val),
                                        None => panic!("Only type indicies supported for call_indirect in get_called_funcs: {:?}", func_type_signature),
                                    };

                                    // We add the speculated call target if the type signature
                                    // matches. We also don't recurse, as that seems to add a lot
                                    // of noise.
                                    // We explicitly allow non-fastcalls to be targeted here
                                    if func_type_index == type_index {
                                        if !imports_map.contains_key(&f_name as &str) {
                                            if nested_loop_count > 0 {
                                                fn_call_in_loop.push(f_name.to_string());
                                            } else {
                                                fn_call.push(f_name.to_string());
                                            }
                                        }
                                    }
                                }
                            },
                            _ => (),
                        }
                    },
                    _ => (),
                }
            }
        },
        _ => (),
    }

    (Vec::from_iter(HashSet::<String>::from_iter(fn_call_in_loop)), Vec::from_iter(HashSet::<String>::from_iter(fn_call)))
}


/*
 * When compiling partitioned applications, if we do not perform any optimizations
 * each GPU kernel contains a single function, and function-level divergence is handled by the VMM.
 *
 * This is done to avoid exceedingly large compile-times and memory usage by the GPU JIT compiler.
 *
 * However, partitioning to N=1 functions per kernel results in high device queueing times & VMM overhead.
 * It is usually better to let the GPU handle divergence (if possible).
 *
 * In order to compromise these two tradeoffs (longer compiles vs more efficient execution),
 * we employ a more complex partitioning proceedure that groups functions that call 
 * each other into the same OpenCL kernel.
 */

pub fn form_partitions(writer_ctx: &OpenCLCWriter, num_funcs_in_partition: u32, instr_count_limit: u32, func_copy_limit: u32, func_names: Vec<String>, fastcalls: &HashSet<String>, func_map: &HashMap<String, wast::Func>, imports_map: &HashMap<String, (&str, Option<&str>, wast::ItemSig)>, kernel_compile_stats: &mut HashMap<u32, (u32, u32, u32, u32, u32, u32)>, indirect_call_mapping: &HashMap<u32, &wast::Index>) -> Vec<(u32, HashSet<String>)> {

    let mut func_set = HashSet::<String>::from_iter(func_names.clone());
    let mut partitions: Vec<(u32, HashSet<String>)> = vec![];
    let mut partition_idx = 0;

    let mut indirect_call_mapping_formatted: Vec<String> = vec![];
    for func_id in indirect_call_mapping.values() {
        let f_name = match func_id {
            wast::Index::Id(id) => format_fn_name(id.name()),
            wast::Index::Num(val, _) => format!("func_{}", val),
        };
        indirect_call_mapping_formatted.push(f_name);
    };

    /*
     * 1) Create a set of all functions in the program (global BTreeSet G)
     * 2) Pop a function (F) out of the global set of functions
     * 3) Run analysis pass on the function
     *    Keep 2 queues (one for high priority calls (calls inside loops), the rest in the other)
     * 4) Using the two queues, prioritize merging from the priority queue first, then merge the rest
     * 5) Once a grouping is formed, remove the selected functions from G
     * 6) Go to 2 and repeat until the global set G is empty
     */

    // track duplicate code
    let mut include_limit: HashMap<String, u32> = HashMap::new();
    // track which partitions have a function duplicated in
    let mut dc_partitions: HashMap<String, HashSet<u32>> = HashMap::new();

    // we don't need to de-dup function names with a set
    let mut func_list = Vec::from_iter(func_names.clone());

    while let Some(f_name) = func_list.pop() {
        let mut current_partition = HashSet::<String>::new();

        // Check to see if this function already got included into a partition
        if !func_set.contains(&f_name) {
            continue
        }

        current_partition.insert(String::from(f_name.clone()));

        let (loop_called_fns, called_fns) = get_called_funcs(writer_ctx, &indirect_call_mapping_formatted, func_map.get(&f_name.clone()).unwrap(), fastcalls, func_map, imports_map, &mut HashSet::new());

        let mut current_partition_count = 0;
        let mut current_instruction_count = 0;

        let (instr_count, _, _, _, _, _) = function_stats(writer_ctx, f_name.clone(), func_map.get(&f_name.clone()).unwrap(), fastcalls, func_map, indirect_call_mapping);
        current_instruction_count += instr_count;

        // Now we can form the partition itself

         for func in loop_called_fns {
             let (instr_count, _, _, _, _, _) = function_stats(writer_ctx, func.clone(), func_map.get(&func.clone()).unwrap(), fastcalls, func_map, indirect_call_mapping);
             /*
              * If the func is the following:
              * - Func is below inclusion limit
              * - Doesn't violate the partition count constraint
              * - Doesn't violate the instruction count constraint
              * - Doesn't violate the func copies count constraint
              */
              let func_copies = include_limit.get(&func).cloned().unwrap_or(0);
              if current_partition_count < num_funcs_in_partition &&
                 current_instruction_count + instr_count <= instr_count_limit &&
                 func_copies < func_copy_limit {
                    // add the func to the set
                    current_partition.insert(String::from(&func));
                    func_set.remove(&func);
                    include_limit.insert(func.clone(), func_copies + 1);
                    current_partition_count += 1;
                    current_instruction_count += instr_count;

                    // Track duplicated code across partitions whenever we insert a function
                    let mut temp = HashSet::new();
                    temp.insert(partition_idx);
                    let mut prev_partitions = dc_partitions.get(&func).cloned().unwrap_or(temp);
                    prev_partitions.insert(partition_idx);
                    dc_partitions.insert(func, prev_partitions);
                }
         }

         for func in called_fns {
            let (instr_count, _, _, _, _, _) = function_stats(writer_ctx, func.clone(), func_map.get(&func.clone()).unwrap(), fastcalls, func_map, indirect_call_mapping);
            /*
             * If the func is the following:
             * - Func is below inclusion limit
             * - Doesn't violate the partition count constraint
             * - Doesn't violate the instruction count constraint
             * - Doesn't violate the func copies count constraint
             */
            let func_copies = include_limit.get(&func).cloned().unwrap_or(0);
            if current_partition_count < num_funcs_in_partition &&
               current_instruction_count + instr_count <= instr_count_limit &&
               func_copies < func_copy_limit {
                // add the func to the set
                current_partition.insert(String::from(&func));
                func_set.remove(&func);
                include_limit.insert(func.clone(), func_copies + 1);
                current_partition_count += 1;
                current_instruction_count += instr_count;

                // Track duplicated code across partitions whenever we insert a function
                let mut temp = HashSet::new();
                temp.insert(partition_idx);
                let mut prev_partitions = dc_partitions.get(&func).cloned().unwrap_or(temp);
                prev_partitions.insert(partition_idx);
                dc_partitions.insert(func, prev_partitions);
            }
        }


        // If we couldn't form a group with anyone, set this function aside for now
        // We will get back to it later if we can't find a partition for it
        if current_partition.len() == 1 {
            continue;
        }

        // only remove this if we managed to form a partition with at least 1 other function
        func_set.remove(&f_name);

        /*
         * At this point we have formed the partition, but we may now have duplicate functions
         * across multiple partitions.
         *
         * To mitigate this, we can now check to see if we can merge this partition we just formed
         * into one of the partitions with the duplicated functions.
         *
         * 1) Get the set S of partitions that have duplicate code
         * 2) for each each partition in set S:
         *      2.1) check if merging our new partition into that partition violates constraints
         *      2.2) if not, perform the insertion
         *      2.3) else, check subsequent partitions to see if we can merge
         * 3) if after scanning all prior partitions w/o merging, insert new partition
         *
         */
        let prev_current_partition = current_partition.clone();
        let mut duplicate_funcs = HashSet::new();
        for func in current_partition.iter() {
            let func_copies = include_limit.get(func).cloned().unwrap_or(0);
            if func_copies > 2 {
                let prev_partitions = dc_partitions.get(func).unwrap();
                duplicate_funcs.extend(prev_partitions.clone());
            }
        }

        let mut modified_prior_part = false;
        for prior_partition in duplicate_funcs.iter() {
            if *prior_partition != partition_idx {
                let (prev_partition_idx, prev_partition_set) = &partitions[*prior_partition as usize].clone();
                // get prev instruction count
                let (prev_instr_count, _, _, _, _, _) = kernel_compile_stats.get(prev_partition_idx).unwrap();
                let part_size: u32 = prev_partition_set.len().try_into().unwrap();
                if current_partition_count + part_size < num_funcs_in_partition &&
                   current_instruction_count + prev_instr_count <= instr_count_limit  {
                    // we can merge the partitions!
                    // Combine the hashsets
                    current_partition.extend(prev_partition_set.clone());
                    // update partitions & update kernel compile stats 
                    partitions[*prior_partition as usize] = (*prev_partition_idx, current_partition.clone());
                    let stats = (current_instruction_count + prev_instr_count, 0, 0, 0, 0, 0);
                    kernel_compile_stats.insert(*prev_partition_idx, stats);
                    modified_prior_part = true;
                    break;
                }
            }
        }

        if !modified_prior_part {
            partitions.push((partition_idx, current_partition.clone()));
            // update the kernel compile stats tracking object
            kernel_compile_stats.insert(partition_idx, (current_instruction_count, 0, 0, 0, 0, 0));
            partition_idx += 1;
        } else {
            // cleanup include_limit & dc_partitions
            // For all funcs in the old partition, cleanup dc_part. as old part. no longer exists
            for func in prev_current_partition.iter() {
                let mut prev_partitions = dc_partitions.get(&func.clone()).unwrap_or(&HashSet::new()).clone();
                prev_partitions.remove(&partition_idx);
                dc_partitions.insert(func.clone(), prev_partitions.clone());
            }
            // Now cleanup all funcs in the intersection between the old & new partitions
            // The include limits should be decremented
            let part_intersection = prev_current_partition.intersection(&current_partition);
            for func in part_intersection {
                let prev_limit = include_limit.get(func).cloned().unwrap_or(0);
                if prev_limit > 0 {
                    include_limit.insert(func.clone(), prev_limit - 1);
                }
            }
        }
    }

    // The remaining functions are either fastcalls, or functions
    for func in func_set {
        let mut set = HashSet::new();
        set.insert(func.clone());
        partitions.push((partition_idx, set));

        let (instr_count, _, _, _, _, _) = function_stats(writer_ctx, func.clone(), func_map.get(&func.clone()).unwrap(), fastcalls, func_map, indirect_call_mapping);

        kernel_compile_stats.insert(partition_idx, (instr_count, 0, 0, 0, 0, 0));
        partition_idx += 1;
    }

    partitions
}
