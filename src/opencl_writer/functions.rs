use crate::opencl_writer;
use std::collections::HashMap;

pub fn emit_fn_call(writer: &opencl_writer::OpenCLCWriter, idx: wast::Index, call_ret_map: &mut HashMap<&str, u32>, call_ret_idx: &mut u32, debug: bool) -> String {
    let id = match idx {
        wast::Index::Id(id) => id.name(),
        _ => panic!("Unable to get Id for function call!"),
    };

    dbg!(&writer.func_map);
    // if the func has calling parameters, set those up
    // on the newly formed stack as well
    let func_type_signature = &writer.func_map.get(id).unwrap().ty;
    let mut offset = 0;
    for parameter in func_type_signature.clone().inline.unwrap().params.to_vec() {
        dbg!(parameter);
        match parameter {
            (Some(id), _, t) => {
                dbg!(id);
                dbg!(t);
                offset += writer.get_size_valtype(&t);
            },
            _ => panic!("Unhandled parameter type")
        }
    }

    // for each function call, map the call to an index
    // we use this index later on to return back to the instruction after the call
    
    let ret_label: &'static str = Box::leak(format!("ret_from_{}_{}", id, call_ret_idx).into_boxed_str());
    call_ret_map.insert(ret_label, *call_ret_idx);

    // get the return type of the function
    let return_size = writer.get_size_valtype(&func_type_signature.clone().inline.unwrap().results[0]);
    println!("return size: {}", return_size);
    let result = if offset > 0 {
        format!("\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n{}\n\t{}\n",
        // move the stack pointer back by the offset required by calling parameters
        // the sp should point at the start of the arguments for the function
        format!("*sp -= {};", offset),
        // increment stack frame pointer
        "*sfp += 1;",
        // save the current stack pointer for unwinding later
        "stack_frames[*sfp] = *sp;",
        // save the callee return stub number
        format!("call_stack[*sfp] = {};", *call_ret_idx),
        // setup calling parameters for function
        format!("goto {};", id),
        format!("call_return_stub_{}:", *call_ret_idx),
        format!("*sp += {};", return_size))
    } else {
        format!("\t{}\n\t{}\n\t{}\n{}\n",
                // increment stack frame pointer
                "*sfp += 1;",
                // save the current stack pointer for unwinding later
                "stack_frames[*sfp] = *sp;",
                // save the callee return stub number
                // setup calling parameters for function
                format!("goto {};", id),
                format!("call_return_stub_{}:", 0))
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
    final_str += &format!("{}_return:\n", fn_name);
    // for each value returned by the function, return it on the stack
    // keep track of the change to stack ptr from previous returns
    let mut sp_counter = 0;
    let mut offset = String::from("");
    for value in results {
        match value {
            wast::ValType::I32 => {
                // compute the offset to read from the bottom of the stack
                if sp_counter > 0 {
                    offset = format!("write_u32((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                (ulong)stack_u32,
                                                read_u32((ulong)(stack_u32+*sp-{}-1), (ulong)stack_u32, warp_idx),
                                                warp_idx);", sp_counter);
                } else {
                    offset = format!("write_u32((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_u32, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u32((ulong)(stack_u32+*sp-1), warp_idx),
                                                warp_idx);");
                }
                final_str += &format!("\t{}\n", offset);
                sp_counter += 1;
            },
            wast::ValType::I64 => {
                // compute the offset to read from the bottom of the stack
                if sp_counter > 0 {
                    offset = format!("write_u64((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_u32, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u64((ulong)(stack_u32+*sp-{}-2), (ulong)stack_u32, warp_idx),
                                                warp_idx);", sp_counter);
                } else {
                    offset = format!("write_u64((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_u32, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u64((ulong)(stack_u32+*sp-2), (ulong)stack_u32, warp_idx),
                                                warp_idx);");
                }
                final_str += &format!("\t{}\n", offset);
                sp_counter += 2;
            },
            wast::ValType::F32 => {
                // compute the offset to read from the bottom of the stack
                if sp_counter > 0 {
                    offset = format!("write_u32((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                (ulong)stack_u32,
                                                read_u32((ulong)(stack_u32+*sp-{}-1), (ulong)stack_u32, warp_idx),
                                                warp_idx);", sp_counter);
                } else {
                    offset = format!("write_u32((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_u32, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u32((ulong)(stack_u32+*sp-1), warp_idx),
                                                warp_idx);");
                }
                final_str += &format!("\t{}\n", offset);
                sp_counter += 1;
            },
            wast::ValType::F64 => {
                // compute the offset to read from the bottom of the stack
                if sp_counter > 0 {
                    offset = format!("write_u64((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_u32, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u64((ulong)(stack_u32+*sp-{}-2), (ulong)stack_u32, warp_idx),
                                                warp_idx);", sp_counter);
                } else {
                    offset = format!("write_u64((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), (ulong)stack_u32, warp_idx)),
                                                (ulong)stack_u32,
                                                read_u64((ulong)(stack_u32+*sp-2), (ulong)stack_u32, warp_idx),
                                                warp_idx);");
                }
                final_str += &format!("\t{}\n", offset);
                sp_counter += 2;
            },
            _ => panic!("Unimplemented function return type!!!"),
        }
    }
    final_str += &format!("\t{}\n",
                            // reset the stack pointer to point at the end of the previous frame
                            "*sp = stack_frames[*sfp];");
    final_str += &format!("\t{}\n\t\t{}\n\t{}\n\t\t{}\n\t{}\n",
                            // check if *sfp == 0
                            "if (*sfp != 0) {",
                            // if *sfp != 0, that means we have to return to the previous stack frame
                                "goto function_return_stub;",
                            "} else {",
                            // we are the top-level stack frame, and can now exit the program
                                "return;",
                            "}");
    final_str
}
