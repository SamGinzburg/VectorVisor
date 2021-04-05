use crate::opencl_writer;
use std::collections::HashMap;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

use wast::Index::*;
use wast::TypeDef::*;

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
        _ => {
            0
        },
    }
}


pub fn emit_fn_call(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, fn_name: String, idx: wast::Index, call_ret_map: &mut HashMap<&str, u32>, call_ret_idx: &mut u32, function_id_map: &HashMap<&str, u32>, stack_sizes: &mut Vec<u32>, is_indirect: bool, debug: bool) -> String {
    let mut ret_str = String::from("");
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
    let mut stack_params = vec![];
    let mut stack_params_types = vec![];

    let mut return_type = None;

    let mut parameter_offset: u32 = 0;
    match func_type_signature.inline {
        // if we can find the type signature
        Some(_) => {
            for parameter in func_type_signature.clone().inline.unwrap().params.to_vec() {
                match parameter {
                    (_, _, t) => {
                        if !is_indirect {
                            stack_sizes.pop().unwrap();
                            stack_params_types.insert(0, StackCtx::convert_wast_types(&t));
                            stack_params.insert(0, stack_ctx.vstack_pop(StackCtx::convert_wast_types(&t)));
                        }
                        parameter_offset += writer.get_size_valtype(&t);
                    },
                }
            }
            if func_type_signature.clone().inline.unwrap().results.len() > 0 {
                return_type = Some(func_type_signature.clone().inline.unwrap().results[0]);
            } else {
                return_type = None;
            }
        },
        // if we cannot find the type signature, we need to look it up to check for the param offset
        None => {
            let fn_type_id = match func_type_signature.index {
                Some(wast::Index::Id(id)) => id.name().to_string(),
                Some(wast::Index::Num(n, _)) => format!("t{}", n),
                None => format!(""),
            };

            let function_type = writer.types.get(&fn_type_id);
            match function_type {
                Some(wast::TypeDef::Func(ft)) => {
                    for parameter in ft.params.to_vec() {
                        match parameter {
                            (_, _, t) => {
                                if !is_indirect {
                                    stack_sizes.pop().unwrap();
                                    stack_params_types.insert(0, StackCtx::convert_wast_types(&t));
                                    stack_params.insert(0, stack_ctx.vstack_pop(StackCtx::convert_wast_types(&t)));
                                }
                                parameter_offset += writer.get_size_valtype(&t);
                            },
                        }
                    }
                    if ft.results.len() > 0 {
                        return_type = Some(ft.results[0]);
                    } else {
                        return_type = None;
                    }
                },
                None => (),
                _ => panic!("Non-function type referenced from function")
            };
        },
    }

    // Now, we need to save the intermediate context
    if !is_indirect {
        ret_str += &stack_ctx.save_context(false);
    }

    // We need to manually write the parameters to the stack
    // For indirect function calls we need to do this before the switch case to over allocating registers
    if !is_indirect {
        for (param, ty) in stack_params.iter().zip(stack_params_types.iter()) {
            match ty {
                StackType::i32 => {
                    ret_str += &format!("\t{};\n\t*sp += 1;\n",
                                            emit_write_u32("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", &param, "warp_idx"));
                },
                StackType::i64 => {
                    ret_str += &format!("\t{};\n\t*sp += 2;\n",
                                            emit_write_u64("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", &param, "warp_idx"));
                },
                StackType::f32 => {
                    ret_str += &format!("\t{{\n");
                    ret_str += &format!("\t\tuint temp = 0;\n");
                    ret_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(uint));\n", param);
                    ret_str += &format!("\t\t{};\n\t*sp += 1;\n",
                                        emit_write_u32("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", "temp", "warp_idx"));
                    ret_str += &format!("\t}}\n");
                },
                StackType::f64 => {
                    ret_str += &format!("\t{{\n");
                    ret_str += &format!("\t\tulong temp = 0;\n");
                    ret_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n", param);
                    ret_str += &format!("\t\t{};\n\t*sp += 2;\n",
                                        emit_write_u64("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", "temp", "warp_idx"));
                    ret_str += &format!("\t}}\n");
                },
            }
        }
    }

    // for each function call, map the call to an index
    // we use this index later on to return back to the instruction after the call
    
    let ret_label: &'static str = Box::leak(format!("ret_from_{}_{}", id, call_ret_idx).into_boxed_str());
    call_ret_map.insert(ret_label, *call_ret_idx);
    
    // get the return type of the function
    let return_size = get_return_size(writer, &func_type_signature.clone());

    if !is_indirect {
        stack_sizes.push(return_size);
    }

    let result = if return_size > 0 {
        format!("\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n",
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
        format!("{}_call_return_stub_{}:", format!("{}{}", "__", fn_name.replace(".", "")), *call_ret_idx),
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
                format!("{}_call_return_stub_{}:", format!("{}{}", "__", fn_name.replace(".", "")), *call_ret_idx),
                format!("*sp -= {};", parameter_offset))
    };
    *call_ret_idx += 1;

    ret_str += &result;

    // Restore the intermediate context, generate the string here
    // We do this here because if we alloc for the return value, we don't want to delete more space

    let restore_context = if !is_indirect {
        stack_ctx.restore_context(false, false)
    } else {
        String::from("")
    };

    // after returning, the top of the stack is our return var (if we have one)
    if return_size > 0 && !is_indirect {
        let result_register = stack_ctx.vstack_alloc(StackCtx::convert_wast_types(&return_type.unwrap()));
        match StackCtx::convert_wast_types(&return_type.unwrap()) {
            StackType::i32 => {
                ret_str += &format!("\t{} = {};\n\t{};\n", result_register, 
                                    emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"),
                                    "*sp -= 1");
            },
            StackType::i64 => {
                ret_str += &format!("\t{} = {};\n\t{};\n", result_register, 
                                    emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                    "*sp -= 2;");
            },
            StackType::f32 => {
                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tuint temp = {};\n", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
                ret_str += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(uint));\n", result_register);
                ret_str += &format!("\t\t*sp -= 1;\n");
                ret_str += &format!("\t}}\n");
            },
            StackType::f64 => {
                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tulong temp = {};\n", emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
                ret_str += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(ulong));\n", result_register);
                ret_str += &format!("\t\t*sp -= 2;\n");
                ret_str += &format!("\t}}\n");
            }
        }
    }

    // restore the intermediate context
    if !is_indirect {
        ret_str += &restore_context;
    }

    ret_str
}

// TODO: this needs to take the function type into account
pub fn function_unwind(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, fn_name: &str, func_ret_info: &Option<wast::FunctionType>, is_start_fn: bool, debug: bool) -> String {
    let mut final_str = String::from("");
    let results: Vec<wast::ValType> = match func_ret_info {
        Some(s) => (*s.results).to_vec(),
        None => {
            vec![]
        }
    };
    
    final_str += &format!("\t{}\n", "/* function unwind */");
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
                    (_, _, t) => {
                        parameter_offset += writer.get_size_valtype(&t);
                    },
                }
            }
        },
        // if we cannot find the type signature, no-op
        // this seems to only come up in cases where there are no parameters
        None => {
            ()
        },
    }

    let mut offset;
    for value in results {
        match value {
            wast::ValType::I32 => {
                // compute the offset to read from the bottom of the stack
                let reg = if !stack_ctx.vstack_is_empty(StackType::i32) {
                    stack_ctx.vstack_pop(StackType::i32)
                } else {
                    String::from("(uint)(-1)")
                };

                if sp_counter > 0 {
                    offset = format!("write_u32((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                (ulong)stack_u32,
                                                {},
                                                warp_idx);", parameter_offset, reg);
                } else {
                    offset = format!("write_u32((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                {},
                                                warp_idx);", parameter_offset, reg);
                }
                final_str += &format!("\t{}\n", offset);
                sp_counter += 1;
            },
            wast::ValType::I64 => {
                // compute the offset to read from the bottom of the stack
                let reg = if !stack_ctx.vstack_is_empty(StackType::i64) {
                    stack_ctx.vstack_pop(StackType::i64)
                } else {
                    String::from("(uint)(-1)")
                };

                if sp_counter > 0 {
                    offset = format!("write_u64((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                {},
                                                warp_idx);", parameter_offset, reg);
                } else {
                    offset = format!("write_u64((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                {},
                                                warp_idx);", parameter_offset, reg);
                }
                final_str += &format!("\t{}\n", offset);
                sp_counter += 2;
            },
            wast::ValType::F32 => {
                // compute the offset to read from the bottom of the stack
                let reg = if !stack_ctx.vstack_is_empty(StackType::f32) {
                    stack_ctx.vstack_pop(StackType::f32)
                } else {
                    String::from("(uint)(-1)")
                };

                if sp_counter > 0 {
                    offset = format!("write_u32((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                {},
                                                warp_idx);", parameter_offset, "temp");
                } else {
                    offset = format!("write_u32((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                {},
                                                warp_idx);", parameter_offset, "temp");
                }
                final_str += &format!("\t{{\n");
                final_str += &format!("\t\tuint temp = 0;\n");
                final_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(float));\n", reg);
                final_str += &format!("\t\t{}\n", offset);
                final_str += &format!("\t}}\n");
                sp_counter += 1;
            },
            wast::ValType::F64 => {
                // compute the offset to read from the bottom of the stack
                // compute the offset to read from the bottom of the stack
                let reg = if !stack_ctx.vstack_is_empty(StackType::f64) {
                    stack_ctx.vstack_pop(StackType::f64)
                } else {
                    String::from("(uint)(-1)")
                };

                if sp_counter > 0 {
                    offset = format!("write_u64((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                {},
                                                warp_idx);", parameter_offset, "temp");
                } else {
                    offset = format!("write_u64((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx)),
                                                (ulong)stack_u32,
                                                {},
                                                warp_idx);", parameter_offset, "temp");
                }
                final_str += &format!("\t{{\n");
                final_str += &format!("\t\tulong temp = 0;\n");
                final_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n", &reg);
                final_str += &format!("\t\t{}\n", offset);
                final_str += &format!("\t}}\n");

                sp_counter += 2;
            },
            _ => panic!("Unimplemented function return type!!!"),
        }
    }

    // If we aren't the start function we need to properly unwind
    if !is_start_fn {
        // load the entry point to return to in the control function
        final_str += &format!("\t*entry_point = {};\n", emit_read_u32("(ulong)(call_return_stack+*sfp)", "(ulong)(call_return_stack)", "warp_idx"));

        final_str += &format!("\t{}\n",
                                // reset the stack pointer to point at the end of the previous frame
                                &format!("*sp = {};", emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx")));

        // set the is_calling parameter to 0, to indicate that we are unwinding the call stack
        final_str += &format!("\t{}\n", "*is_calling = 0;");

        final_str += &format!("\t{}\n", "return;");
    }

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
pub fn emit_call_indirect(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, call_indirect: &wast::CallIndirect, fn_name: String, _parameter_offset: i32, table: &HashMap<u32, &wast::Index>, call_ret_map: &mut HashMap<&str, u32>, call_ret_idx: &mut u32, function_id_map: HashMap<&str, u32>, stack_sizes: &mut Vec<u32>, debug: bool) -> String {
    let mut result = String::from("");
    // set up a switch case statement, we read the last value on the stack and determine what function we are going to call
    // this adds code bloat, but it reduces the complexity of the compiler.
    // It is worth revisiting this later, but not urgent. 

    // the index into the function table
    if stack_sizes.pop().unwrap() != 1 {
        panic!("Function table index for indirect call must be of type i32");
    }

    let index_register = stack_ctx.vstack_pop(StackType::i32);

    let mut stack_params = vec![];
    let mut stack_params_types = vec![];

    // Get the type information for the indirect call! We need this to handle parametric
    // statements after a call such as select
    let mut result_types = vec![];
    match (call_indirect.ty.index.as_ref(), call_indirect.ty.inline.as_ref()) {
        (Some(index), _) => {
            // if we have an index, we need to look it up in the global structure
            let type_index = match index {
                Num(n, _) => format!("t{}", n),
                Id(i) => i.name().to_string(),
            };

            let func_type = match writer.types.get(&type_index).unwrap() {
                Func(ft) => ft,
                _ => panic!("Indirect call cannot have a type of something other than a func"),
            };

            // First, pop off the parameters
            for (_, _, param_type) in func_type.params.iter() {
                stack_sizes.pop().unwrap();
                stack_params_types.insert(0, StackCtx::convert_wast_types(&param_type));
                stack_params.insert(0, stack_ctx.vstack_pop(StackCtx::convert_wast_types(&param_type)));
            }

            // Next, push the result(s) back
            for return_type in func_type.results.iter() {
                stack_sizes.push(writer.get_size_valtype(return_type));
                result_types.push(return_type);
            }
        },
        (_, Some(inline)) => panic!("Inline types for call_indirect not implemented yet"),
        _ => (),
    };

    // Save the context before entering the switch case
    result += &stack_ctx.save_context(false);

    let restore_ctx = stack_ctx.restore_context(false, false);

    // Push the parameters to the stack
    for (param, ty) in stack_params.iter().zip(stack_params_types.iter()) {
        match ty {
            StackType::i32 => {
                result += &format!("\t\t{};\n\t\t*sp += 1;\n",
                                        emit_write_u32("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", &param, "warp_idx"));
            },
            StackType::i64 => {
                result += &format!("\t\t{};\n\t\t*sp += 2;\n",
                                        emit_write_u64("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", &param, "warp_idx"));
            },
            StackType::f32 => {
                result += &format!("\t{{\n");
                result += &format!("\t\tuint temp = 0;\n");
                result += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(uint));\n", param);
                result += &format!("\t\t{};\n\t\t*sp += 1;\n",
                                    emit_write_u32("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", "temp", "warp_idx"));
                result += &format!("\t}}\n");
            },
            StackType::f64 => {
                result += &format!("\t{{\n");
                result += &format!("\t\tulong temp = 0;\n");
                result += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n", param);
                result += &format!("\t\t{};\n\t\t*sp += 2;\n",
                                    emit_write_u64("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", "temp", "warp_idx"));
                result += &format!("\t}}\n");
            }
        }
    }

    // Allocate a register for return values
    let result_register = if result_types.len() > 0 {
        stack_ctx.vstack_alloc(StackCtx::convert_wast_types(&result_types[0]))
    } else {
        String::from("")
    };

    result += &format!("\t{}\n",
                       &format!("switch({}) {{", index_register));
    // generate all of the cases in the table, all uninitialized values will trap to the default case
    for (key, value) in table {
        result += &format!("\t\t{}\n", format!("case {}:", key));
        // emit the function call here!
        result += &format!("{}", emit_fn_call(writer, stack_ctx, fn_name.clone(), **value, call_ret_map, call_ret_idx, &function_id_map, stack_sizes, true, debug));
        result += &format!("\t\t\t{}\n", format!("break;"));
    }

    // emit a default case, to handle lookups to invalid indicies!
    result += &format!("\t\t{}\n", "default:");
    result += &format!("\t\t\t{}\n", "return;");
    result += &format!("\t}}\n");

    // Read the result value into a register
    if result_types.len() > 0 {
        match StackCtx::convert_wast_types(&result_types[0]) {
            StackType::i32 => {
                result += &format!("\t{} = {};\n\t{};\n", result_register, 
                                    emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"),
                                    "*sp -= 1");
            },
            StackType::i64 => {
                result += &format!("\t{} = {};\n\t{};\n", result_register, 
                                    emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                    "*sp -= 2;");
            },
            StackType::f32 => {
                result += &format!("\t{{\n");
                result += &format!("\t\tuint temp = {};\n", emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx"));
                result += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(uint));\n", result_register);
                result += &format!("\t\t*sp -= 1;\n");
                result += &format!("\t}}\n");
            },
            StackType::f64 => {
                result += &format!("\t{{\n");
                result += &format!("\t\tulong temp = {};\n", emit_read_u64("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
                result += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(ulong));\n", result_register);
                result += &format!("\t\t*sp -= 2;\n");
                result += &format!("\t}}\n");
            }
        }
    }

    // Restore the context
    result += &restore_ctx;

    result
}
