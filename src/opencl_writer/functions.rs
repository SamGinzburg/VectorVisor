use crate::opencl_writer;
use std::collections::HashMap;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;

/*
 * Notes on Irreducible Control Flow (ICF):
 * 
 * WASM includes a set of branching instructions that behave as GOTOs essentially (br, br_if).
 * These instructions jump directly to a specific label in the program and then continue execution.
 * 
 * Using GOTOs is the easiest way to implement this - in fact the official wasm2c compiler uses GOTOs to do
 * exactly this! If we were targeting C, this would be fine, but we are targeting OpenCL C which does not allow
 * for ICF at all!
 * 
 * We *have* to use gotos to implement our continuations, but we cannot use gotos to "call" the functions
 * 
 * In order to bypass this restriction we always return control to the main "wasm_entry" control function
 * This function is responsible for starting/resuming all function calls
 * 
 */

pub fn get_return_size(writer: &opencl_writer::OpenCLCWriter, ty: &wast::TypeUse<wast::FunctionType>) -> u32 {
    match ty.clone().inline {
        Some(r) => {
            if r.results.len() > 0 {
                writer.get_size_valtype(&r.results[0])
            } else {
                0
            }
        },
        _ => 0,
    }
}


pub fn emit_fn_call(writer: &opencl_writer::OpenCLCWriter, idx: wast::Index, call_ret_map: &mut HashMap<&str, u32>, call_ret_idx: &mut u32, function_id_map: &HashMap<&str, u32>, debug: bool) -> String {
    let id = match idx {
        wast::Index::Id(id) => id.name(),
        _ => panic!("Unable to get Id for function call!"),
    };

    // if the func has calling parameters, set those up
    // on the newly formed stack as well
    let func_type_signature = &writer.func_map.get(id).unwrap().ty;

    /*
     * We have to recompute the parameter offset, because this is the offset for the function
     * we are about to call, not the current function
     */
    let mut parameter_offset: u32 = 0;
    match func_type_signature.inline {
        // if we can find the type signature
        Some(_) => {
            for parameter in func_type_signature.clone().inline.unwrap().params.to_vec() {
                match parameter {
                    (_, _, t) => {
                        parameter_offset += writer.get_size_valtype(&t);
                    },
                    _ => panic!("Unhandled parameter type")
                }
            }
        },
        // if we cannot find the type signature, no-op
        None => (),
    }

    // for each function call, map the call to an index
    // we use this index later on to return back to the instruction after the call
    
    let ret_label: &'static str = Box::leak(format!("ret_from_{}_{}", id, call_ret_idx).into_boxed_str());
    call_ret_map.insert(ret_label, *call_ret_idx);

    // get the return type of the function
    let return_size = get_return_size(writer, &func_type_signature.clone());

    let result = if parameter_offset > 0 {
        format!("\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n{}\n\t{}\n",
        // move the stack pointer back by the offset required by calling parameters
        // the sp should point at the start of the locals for the function
        // increment stack frame pointer
        "*sfp += 1;",
        // save the current stack pointer for unwinding later
        //"stack_frames[*sfp] = *sp;",
        format!("{};", emit_write_u32("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "*sp", "warp_idx")),
        // save the callee return stub number
        format!("{}", &format!("{};",
                      emit_write_u64("(ulong)(call_stack+*sfp)",
                                     "(ulong)(call_stack)",
                                     &format!("{}", *call_ret_idx), "warp_idx"))),
        // push the entry point of the current function so we can return to it after the call
        format!("{};", emit_write_u32("(ulong)(call_return_stack+*sfp)",
                                      "(ulong)(call_return_stack)",
                                      "(uint)*entry_point",
                                      "warp_idx")),
        // set the entry point for the control function
        format!("*entry_point = {};", function_id_map.get(id).unwrap()),
        // set the is_calling parameter to true, to indicate that we are calling a function
        format!("{}", "*is_calling = 1;"),
        // return to the control function
        "return;",
        format!("call_return_stub_{}:", *call_ret_idx),
        format!("*sp += {};", return_size),
        format!("*sp -= {};", parameter_offset))
    } else {
        format!("\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n{}\n",
                // increment stack frame pointer
                "*sfp += 1;",
                // save the current stack pointer for unwinding later
                format!("{};", emit_write_u32("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "*sp", "warp_idx")),
                // save the callee return stub number
                format!("{}", &format!("{};",
                      emit_write_u64("(ulong)(call_stack+*sfp)",
                                     "(ulong)(call_stack)",
                                     &format!("{}", *call_ret_idx), "warp_idx"))),
                // push the entry point of the current function so we can return to it after the call
                format!("{};", emit_write_u32("(ulong)(call_return_stack+*sfp)",
                                              "(ulong)(call_return_stack)",
                                              "(uint)*entry_point",
                                              "warp_idx")),
                // set the entry point of the function we want to call...
                format!("*entry_point = {};", function_id_map.get(id).unwrap()),
                // set the is_calling parameter to true, to indicate that we are calling a function
                format!("{}", "*is_calling = 1;"),
                // return to the control function
                "return;",
                format!("call_return_stub_{}:", *call_ret_idx),
                format!("*sp -= {};", parameter_offset))
    };
    *call_ret_idx += 1;

    result
}

// TODO: this needs to take the function type into account
pub fn function_unwind(writer: &opencl_writer::OpenCLCWriter, fn_name: &str, func_ret_info: &Option<wast::FunctionType>, debug: bool) -> String {
    let mut final_str = String::from("");

    let results: Vec<wast::ValType> = match func_ret_info {
        Some(s) => (*s.results).to_vec(),
        None => vec![]
    };
    
    final_str += &format!("\t{}\n", "/* function unwind */");
    // strip illegal chars from func name
    final_str += &format!("{}_return:\n", format!("{}{}", "$_", fn_name.replace(".", "")));
    // for each value returned by the function, return it on the stack
    // keep track of the change to stack ptr from previous returns
    let mut sp_counter = 0;

    /*
     * We have to recompute the parameter offset, because this is the offset for the function
     * we are about to call, not the current function
     */
    let mut parameter_offset: u32 = 0;
    let func_type_signature = &writer.func_map.get(fn_name).unwrap().ty;
    match func_type_signature.inline {
        // if we can find the type signature
        Some(_) => {
            for parameter in func_type_signature.clone().inline.unwrap().params.to_vec() {
                match parameter {
                    (Some(_), _, t) => {
                        parameter_offset += writer.get_size_valtype(&t);
                    },
                    _ => panic!("Unhandled parameter type")
                }
            }
        },
        // if we cannot find the type signature, no-op
        None => (),
    }
    
    let mut offset;
    for value in results {
        match value {
            wast::ValType::I32 => {
                // compute the offset to read from the bottom of the stack
                if sp_counter > 0 {
                    offset = format!("write_u32((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                (ulong)stack_u32,
                                                read_u32((ulong)(stack_u32+*sp-{}-1), (ulong)stack_u32, warp_idx),
                                                warp_idx);", parameter_offset, sp_counter);
                } else {
                    offset = format!("write_u32((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u32((ulong)(stack_u32+*sp-1), (ulong)stack_u32, warp_idx),
                                                warp_idx);", parameter_offset);
                }
                final_str += &format!("\t{}\n", offset);
                sp_counter += 1;
            },
            wast::ValType::I64 => {
                // compute the offset to read from the bottom of the stack
                if sp_counter > 0 {
                    offset = format!("write_u64((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u64((ulong)(stack_u32+*sp-{}-2), (ulong)stack_u32, warp_idx),
                                                warp_idx);", parameter_offset, sp_counter);
                } else {
                    offset = format!("write_u64((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u64((ulong)(stack_u32+*sp-2), (ulong)stack_u32, warp_idx),
                                                warp_idx);", parameter_offset);
                }
                final_str += &format!("\t{}\n", offset);
                sp_counter += 2;
            },
            wast::ValType::F32 => {
                // compute the offset to read from the bottom of the stack
                if sp_counter > 0 {
                    offset = format!("write_u32((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u32((ulong)(stack_u32+*sp-{}-1), (ulong)stack_u32, warp_idx),
                                                warp_idx);", parameter_offset, sp_counter);
                } else {
                    offset = format!("write_u32((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u32((ulong)(stack_u32+*sp-1), (ulong)stack_u32, warp_idx),
                                                warp_idx);", parameter_offset);
                }
                final_str += &format!("\t{}\n", offset);
                sp_counter += 1;
            },
            wast::ValType::F64 => {
                // compute the offset to read from the bottom of the stack
                if sp_counter > 0 {
                    offset = format!("write_u64((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u64((ulong)(stack_u32+*sp-{}-2), (ulong)stack_u32, warp_idx),
                                                warp_idx);", parameter_offset, sp_counter);
                } else {
                    offset = format!("write_u64((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u64((ulong)(stack_u32+*sp-2), (ulong)stack_u32, warp_idx),
                                                warp_idx);", parameter_offset);
                }
                final_str += &format!("\t{}\n", offset);
                sp_counter += 2;
            },
            _ => panic!("Unimplemented function return type!!!"),
        }
    }

    // load the entry point to return to in the control function
    final_str += &format!("\t*entry_point = {};\n", emit_read_u32("(ulong)(call_return_stack+*sfp)", "(ulong)(call_return_stack)", "warp_idx"));

    final_str += &format!("\t{}\n",
                            // reset the stack pointer to point at the end of the previous frame
                            &format!("*sp = {};", emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx")));

    // set the is_calling parameter to 0, to indicate that we are unwinding the call stack
    final_str += &format!("\t{}\n", "*is_calling = 0;");

    final_str += &format!("\t{}\n", "return;");

    final_str
}

/*
 * NOTE: In the current version of WebAssembly, at most one table may be defined or
 *  imported in a single module, and all constructs implicitly reference this table 0.
 *  This restriction may be lifted in future versions.
 * 
 *  This means we can afford to hardcode table "$T0" for now. In the future we may have to 
 *  generate multiple tables or deal with the tables as a global structure instead of evaluating
 *  them at compile time.
 *  
 *  We can have multiple elements initializing the table in sequence, and they can overwrite
 *  each other, so we must process them sequentially.
 * 
 */
pub fn emit_call_indirect(writer: &opencl_writer::OpenCLCWriter, parameter_offset: i32, table: &HashMap<u32, &wast::Index>, call_ret_map: &mut HashMap<&str, u32>, call_ret_idx: &mut u32, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    let mut result = String::from("");
    // set up a switch case statement, we read the last value on the stack and determine what function we are going to call
    // this adds code bloat, but it reduces the complexity of the compiler.
    // It is worth revisiting this later, but not urgent. 

    result += &format!("\t{}\n",
                       // the most recent item on the stack is 
                       &format!("switch({}) {{", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx")));
    // generate all of the cases in the table, all uninitialized values will trap to the default case
    for (key, value) in table {
        result += &format!("\t\t{}\n", format!("case {}:", key));
        // now that we have found the appropriate call, we can pop the value off of the stack
        result += &format!("\t\t\t{}\n", format!("*sp -= 1;"));
        // emit the function call here!
        result += &format!("{}", emit_fn_call(writer, **value, call_ret_map, call_ret_idx, &function_id_map, debug));
        result += &format!("\t\t\t{}\n", format!("break;"));
    }

    // emit a default case, to handle lookups to invalid indicies!
    result += &format!("\t\t{}\n", "default:");
    result += &format!("\t\t\t{}\n", "return;");
    result += &format!("\t}}\n");

    result
}