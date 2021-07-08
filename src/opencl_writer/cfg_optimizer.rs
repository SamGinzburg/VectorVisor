use crate::opencl_writer::compile_stats::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

/*
 * Get the names of:
 * - Called functions inside loops, Called functions
 */
fn get_called_funcs(func: &wast::Func, fastcalls: &HashSet<String>, func_map: &HashMap<String, &wast::Func>, imports_map: &HashMap<&str, (&str, Option<&str>, wast::ItemSig)>, visited_funcs: &mut HashSet<String>) -> (Vec<String>, Vec<String>) {
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
        (wast::FuncKind::Inline{locals, expression}, Some(_id), _typeuse) => {
            for instr in expression.instrs.iter() {
                match instr {
                    wast::Instruction::Call(idx) => {
                        let id = match idx {
                            wast::Index::Id(id) => id.name(),
                            _ => panic!("Unable to get Id for function call: {:?}", idx),
                        };

                        // Only count non-fastcalls and non-syscalls
                        if !fastcalls.contains(id) && !imports_map.contains_key(id) {
                            if nested_loop_count > 0 {
                                fn_call_in_loop.push(id.to_string());
                                // Also track nested function calls
                                if !visited_funcs.contains(id) {
                                    visited_funcs.insert(id.to_string());
                                    let (nested_fn_call_in_loop, nested_fn_calls) = get_called_funcs(func_map.get(id).unwrap(), fastcalls, func_map, imports_map, visited_funcs);
                                    fn_call_in_loop.extend(nested_fn_call_in_loop);
                                    fn_call.extend(nested_fn_calls);    
                                }
                            } else {
                                fn_call.push(id.to_string());
                                if !visited_funcs.contains(id) {
                                    visited_funcs.insert(id.to_string());
                                    let (nested_fn_call_in_loop, nested_fn_calls) = get_called_funcs(func_map.get(id).unwrap(), fastcalls, func_map, imports_map, visited_funcs);
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
                    wast::Instruction::End(_) => {
                        if control_stack.pop().unwrap() {
                            nested_loop_count -= 1;
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

pub fn form_partitions(num_funcs_in_partition: u32, instr_count_limit: u32, func_names: Vec<&String>, fastcalls: &HashSet<String>, func_map: &HashMap<String, &wast::Func>, imports_map: &HashMap<&str, (&str, Option<&str>, wast::ItemSig)>, kernel_compile_stats: &mut HashMap<u32, (u32, u32, u32, u32, u32, u32)>) -> Vec<(u32, HashSet<String>)> {

    let mut func_set = HashSet::<&String>::from_iter(func_names);
    let mut partitions = vec![];
    let mut partition_idx = 0;
    /*
     * 1) Create a set of all functions in the program (global BTreeSet G)
     * 2) Pop a function (F) out of the global set of functions
     * 3) Run analysis pass on the function
     *    Keep 2 queues (one for high priority calls (calls inside loops), the rest in the other)
     * 4) Using the two queues, prioritize merging from the priority queue first, then merge the rest
     * 5) Once a grouping is formed, remove the selected functions from G
     * 6) Go to 2 and repeat until the global set G is empty
     */

    let mut include_limit: HashMap<String, u32> = HashMap::new();
    let func_copy_limit = 2;

    let mut func_list = Vec::from_iter(func_set.clone());
    while let Some(f_name) = func_list.pop() {
        let mut current_partition = HashSet::<String>::new();

        // Check to see if this function already got included into a partition
        if !func_set.contains(&f_name) {
            continue
        }

        current_partition.insert(String::from(f_name));

        let (loop_called_fns, called_fns) = get_called_funcs(func_map.get(&f_name.clone()).unwrap(), fastcalls, func_map, imports_map, &mut HashSet::new());
        
        let mut current_partition_count = 0;
        let mut current_instruction_count = 0;

        let (instr_count, _, _, _, _, _) = function_stats(func_map.get(&f_name.clone()).unwrap(), fastcalls, func_map);
        current_instruction_count += instr_count;

        // Now we can form the partition itself

         for func in loop_called_fns {
             let (instr_count, _, _, _, _, _) = function_stats(func_map.get(&func.clone()).unwrap(), fastcalls, func_map);
             /*
              * If the func is the following:
              * - Func is below inclusion limit
              * - Doesn't violate the partition count constraint
              * - Doesn't violate the instruction count constraint
              */
              let func_copies = include_limit.get(&func).cloned().unwrap_or(0);
              if current_partition_count < num_funcs_in_partition &&
                 current_instruction_count + instr_count <= instr_count_limit &&
                 func_copies < func_copy_limit {
                    // add the func to the set
                    current_partition.insert(String::from(&func));
                    func_set.remove(&func);
                    include_limit.insert(func, func_copies + 1);
                    current_partition_count += 1;
                    current_instruction_count += instr_count;
                }
         }

         for func in called_fns {
            let (instr_count, _, _, _, _, _) = function_stats(func_map.get(&func.clone()).unwrap(), fastcalls, func_map);
            /*
             * If the func is the following:
             * - Func is below inclusion limit
             * - Doesn't violate the partition count constraint
             * - Doesn't violate the instruction count constraint
             */
            let func_copies = include_limit.get(&func).cloned().unwrap_or(0);
             if current_partition_count < num_funcs_in_partition &&
                current_instruction_count + instr_count <= instr_count_limit &&
                func_copies < func_copy_limit {
                   // add the func to the set
                   current_partition.insert(String::from(&func));
                   func_set.remove(&func);
                   include_limit.insert(func, func_copies + 1);
                   current_partition_count += 1;
                   current_instruction_count += instr_count;
                }
        }


        // If we couldn't form a group with anyone, set this function aside for now
        // We will get back to it later if we can't find a partition for it
        if current_partition.len() == 1 {
            continue;
        }

        // only remove this if we managed to form a partition with at least 1 other function
        func_set.remove(f_name);

        partitions.push((partition_idx, current_partition.clone()));

        // update the kernel compile stats tracking object
        dbg!(&current_instruction_count);
        kernel_compile_stats.insert(partition_idx, (current_instruction_count, 0, 0, 0, 0, 0));

        partition_idx += 1;
    }

    // The remaining functions are either fastcalls, or functions
    for func in func_set {
        let mut set = HashSet::new();
        set.insert(func.clone());
        partitions.push((partition_idx, set));

        let (instr_count, _, _, _, _, _) = function_stats(func_map.get(&func.clone()).unwrap(), fastcalls, func_map);

        kernel_compile_stats.insert(partition_idx, (instr_count, 0, 0, 0, 0, 0));
        partition_idx += 1;
    }

    dbg!(&partitions.len());
    partitions
}
