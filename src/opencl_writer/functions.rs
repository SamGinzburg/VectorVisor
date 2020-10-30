use crate::opencl_writer;
use std::collections::HashMap;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;

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
        //"stack_frames[*sfp] = *sp;",
        format!("{};", emit_write_u32("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "*sp", "warp_idx")),
        // save the callee return stub number
        //format!("call_stack[*sfp] = {};", *call_ret_idx),
        format!("{}", &format!("{};",
                      emit_write_u64("(ulong)(call_stack+*sfp)",
                                     "(ulong)(call_stack)",
                                     &format!("{}", *call_ret_idx), "warp_idx"))),
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
                                                read_u32((ulong)(stack_u32+*sp-1), (ulong)stack_u32, warp_idx),
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
                                                read_u32((ulong)(stack_u32+*sp-1), (ulong)stack_u32, warp_idx),
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
                            &format!("*sp = {};", emit_read_u32("(ulong)(stack_frames+*sfp)", "(ulong)(stack_frames)", "warp_idx")));
                            //"*sp = stack_frames[*sfp];");
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
pub fn emit_call_indirect(writer: &opencl_writer::OpenCLCWriter, table: &HashMap<u32, &wast::Index>, call_ret_map: &mut HashMap<&str, u32>, call_ret_idx: &mut u32, debug: bool) -> String {
    dbg!(table);
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
        result += &format!("{}", emit_fn_call(writer, **value, call_ret_map, call_ret_idx, debug));
        result += &format!("\t\t\t{}\n", format!("break;"));
    }

    // emit a default case, to handle lookups to invalid indicies!
    result += &format!("\t\t{}\n", "default:");
    result += &format!("\t\t\t{}\n", "return;");
    result += &format!("\t}}\n");

    result
}