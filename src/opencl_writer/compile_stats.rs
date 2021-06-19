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


pub fn function_stats(func: &wast::Func, fastcalls: &HashSet<String>, func_map: &HashMap<String, &wast::Func>) -> (u32, u32, u32, u32, u32, u32) {

    let mut total_instr_count: u32 = 0;
    let mut total_func_count: u32 = 0;
    let mut total_fastcall_count: u32 = 0;
    let mut total_indirect_count: u32 = 0;
    let mut total_block_count: u32 = 0;
    let mut total_loop_count: u32 = 0;

    match (&func.kind, &func.id, &func.ty) {
        (wast::FuncKind::Import(_), _, _) => {
            // In this case, we have an InlineImport of the form:
            // (func (type 3) (import "foo" "bar"))
            panic!("InlineImport functions not yet implemented");
        },
        (wast::FuncKind::Inline{locals, expression}, Some(id), typeuse) => {
            total_instr_count = expression.instrs.len().try_into().unwrap();
            for instr in expression.instrs.iter() {
                match instr {
                    wast::Instruction::Call(idx) => {
                        let id = match idx {
                            wast::Index::Id(id) => id.name(),
                            _ => panic!("Unable to get Id for function call: {:?}", idx),
                        };

                        if fastcalls.contains(id) {
                            total_fastcall_count += 1;
                            // get the func
                            let func = func_map.get(id).unwrap();
                            // Look up the compile stats for the fastcall and add it to our own
                            let (nested_total_instr_count, nested_total_func_count, nested_total_fastcall_count, nested_total_indirect_count, nested_total_block_count, nested_total_loop_count) = function_stats(func, fastcalls, func_map);
                            total_instr_count += nested_total_instr_count;
                            total_func_count += nested_total_func_count;
                            total_indirect_count += nested_total_indirect_count;
                            total_loop_count += nested_total_loop_count;
                            total_block_count += nested_total_block_count;
                        } else {
                            total_func_count += 1;
                        }
                    },
                    wast::Instruction::CallIndirect(_) => total_indirect_count += 1,
                    wast::Instruction::Block(_) => total_block_count += 1,
                    wast::Instruction::Loop(_) => total_loop_count += 1,
                    _ => (),
                }
            }
        },
        _ => panic!("Unknown function type in function_stats"),
    };

    

    (total_instr_count, total_func_count, total_fastcall_count, total_indirect_count, total_block_count, total_loop_count)
}