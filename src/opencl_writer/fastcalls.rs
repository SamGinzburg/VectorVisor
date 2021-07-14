use crate::opencl_writer;
use crate::opencl_writer::WASI_SNAPSHOT_PREVIEW1;

use std::collections::HashSet;

/*
 * Our CPS-style transform is too expensive for most function calls, so we perform some basic static analysis
 * to identify which calls can be translated into 'fastcalls'.
 * 
 * Fastcalls must be functions with the following properties:
 * - Not the "_start" function
 * - Fastcalls may not perform hypercalls
 * - Fastcalls may not be the target of an indirect function call
 * - Fastcalls may not perform indirect calls
 * - Fastcalls can only call other fastcalls
 * - Fastcalls may not perform recursion
 * 
 * During the search functions may be in one of three states:
 * - Known to be possible to emit as a fastcall
 * - Known to not be a fastcall
 * - Amiguous (blocked on a list of amiguous functions):
 *      ex:
 *          fn A:
 *              call B
 *          fn B:
 *              call A
 *      Here, A is blocked on B, and B is blocked on A.
 * 
 * 
 * At some point during our main pass, we will end up in a steady state where we have only ambiguous functions remaining (or none).
 * 
 * We then perform a second pass to identify if any ambiguous functions can be fastcall-optimized as well.
 * 
 * TODO: most of the functions we would like to optimize are actually stopped by panic! code - this can special cased 
 * 
 */

#[derive(Clone)]
pub enum FastcallPassStatus {
    fastcall_false(String), // The string is for debugging the compiler
    fastcall_true,
    fastcall_ambiguous(HashSet<String>)
}


 fn is_fastcall(writer: &opencl_writer::OpenCLCWriter, func: &wast::Func, fastcall_set: &mut HashSet<String>, indirect_calls: &mut HashSet<String>) -> FastcallPassStatus {
    match (&func.kind, &func.id, &func.ty) {
        (wast::FuncKind::Import(_), _, _) => {
            panic!("InlineImport functions not yet implemented (fastcall pass)");
        },
        (wast::FuncKind::Inline{locals, expression}, Some(id), _typeuse) => {

            // Is this function the start function?
            if id.name() == "_start" {
                return FastcallPassStatus::fastcall_false(String::from("is start fn"))
            }

            // Is this function the target of an indirect call?
            if indirect_calls.contains(id.name()) {
                // return FastcallPassStatus::fastcall_false(String::from("indirect call target"))
            }

            // Is this function in the indirect call table?
            let mut ambiguous_dep_list = HashSet::new();
            for instruction in expression.instrs.iter() {
                match instruction {
                    wast::Instruction::Call(idx) => {
                        let id = match idx {
                            wast::Index::Id(id) => id.name(),
                            _ => panic!("Unable to get Id for function call: {:?}", idx),
                        };

                        // Fastcalls may not perform recursion
                        if id.to_string() == func.id.unwrap().name().to_string() {
                            return FastcallPassStatus::fastcall_false(String::from("performs recursion"))
                        }

                        // Is this a hypercall?
                        match writer.imports_map.get(id) {
                            Some((wasi_api, Some(wasi_fn_name), _)) => {    
                                match (wasi_api, WASI_SNAPSHOT_PREVIEW1.get(wasi_fn_name)) {
                                    (_, Some(true)) => {
                                        // if we found a WASI hypercall...
                                        return FastcallPassStatus::fastcall_false(String::from("performs hypercall"))
                                    },
                                    _ => (),
                                }
                            },
                            _ => (),
                        }

                        // Is this a fastcall or not?
                        if fastcall_set.contains(&id.to_string()) {
                            // If this is a fastcall, then keep checking the rest of the function
                            continue
                        } else {
                            // else if this is an unknown call & not a hypercall, mark it as ambiguous for now
                            ambiguous_dep_list.insert(id.to_string());
                        }
                    },
                    wast::Instruction::CallIndirect(_) => {
                        return FastcallPassStatus::fastcall_false(String::from("performs indirect call"))
                    },
                    _ => (),
                }
            }
            if ambiguous_dep_list.clone().len() > 0 {
                return FastcallPassStatus::fastcall_ambiguous(ambiguous_dep_list)
            }
        },
        (_, _, _) => panic!("Inline function must always have a valid identifier in wasm"),
    }

    FastcallPassStatus::fastcall_true
 }

 /*
  * Check all the functions in the program to see which ones we can convert into fastcalls
  * Returns a set of function IDs that can be converted
  */
 pub fn compute_fastcall_set(writer: &opencl_writer::OpenCLCWriter, func_list: Vec<&wast::Func>, indirect_calls: &mut HashSet<String>) -> HashSet<String> {
    let mut called_funcs = HashSet::new();
    let mut known_bad_calls = HashSet::new();

    let mut fastcall_count = 0;
    let mut ambiguous_fastcalls;

    loop {
        //println!("Fastcall analysis pass, found: {:?} functions to optimize", fastcall_count);
        ambiguous_fastcalls = vec![];
        for func in &func_list {
            let is_fastcall = is_fastcall(writer, func, &mut called_funcs, indirect_calls);
            match is_fastcall {
                FastcallPassStatus::fastcall_true => {
                    called_funcs.insert(func.id.unwrap().name().to_string());
                },
                FastcallPassStatus::fastcall_ambiguous(fastcall_ambiguous) => {
                    ambiguous_fastcalls.push((func.id.unwrap().name().to_string(), fastcall_ambiguous));
                },
                FastcallPassStatus::fastcall_false(_) => {
                    known_bad_calls.insert(func.id.unwrap().name().to_string());
                }
            }
        }

        // If there has been no change in the amount of fastcall funcs, then we have reached a stable state and are done
        if fastcall_count == called_funcs.clone().len() {
            break;
        } else {
            fastcall_count = called_funcs.clone().len();
        }
    }

    // Loop through the ambiguous calls, removing any that make bad calls, and adding those calls to the bad call set
    let mut last_bad_call_count = 0;
    loop {
        // Keep going until we propogate all of the bad calls through
        for (call, set) in ambiguous_fastcalls.clone().iter() {
            let intersection = set.intersection(&known_bad_calls);
            // If a function makes a bad call, then it is also bad
            if intersection.into_iter().collect::<Vec<&String>>().len() != 0 {
                known_bad_calls.insert(call.to_string());
            }
        }
        if last_bad_call_count == known_bad_calls.len() {
            break;
        } else {
            last_bad_call_count = known_bad_calls.len();
        }
    }

    // Now check how many ambiguous calls we can add back
    for (call, set) in ambiguous_fastcalls.clone().iter() {
        let intersection = set.intersection(&known_bad_calls);
        if intersection.into_iter().collect::<Vec<&String>>().len() == 0 {
            called_funcs.insert(call.to_string());
        }
    }

    called_funcs
 }
