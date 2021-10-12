use crate::opencl_writer;
use crate::opencl_writer::WASI_SNAPSHOT_PREVIEW1;
use crate::opencl_writer::format_fn_name;
use std::collections::{HashSet, HashMap};

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
 * - Fastcalls may not exceed register usage threshold
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


 fn is_fastcall(writer: &opencl_writer::OpenCLCWriter, func_name: String, func: &wast::Func, fastcall_set: &mut HashSet<String>, indirect_calls: &mut HashSet<String>, start_func: String) -> FastcallPassStatus {
    match (&func.kind, &func.id, &func.ty) {
        (wast::FuncKind::Import(_), _, _) => {
            panic!("InlineImport functions not yet implemented (fastcall pass)");
        },
        (wast::FuncKind::Inline{locals, expression}, _, typeuse) => {

            let mut local_type_info: HashMap<String, wast::ValType> = HashMap::new();
            let mut param_idx = 0;
			match typeuse.clone().inline {
				Some(params) => {
					for parameter in params.params.to_vec() {
						match parameter {
							(Some(id), _, t) => {
								local_type_info.insert(id.name().to_string(), t.clone());
							},
							// if there is no id, we have to name the parameter ourselves!
							(None, _, t) => {
								local_type_info.insert(format!("p{}", param_idx), t.clone());
							},
							_ => panic!("Unhandled parameter type")
						}
						param_idx += 1;
					}
	
				},
				None => {
					()
				},
			}
			for local in locals {
				let local_id = match local.id {
					Some(name) => name.name().to_string(),
					None => format!("l{}", param_idx),
				};
				local_type_info.insert(local_id.clone(), local.ty.clone());
                param_idx += 1;
			}

			let mut size = 0;
			for (_, t) in local_type_info {
				size += match t {
					wast::ValType::I32 => 4,
					wast::ValType::I64 => 8,
					wast::ValType::F32 => 4,
					wast::ValType::F64 => 8,
					_ => panic!("Unimplemented type in fastcall pass"),
				};
			}

            if size > 128 {
                return FastcallPassStatus::fastcall_false(String::from("local/param size too large to convert to fastcall"))
            }

            // Is this function the start function?
            if func_name == start_func {
                return FastcallPassStatus::fastcall_false(String::from("is start fn"))
            }

            // Is this function the target of an indirect call?
            if indirect_calls.contains(&func_name) {
                // return FastcallPassStatus::fastcall_false(String::from("indirect call target"))
            }

            // Is this function in the indirect call table?
            let mut ambiguous_dep_list = HashSet::new();
            for instruction in expression.instrs.iter() {
                match instruction {
                    wast::Instruction::Call(idx) => {
                        let id = match idx {
                            wast::Index::Id(id) => format_fn_name(id.name()),
                            wast::Index::Num(val, _) => format!("func_{}", val),
                            _ => panic!("Unable to get Id for function call: {:?}", idx),
                        };

                        // Fastcalls may not perform recursion
                        if id.to_string() == func_name {
                            return FastcallPassStatus::fastcall_false(String::from("performs recursion"))
                        }

                        // Is this a hypercall?
                        match writer.imports_map.get(&id) {
                            Some((wasi_api, Some(wasi_fn_name), _)) => {    
                                match (wasi_api, WASI_SNAPSHOT_PREVIEW1.get(wasi_fn_name)) {
                                    (_, Some(true)) => {
                                        match wasi_fn_name {
                                            &"proc_exit" => {
                                                // proc_exit is special cased, since we don't
                                                // actually need to return
                                            },
                                            _ => {
                                                // if we found a WASI hypercall...
                                                return FastcallPassStatus::fastcall_false(String::from("performs hypercall"))
                                            },
                                        }
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
 pub fn compute_fastcall_set(writer: &opencl_writer::OpenCLCWriter, func_map: &HashMap<String, wast::Func>, indirect_calls: &mut HashSet<String>, start_func: String) -> HashSet<String> {
    let mut called_funcs = HashSet::new();
    let mut known_bad_calls = HashSet::new();

    let mut fastcall_count = 0;
    let mut ambiguous_fastcalls;

    loop {
        //println!("Fastcall analysis pass, found: {:?} functions to optimize", fastcall_count);
        ambiguous_fastcalls = vec![];
        for (f_name, func) in func_map.iter() {
            let is_fastcall = is_fastcall(writer, f_name.to_string(), func, &mut called_funcs, indirect_calls, start_func.clone());
            match is_fastcall {
                FastcallPassStatus::fastcall_true => {
                    called_funcs.insert(f_name.to_string());
                },
                FastcallPassStatus::fastcall_ambiguous(fastcall_ambiguous) => {
                    ambiguous_fastcalls.push((f_name.to_string(), fastcall_ambiguous));
                },
                FastcallPassStatus::fastcall_false(_) => {
                    known_bad_calls.insert(f_name.to_string());
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
