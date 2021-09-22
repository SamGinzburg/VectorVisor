use crate::opencl_writer;
use std::collections::{HashMap, HashSet};
use crate::opencl_writer::mem_interleave::emit_read_u32_aligned;
use crate::opencl_writer::mem_interleave::emit_write_u32_aligned;
use crate::opencl_writer::mem_interleave::emit_read_u64_aligned;
use crate::opencl_writer::mem_interleave::emit_write_u64_aligned;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;
use crate::opencl_writer::trap::{emit_trap, TrapCode};

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
            let index = ty.clone().index.unwrap();
            let ty_name = match index {
                Num(n, _) => format!("t{}", n),
                Id(i) => i.name().to_string(),
            };

            let func_type = match writer.types.get(&ty_name).unwrap() {
                Func(f) => f,
                _ => panic!("non-function type found for function in get_return_size"),
            };

            let ret_val = if func_type.results.len() > 0 {
                writer.get_size_valtype(&func_type.results[0])
            } else {
                0
            };

            ret_val
        },
    }
}

pub fn get_func_params(writer: &opencl_writer::OpenCLCWriter, ty: &wast::TypeUse<wast::FunctionType>) -> Vec<StackType> {
    let mut return_params = vec![];

    match (ty.index.as_ref(), ty.inline.as_ref()) {
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

            for (_, _, param_type) in func_type.params.iter() {
                return_params.push(StackCtx::convert_wast_types(&param_type));
            }
        },
        (_, Some(inline)) => {
            for (_, _, param_type) in inline.params.iter() {
                return_params.push(StackCtx::convert_wast_types(&param_type));
            }
        },
        //_ => panic!("Neither inline or index type found for func in get_func_params!"),
        _ => { () },
    }

    return_params
}

pub fn get_func_result(writer: &opencl_writer::OpenCLCWriter, ty: &wast::TypeUse<wast::FunctionType>) -> Option<StackType> {
    let mut ret_val = None;

    match (ty.index.as_ref(), ty.inline.as_ref()) {
        (Some(index), _) => {
            // if we have an index, we need to look it up in the global structure
            let type_index = match index {
                Num(n, _) => format!("t{}", n),
                Id(i) => i.name().to_string(),
            };

            let func_type = match writer.types.get(&type_index).unwrap() {
                Func(ft) => ft,
                _ => panic!("get_func_result cannot have a type of something other than a func"),
            };
            if func_type.results.len() > 0 {
                ret_val = Some(StackCtx::convert_wast_types(&func_type.results[0]));
            }
        },
        (_, Some(inline)) => {
            if inline.results.len() > 0 {
                ret_val = Some(StackCtx::convert_wast_types(&inline.results[0]));
            }
        },
        //_ => panic!("Neither inline or index type found for func in get_func_result!"),
        _ => { () },
    }

    ret_val
}


pub fn emit_fn_call(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, fn_name: String, idx: wast::Index, call_ret_map: &mut HashMap<&str, u32>, call_ret_idx: &mut u32, function_id_map: &HashMap<&str, u32>, is_indirect: bool, is_fastcall: bool, indirect_fastcall_param: String, indirect_fastcall_stack_params: Vec<String>, _debug: bool) -> String {
    let mut ret_str = String::from("");
    let id = &match idx {
        wast::Index::Id(id) => id.name().to_string(),
        wast::Index::Num(val, _) => format!("func_{}", val),
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
            for parameter in func_type_signature.inline.as_ref().unwrap().params.to_vec() {
                match parameter {
                    (_, _, t) => {
                        if !is_indirect {
                            let (param, param_type) = stack_ctx.vstack_pop_any();
                            stack_params.insert(0, param);
                            stack_params_types.insert(0, param_type);
                        }
                        parameter_offset += 2;
                    },
                }
            }
            if func_type_signature.inline.as_ref().unwrap().results.len() > 0 {
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
                                    let (param, param_type) = stack_ctx.vstack_pop_any();
                                    stack_params.insert(0, param);
                                    stack_params_types.insert(0, param_type);
                                }
                                parameter_offset += 2;
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
    if !is_indirect && !is_fastcall {
        ret_str += &stack_ctx.save_context(false, false);
    }

    // We need to manually write the parameters to the stack
    // For indirect function calls we need to do this before the switch case to over allocating registers
    if !is_indirect && !is_fastcall {
        for (param, ty) in stack_params.iter().zip(stack_params_types.iter()) {
            match ty {
                StackType::i32 => {
                    ret_str += &format!("\t{};\n\t*sp += 2;\n",
                                            emit_write_u32_aligned("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", &param, "warp_idx"));
                },
                StackType::i64 => {
                    ret_str += &format!("\t{};\n\t*sp += 2;\n",
                                            emit_write_u64_aligned("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", &param, "warp_idx"));
                },
                StackType::f32 => {
                    ret_str += &format!("\t{{\n");
                    ret_str += &format!("\t\tuint temp = 0;\n");
                    ret_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(uint));\n", param);
                    ret_str += &format!("\t\t{};\n\t*sp += 2;\n",
                                        emit_write_u32_aligned("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", "temp", "warp_idx"));
                    ret_str += &format!("\t}}\n");
                },
                StackType::f64 => {
                    ret_str += &format!("\t{{\n");
                    ret_str += &format!("\t\tulong temp = 0;\n");
                    ret_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n", param);
                    ret_str += &format!("\t\t{};\n\t*sp += 2;\n",
                                        emit_write_u64_aligned("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", "temp", "warp_idx"));
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
    let return_size = if get_return_size(writer, &func_type_signature.clone()) > 0 {
        2
    } else {
        0
    };

    let result = if return_size > 0 && !is_fastcall {
        format!("\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n",
        // move the stack pointer back by the offset required by calling parameters
        // the sp should point at the start of the locals for the function
        // increment stack frame pointer
        "*sfp += 1;",
        // save the current stack pointer for unwinding later
        //"stack_frames[*sfp] = *sp;",
        format!("{};", emit_write_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "*sp", "warp_idx")),
        // save the callee return stub number
        format!("{}", &format!("{};",
                      emit_write_u64_aligned("(ulong)(call_stack+*sfp)",
                                     "(ulong)(call_stack)",
                                     &format!("{}", *call_ret_idx), "warp_idx"))),
        // push the entry point of the current function so we can return to it after the call
        format!("{};", emit_write_u32_aligned("(ulong)(call_return_stack+*sfp)",
                                      "(ulong)(call_return_stack)",
                                      "(uint)*entry_point",
                                      "warp_idx")),
        // set the entry point for the control function
        format!("*entry_point = {};", function_id_map.get(id as &str).unwrap()),
        // set the is_calling parameter to true, to indicate that we are calling a function
        format!("{}", "*is_calling = 1;"),
        // return to the control function
        "return;",
        format!("{}_call_return_stub_{}:", format!("{}{}", "__", fn_name.replace(".", "")), *call_ret_idx),
        format!("*sp += {};", return_size),
        format!("*sp -= {};", parameter_offset))
    } else if return_size <= 0 && !is_fastcall {
        format!("\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n{}\n",
                // increment stack frame pointer
                "*sfp += 1;",
                // save the current stack pointer for unwinding later
                format!("{};", emit_write_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "*sp", "warp_idx")),
                // save the callee return stub number
                format!("{}", &format!("{};",
                      emit_write_u64_aligned("(ulong)(call_stack+*sfp)",
                                     "(ulong)(call_stack)",
                                     &format!("{}", *call_ret_idx), "warp_idx"))),
                // push the entry point of the current function so we can return to it after the call
                format!("{};", emit_write_u32_aligned("(ulong)(call_return_stack+*sfp)",
                                              "(ulong)(call_return_stack)",
                                              "(uint)*entry_point",
                                              "warp_idx")),
                // set the entry point of the function we want to call...
                format!("*entry_point = {};", function_id_map.get(id as &str).unwrap()),
                // set the is_calling parameter to true, to indicate that we are calling a function
                format!("{}", "*is_calling = 1;"),
                // return to the control function
                "return;",
                format!("{}_call_return_stub_{}:", format!("{}{}", "__", fn_name.replace(".", "")), *call_ret_idx),
                format!("*sp -= {};", parameter_offset))
    } else {
        // No-op for fastcalls
        format!("")
    };

    // only increment the call ret idx if we are not a fastcall
    if !is_fastcall {
        *call_ret_idx += 1;
    }

    ret_str += &result;

    // Restore the intermediate context, generate the string here
    // We do this here because if we alloc for the return value, we don't want to delete more space

    let restore_context = if !is_indirect && !is_fastcall {
        stack_ctx.restore_context(false, false)
    } else {
        String::from("")
    };

    // after returning, the top of the stack is our return var (if we have one)
    if return_size > 0 && !is_indirect && !is_fastcall {
        let result_register = stack_ctx.vstack_alloc(StackCtx::convert_wast_types(&return_type.unwrap()));
        match StackCtx::convert_wast_types(&return_type.unwrap()) {
            StackType::i32 => {
                ret_str += &format!("\t{} = {};\n\t{};\n", result_register, 
                                    emit_read_u32_aligned("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                    "*sp -= 2");
            },
            StackType::i64 => {
                ret_str += &format!("\t{} = {};\n\t{};\n", result_register, 
                                    emit_read_u64_aligned("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                    "*sp -= 2;");
            },
            StackType::f32 => {
                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tuint temp = {};\n", emit_read_u32_aligned("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
                ret_str += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(uint));\n", result_register);
                ret_str += &format!("\t\t*sp -= 2;\n");
                ret_str += &format!("\t}}\n");
            },
            StackType::f64 => {
                ret_str += &format!("\t{{\n");
                ret_str += &format!("\t\tulong temp = {};\n", emit_read_u64_aligned("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
                ret_str += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(ulong));\n", result_register);
                ret_str += &format!("\t\t*sp -= 2;\n");
                ret_str += &format!("\t}}\n");
            }
        }
    }

    // restore the intermediate context, increment the call ret idx
    if !is_indirect && !is_fastcall {
        ret_str += &restore_context;
    }

    // Insert the code for fastcalls here
    if is_fastcall {
        let calling_func_name = format!("{}{}", "__", id.replace(".", ""));
        let mut parameter_list = String::from("");

        if is_indirect {
            for param in indirect_fastcall_stack_params.iter() {
                parameter_list += &format!("{}, ", param);
            }
        } else {
            for (param, _ty) in stack_params.iter().zip(stack_params_types.iter()) {
                parameter_list += &format!("{}, ", param);
            }
        }

        if return_size > 0 && !is_indirect {
            let result_register = stack_ctx.vstack_alloc(StackCtx::convert_wast_types(&return_type.unwrap()));
            ret_str += &format!("\t{} = {}_fastcall({}heap_u32, current_mem_size, max_mem_size, globals_buffer, warp_idx, thread_idx, read_idx, scratch_space);\n", result_register, calling_func_name, parameter_list);
        } else if return_size > 0 {
            ret_str += &format!("\t{} = {}_fastcall({}heap_u32, current_mem_size, max_mem_size, globals_buffer, warp_idx, thread_idx, read_idx, scratch_space);\n", indirect_fastcall_param, calling_func_name, parameter_list);
        } else {
            ret_str += &format!("\t{}_fastcall({}heap_u32, current_mem_size, max_mem_size, globals_buffer, warp_idx, thread_idx, read_idx, scratch_space);\n", calling_func_name, parameter_list);
        }
    }

    ret_str
}

// TODO: this needs to take the function type into account
pub fn function_unwind(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, fn_name: &str, func_ret_info: &Option<wast::FunctionType>, is_start_fn: bool, is_fastcall: bool, _debug: bool) -> String {
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
                        parameter_offset += 2;
                    },
                }
            }
        },
        // if we cannot find the type signature, no-op
        // this seems to only come up in cases where there are no parameters
        None => {
            //dbg!("Unable to find type signature for parameters in func_unwind: fn_name: {:?}", &fn_name);
            ()
        },
    }

    if is_fastcall {
        // Get the return type
        if results.len() > 0 {
            match results[0] {
                wast::ValType::I32 => {
                    let reg = if !stack_ctx.vstack_is_empty(StackType::i32) {
                        stack_ctx.vstack_pop(StackType::i32)
                    } else {
                        String::from("(uint)(-1)")
                    };
                    final_str += &format!("\treturn {};\n", reg);
                },
                wast::ValType::I64 => {
                    let reg = if !stack_ctx.vstack_is_empty(StackType::i64) {
                        stack_ctx.vstack_pop(StackType::i64)
                    } else {
                        String::from("(ulong)(-1)")
                    };
                    final_str += &format!("\treturn {};\n", reg);
                },
                wast::ValType::F32 => {
                    let reg = if !stack_ctx.vstack_is_empty(StackType::f32) {
                        stack_ctx.vstack_pop(StackType::f32)
                    } else {
                        String::from("(float)(-1)")
                    };
                    final_str += &format!("\treturn {};\n", reg);
                },
                wast::ValType::F64 => {
                    let reg = if !stack_ctx.vstack_is_empty(StackType::f64) {
                        stack_ctx.vstack_pop(StackType::f64)
                    } else {
                        String::from("(double)(-1)")
                    };
                    final_str += &format!("\treturn {};\n", reg);
                },
                _ => panic!("Unimplemented function return type in fastcall unwind!!!"),
            }
        } else {
            final_str += &format!("\treturn;\n");
        }
    } else {
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
                        let read_sfp = emit_read_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx");
                        let write_u32 = emit_write_u32_aligned(&format!("(ulong)(stack_u32-{}+{})", parameter_offset, read_sfp), "(ulong)stack_u32", &format!("{}", reg), "warp_idx");
                        /*
                        offset = format!("write_u32((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), warp_idx, read_idx, thread_idx, scratch_space)),
                                                    (ulong)stack_u32,
                                                    {},
                                                    warp_idx, read_idx, scratch_space);", parameter_offset, reg);
                        */
                        offset = write_u32;
                    } else {
                        let read_sfp = emit_read_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx");
                        let write_u32 = emit_write_u32_aligned(&format!("(ulong)(stack_u32-{}+{})", parameter_offset, read_sfp), "(ulong)stack_u32", &format!("{}", reg), "warp_idx");
                        /*
                        offset = format!("write_u32((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx, read_idx, thread_idx, scratch_space)),
                                                    (ulong)stack_u32,
                                                    {},
                                                    warp_idx, read_idx, thread_idx, scratch_space);", parameter_offset, reg);
                        */
                        offset = write_u32;
                    }
                    final_str += &format!("\t{};\n", offset);
                    sp_counter += 2;
                },
                wast::ValType::I64 => {
                    // compute the offset to read from the bottom of the stack
                    let reg = if !stack_ctx.vstack_is_empty(StackType::i64) {
                        stack_ctx.vstack_pop(StackType::i64)
                    } else {
                        String::from("(uint)(-1)")
                    };
    
                    if sp_counter > 0 {
                        let read_sfp = emit_read_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx");
                        let write_u64 = emit_write_u64_aligned(&format!("(ulong)(stack_u32-{}+{})", parameter_offset, read_sfp), "(ulong)stack_u32", &format!("{}", reg), "warp_idx");
                        /*
                        offset = format!("write_u64((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx, read_idx, thread_idx, scratch_space)),
                                                    (ulong)stack_u32,
                                                    {},
                                                    warp_idx, read_idx);", parameter_offset, reg);
                        */
                        offset = write_u64;
                    } else {
                        let read_sfp = emit_read_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx");
                        let write_u64 = emit_write_u64_aligned(&format!("(ulong)(stack_u32-{}+{})", parameter_offset, read_sfp), "(ulong)stack_u32", &format!("{}", reg), "warp_idx");
                        /*
                        offset = format!("write_u64((ulong)(stack_u32-{}+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx, read_idx, thread_idx, scratch_space)),
                                                    (ulong)stack_u32,
                                                    {},
                                                    warp_idx, read_idx);", parameter_offset, reg);
                        */
                        offset = write_u64;
                    }
                    final_str += &format!("\t{};\n", offset);
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
                        let read_sfp = emit_read_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx");
                        let write_u32 = emit_write_u32_aligned(&format!("(ulong)(stack_u32-{}+{})", parameter_offset, read_sfp), "(ulong)stack_u32", &format!("temp"), "warp_idx");
                        offset = write_u32;
                    } else {
                        let read_sfp = emit_read_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx");
                        let write_u32 = emit_write_u32_aligned(&format!("(ulong)(stack_u32-{}+{})", parameter_offset, read_sfp), "(ulong)stack_u32", &format!("temp"), "warp_idx");
                        offset = write_u32;
                    }

                    final_str += &format!("\t{{\n");
                    if !stack_ctx.vstack_is_empty(StackType::f32) {
                        let reg = stack_ctx.vstack_pop(StackType::f32);
                        final_str += &format!("\t\tuint temp = 0;\n");
                        final_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(float));\n", reg);
                    } else {
                        final_str += &format!("\t\tuint temp = -1.0f;\n");
                    }
                    final_str += &format!("\t\t{};\n", offset);
                    final_str += &format!("\t}}\n");
                    sp_counter += 2;
                },
                wast::ValType::F64 => {
                    // compute the offset to read from the bottom of the stack
                    if sp_counter > 0 {
                        let read_sfp = emit_read_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx");
                        let write_u64 = emit_write_u64_aligned(&format!("(ulong)(stack_u32-{}+{})", parameter_offset, read_sfp), "(ulong)stack_u32", &format!("temp"), "warp_idx");
                        offset = write_u64;
                    } else {
                        let read_sfp = emit_read_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx");
                        let write_u64 = emit_write_u64_aligned(&format!("(ulong)(stack_u32-{}+{})", parameter_offset, read_sfp), "(ulong)stack_u32", &format!("temp"), "warp_idx");
                        offset = write_u64;
                    }

                    final_str += &format!("\t{{\n");
                    if !stack_ctx.vstack_is_empty(StackType::f64) {
                        let reg = stack_ctx.vstack_pop(StackType::f64);
                        final_str += &format!("\t\tulong temp = 0;\n");
                        final_str += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n", reg);
                    } else {
                        final_str += &format!("\t\tulong temp = -1.0f;\n");
                    }
                    final_str += &format!("\t\t{};\n", offset);
                    final_str += &format!("\t}}\n");
                    sp_counter += 2;
                },
                _ => panic!("Unimplemented function return type!!!"),
            }
        }    
    }

    // If we aren't the start function we need to properly unwind
    if !is_start_fn && !is_fastcall {
        // load the entry point to return to in the control function
        final_str += &format!("\t*entry_point = {};\n", emit_read_u32_aligned("(ulong)(call_return_stack+*sfp)", "(ulong)(call_return_stack)", "warp_idx"));

        final_str += &format!("\t{}\n",
                                // reset the stack pointer to point at the end of the previous frame
                                &format!("*sp = {};", emit_read_u32_aligned("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx")));

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
pub fn emit_call_indirect(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, call_indirect: &wast::CallIndirect, curr_fn_name: String, fastcalls: &HashSet<String>, table: &HashMap<u32, &wast::Index>, call_ret_map: &mut HashMap<&str, u32>, call_ret_idx: &mut u32, call_indirect_count: &mut u32, function_id_map: HashMap<&str, u32>, call_indirect_type_index: String, debug: bool) -> String {
    let mut result = String::from("");
    // set up a switch case statement, we read the last value on the stack and determine what function we are going to call
    // this adds code bloat, but it reduces the complexity of the compiler.
    // It is worth revisiting this later, but not urgent. 

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
            for (_, _, _) in func_type.params.iter() {
                let (param, param_type) = stack_ctx.vstack_pop_any();
                stack_params.insert(0, param);
                stack_params_types.insert(0, param_type);
            }

            // Next, push the result(s) back
            for return_type in func_type.results.iter() {
                result_types.push(return_type);
            }
        },
        (_, Some(_inline)) => panic!("Inline types for call_indirect not implemented yet"),
        _ => (),
    };

    let save_ctx = stack_ctx.save_context(false, false);
    let restore_ctx = stack_ctx.restore_context(false, false);

    // First, generate the code for fastcall optimized cases for the indirect call
    // Allocate a register for return values
    let result_register = if result_types.len() > 0 {
        stack_ctx.vstack_alloc(StackCtx::convert_wast_types(&result_types[0]))
    } else {
        String::from("")
    };

    // generate a temp var to store the result of attempting to perform an optmized indirect call
    
    result += &format!("\tuchar indirect{} = 1;\n", call_indirect_count);

    result += &format!("\t{}\n",
                       &format!("switch({}) {{", index_register));
    /* Only emit cases here that:
     * 1) match the type signature
     * 2) are fastcall optimized
     * 3) are not recursive
     */ 
    for (_key, value) in table {
        let f_name = match **value {
            wast::Index::Id(id) => id.name().to_string(),
            wast::Index::Num(val, _) => format!("func_{}", val),
        };
        let func_type_signature = &writer.func_map.get(&f_name).unwrap().ty;
        let _func_type_index = match func_type_signature.index {
            Some(wast::Index::Id(id)) => id.name().to_string(),
            Some(wast::Index::Num(val, _)) => format!("t{}", val),
            None => panic!("Only type indicies supported for call_indirect in call_indirect (functions.rs)"),
        };

        /*
        // fastcalls as indirect calls appear to overflow the stack, related to hardware stack limits
        // TODO: figure out a way to re-enable this
        if fastcalls.contains(&f_name) &&
           curr_fn_name != f_name &&
           func_type_index == call_indirect_type_index {
            result += &format!("\t\t{}\n", format!("case {}:", key));
            result += &format!("{}", emit_fn_call(writer, stack_ctx, curr_fn_name.clone(), **value, call_ret_map, call_ret_idx, &function_id_map, true, true, result_register.clone(), stack_params.clone(), debug));
            result += &format!("\t\t\t{}\n", format!("break;"));
        }
        */
    }

    // emit a default case, to handle lookups to invalid indicies!
    result += &format!("\t\t{}\n", "default:");
    // Set a flag indicating we didn't perform the fastcall
    result += &format!("\t\t\tindirect{} = 0;\n", call_indirect_count);
    result += &format!("\t\t\t{}\n", "break;");
    result += &format!("\t}}\n");


    // If we successfully performed a hypercall the temp var is now set to true, so we can check that to see if we can skip
    result += &format!("\tif (indirect{}) {{\n", call_indirect_count);
    result += &format!("\t\tgoto call_indirect_fastpath_{};\n", call_indirect_count);
    result += &format!("\t}}\n");


    // After trying to perform fastcalls, we check the remaining cases
    // Save the context before entering the switch case
    result += &save_ctx;

    // Push the parameters to the stack
    for (param, ty) in stack_params.iter().zip(stack_params_types.iter()) {
        match ty {
            StackType::i32 => {
                result += &format!("\t{};\n\t*sp += 2;\n",
                                        emit_write_u32_aligned("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", &param, "warp_idx"));
            },
            StackType::i64 => {
                result += &format!("\t{};\n\t*sp += 2;\n",
                                        emit_write_u64_aligned("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", &param, "warp_idx"));
            },
            StackType::f32 => {
                result += &format!("\t{{\n");
                result += &format!("\t\tuint temp = 0;\n");
                result += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(uint));\n", param);
                result += &format!("\t\t{};\n\t\t*sp += 2;\n",
                                    emit_write_u32_aligned("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", "temp", "warp_idx"));
                result += &format!("\t}}\n");
            },
            StackType::f64 => {
                result += &format!("\t{{\n");
                result += &format!("\t\tulong temp = 0;\n");
                result += &format!("\t\t___private_memcpy_nonmmu(&temp, &{}, sizeof(double));\n", param);
                result += &format!("\t\t{};\n\t\t*sp += 2;\n",
                                    emit_write_u64_aligned("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", "temp", "warp_idx"));
                result += &format!("\t}}\n");
            }
        }
    }

    result += &format!("\t{}\n",
                       &format!("switch({}) {{", index_register));
    // generate all of the cases in the table, all uninitialized values will trap to the default case
    for (key, value) in table {
        let f_name = match **value {
            wast::Index::Id(id) => id.name().to_string(),
            wast::Index::Num(val, _) => format!("func_{}", val),
        };
        let func_type_signature = &writer.func_map.get(&f_name).unwrap().ty;
        let func_type_index = match func_type_signature.index {
            Some(wast::Index::Id(id)) => id.name().to_string(),
            Some(wast::Index::Num(val, _)) => format!("t{}", val),
            None => panic!("Only type indicies supported for call_indirect in call_indirect (functions.rs)"),
        };

        // emit the function call here!
        if func_type_index == call_indirect_type_index {
            result += &format!("\t\t{}\n", format!("case {}:", key));
            result += &format!("{}", emit_fn_call(writer, stack_ctx, curr_fn_name.clone(), **value, call_ret_map, call_ret_idx, &function_id_map, true, false, String::from(""), vec![], debug));
            result += &format!("\t\t\t{}\n", format!("break;"));
        }
    }

    // emit a default case, to handle lookups to invalid indicies!
    result += &format!("\t\t{}\n", "default:");
    result += &format!("\t\t\t{}\n", emit_trap(TrapCode::TrapCallIndirectNotFound, true));
    result += &format!("\t\t\t{}\n", "return;");
    result += &format!("\t}}\n");

    // Restore the context
    result += &restore_ctx;

    // Read the result value into a register
    if result_types.len() > 0 {
        match StackCtx::convert_wast_types(&result_types[0]) {
            StackType::i32 => {
                result += &format!("\t{} = {};\n\t{};\n", result_register, 
                                    emit_read_u32_aligned("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                    "*sp -= 2");
            },
            StackType::i64 => {
                result += &format!("\t{} = {};\n\t{};\n", result_register, 
                                    emit_read_u64_aligned("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"),
                                    "*sp -= 2;");
            },
            StackType::f32 => {
                result += &format!("\t{{\n");
                result += &format!("\t\tuint temp = {};\n", emit_read_u32_aligned("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
                result += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(uint));\n", result_register);
                result += &format!("\t\t*sp -= 2;\n");
                result += &format!("\t}}\n");
            },
            StackType::f64 => {
                result += &format!("\t{{\n");
                result += &format!("\t\tulong temp = {};\n", emit_read_u64_aligned("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx"));
                result += &format!("\t\t___private_memcpy_nonmmu(&{}, &temp, sizeof(ulong));\n", result_register);
                result += &format!("\t\t*sp -= 2;\n");
                result += &format!("\t}}\n");
            }
        }
    }

    result += &format!("\tcall_indirect_fastpath_{}:\n", call_indirect_count);

    *call_indirect_count += 1;

    result
}
