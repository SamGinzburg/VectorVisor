use crate::opencl_writer;
use crate::opencl_writer::Regex;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;
use crate::opencl_writer::function_unwind;
use crate::opencl_writer::WasmHypercallId;
use crate::opencl_writer::get_func_result;

use std::collections::HashMap;
use std::convert::TryInto;

/*
 * Every time we encounter a Loop, Block, or If statement, we store the entry on the control stack
 * We store the label, 
 */
pub type ControlStackEntryType = (String, u32, i32, u32, Option<StackType>, Option<String>);

// TODO: double check the semantics of this? 
pub fn emit_return(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, fn_name: &str, start_fn_name: String, hypercall_id_count: &mut u32, is_fastcall: bool, debug: bool) -> String {
    let mut ret_str = String::from("");

    let fn_type = &writer.func_map.get(&fn_name.to_string()).unwrap().ty.inline;

    if fn_name.to_string() == start_fn_name {
        // emit modified func unwind for _start
        ret_str += &function_unwind(&writer, stack_ctx, fn_name, &fn_type, true, is_fastcall, debug);
        ret_str += &writer.emit_hypercall(WasmHypercallId::proc_exit, stack_ctx, hypercall_id_count, fn_name.to_string(), true, debug);
    } else {
        // to unwind from the function we unwind the call stack by moving the stack pointer
        // and returning the last value on the stack 
        ret_str += &function_unwind(writer, stack_ctx, fn_name, &fn_type, false, is_fastcall, debug);
    }

    ret_str
}

// this function is semantically equivalent to function_unwind
pub fn emit_br(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, idx: wast::Index, fn_name: &str, control_stack: &mut Vec<ControlStackEntryType>, function_id_map: HashMap<&str, u32>, is_fastcall: bool, _debug: bool) -> String {
    let mut ret_str = String::from("");

    // we need to do linear scans for blocks that are pre-named
    let mut temp_map: HashMap<String, ControlStackEntryType> = HashMap::new();
    for (label, block_type, reentry, loop_or_block_idx, block_result_type, result_register) in control_stack.clone() {
        temp_map.insert(label.to_string(), (label.to_string(), block_type, reentry, loop_or_block_idx, block_result_type, result_register));
    }

    let (block_name, block_type, loop_header_reentry, block_or_loop_idx, block_result_type, result_register) = match idx {
        wast::Index::Id(id) => {
            temp_map.get(id.name()).unwrap()
        },
        wast::Index::Num(value, _) => {
            control_stack.get(control_stack.len() - 1 - value as usize).unwrap()
        },
    };

    if *block_type == 1 && *loop_header_reentry < 0 {
        panic!("Invalid loop re-entry point");
    }

    // First, determine if the branch is a forward branch or a backwards branch (targeting a loop header)
    // block = 0, loop = 1
    if *block_type == 0 {
        // Check for return values, if this branch targets a block with a return value, we need to set that return value 
        // We pop the most recent value on the stack and set the result register to be equal to that
        match (block_result_type, result_register) {
            (Some(stack_size), Some(result)) => {
                let val = stack_ctx.vstack_pop(stack_size.clone());
                ret_str += &format!("\t{} = {};\n", result, val);
            },
            _ => (),
        }

        if !is_fastcall {
            ret_str += &format!("\t{}\n", format!("goto {}_{};", format!("{}{}", "__", fn_name.replace(".", "")), block_name));
        } else {
            ret_str += &format!("\t{}\n", format!("goto {}_{}_fastcall;", format!("{}{}", "__", fn_name.replace(".", "")), block_name));
        }

    } else {
        // For loops, we need to check if we are targeting a tainted loop
        let is_loop_tainted = stack_ctx.is_loop_tainted((*block_or_loop_idx).try_into().unwrap());
        if !is_fastcall && is_loop_tainted {
            // If we are targeting a loop, we have to emit a return instead, to convert the iterative loop into a recursive function call
            // save the context, since we are about to call a function (ourself)
            ret_str += &stack_ctx.save_context(true);

            ret_str += &format!("\t{}\n",
                                "*sfp += 1;");
            // increment the stack frame pointer & save the label of the loop header so we return to it
            ret_str += &format!("\t{}\n", &format!("{};",
                        emit_write_u64("(ulong)(call_stack+*sfp)",
                                        "(ulong)(call_stack)",
                                        &format!("{}", *loop_header_reentry), "warp_idx")));

            // set our re-entry target to ourself
            ret_str += &format!("\t{}\n",
                                format!("*entry_point = {};", function_id_map.get(fn_name).unwrap()));
            // set is_calling to false to perform the recursive call to ourself
            // upon re-entry, we will pop off the top call_stack value which will be pointing at our loop header
            ret_str += &format!("\t{}\n",
                                "*is_calling = 0;");
            ret_str += &format!("\t{}\n",
                                "return;");
        } else if !is_loop_tainted && !is_fastcall {
            ret_str += &format!("\t{}\n", format!("goto {}_{}_loop;", format!("{}{}", "__", fn_name.replace(".", "")), block_name));
        } else {
            ret_str += &format!("\t{}\n", format!("goto {}_{}_fastcall;", format!("{}{}", "__", fn_name.replace(".", "")), block_name));
        }
    }
    
    
    ret_str
}

pub fn emit_br_if(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, idx: wast::Index, fn_name: &str, stack_sizes: &mut Vec<u32>, control_stack: &mut Vec<ControlStackEntryType>, function_id_map: HashMap<&str, u32>, is_fastcall: bool, debug: bool) -> String {
    let mut ret_str = String::from("");
    
    stack_sizes.pop().unwrap();

    let reg = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\tif ({} != 0) {{\n", reg);
    ret_str += &emit_br(writer, stack_ctx, idx, fn_name, control_stack, function_id_map, is_fastcall, debug);
    ret_str += &format!("\t}}\n");

    ret_str
}

// semantically, the end statement pops from the control stack,
// in our compiler, this is a no-op
pub fn emit_end<'a>(_writer: &opencl_writer::OpenCLCWriter<'a>, stack_ctx: &mut StackCtx, _id: &Option<wast::Id<'a>>, label: &str, block_type: u32, fn_name: &str, result_type: Option<StackType>, is_fastcall: bool, _debug: bool) -> String {
    let mut result = String::from("");

    // after a block ends, we need to unwind the stack!
    let re = Regex::new(r"\d+").unwrap();
    // we can use the branch index to save to global state

    let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
    let branch_idx_u32 = branch_idx.parse::<u32>().unwrap();
    if branch_idx_u32 > 1024 {
        panic!("Only up to 1024 branches per function are supported");
    }
    
    // First restore the context
    if !is_fastcall && block_type == 1 {
        // unwind the stack frame
        stack_ctx.vstack_pop_stack_frame();
        // pop the *sp tracking data
        stack_ctx.vstack_pop_stack_info();
        // restore the intermediate values only after ending a block
        result += &stack_ctx.restore_context(false, true);
    }

    // If there is a result value to push back, do it here
    // The top of the stack is the register containing the value 
    // The next value in the stack is the register we are storing the result into
    // Do this for blocks / If statements
    if block_type == 0 || block_type == 2 {
        result += &match result_type {
            Some(StackType::i32) => {
                let ret_val = stack_ctx.vstack_pop(StackType::i32);
                let result_register = stack_ctx.vstack_peak(StackType::i32, 0);
                format!("\t{} = {};\n", result_register, ret_val)
            },
            Some(StackType::i64) => {
                let ret_val = stack_ctx.vstack_pop(StackType::i64);
                let result_register = stack_ctx.vstack_peak(StackType::i64, 0);
                format!("\t{} = {};\n", result_register, ret_val)
            },
            Some(StackType::f32) => {
                let ret_val = stack_ctx.vstack_pop(StackType::f32);
                let result_register = stack_ctx.vstack_peak(StackType::f32, 0);
                format!("\t{} = {};\n", result_register, ret_val)
            },
            Some(StackType::f64) => {
                let ret_val = stack_ctx.vstack_pop(StackType::f64);
                let result_register = stack_ctx.vstack_peak(StackType::f64, 0);
                format!("\t{} = {};\n", result_register, ret_val)
            },
            None => String::from(""),
        };
    }

    // if the end statement corresponds to a block -> we want to put the label *here* and not at the top
    // of the block, otherwise for loops we jump back to the start of the loop!
    // 0 -> block (label goes here, at the end statement)
    // 1 -> loop (label was already inserted at the top, this is a no-op here)
    // 2 -> if
    if block_type == 0 {
        if !is_fastcall {
            result += &format!("\n{}_{}:\n", format!("{}{}", "__", fn_name.replace(".", "")), label);
        } else {
            result += &format!("\n{}_{}_fastcall:\n", format!("{}{}", "__", fn_name.replace(".", "")), label);
        }
    } else if block_type == 1 {
        result += &format!("\t/* END (loop: {}_{}) */\n", format!("{}{}", "__", fn_name.replace(".", "")), label);
    } else if block_type == 2 {
        // just close the brackets
        result += &format!("\t}}\n");
    }

    result
}

// basically the same as emit_block, except we have to reset the stack pointer
// at the *top* of the block, since we are doing a backwards jump not a forward jump
pub fn emit_loop(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _block: &wast::BlockType, label: String, _branch_idx_u32: u32, fn_name: &str, _function_id_map: HashMap<&str, u32>, call_ret_idx: &mut u32, is_fastcall: bool, is_loop_tainted: bool, _debug: bool) -> String {
    let mut result: String = String::from("");

    if !is_fastcall && is_loop_tainted {
        // We need to save before we push the new stack frame
        result += &stack_ctx.save_context(false);
        stack_ctx.vstack_push_stack_frame();

        // We have to save the context, since this is the entry point for a function call
        // TODO: optimize this by checking if we actually call a function inside the loop
        // we can replace with a GOTO in certain situations
        // This will have *huge* speedups for small loops
        stack_ctx.vstack_push_stack_info(stack_ctx.stack_frame_size().try_into().unwrap());

        // we convert our loop into a recursive call here - the loop header is treated as a function call re-entry point
        result += &format!("{}_call_return_stub_{}:\n", format!("{}{}", "__", fn_name.replace(".", "")), *call_ret_idx);

        // We have to issue a restore here because on subsequent invocations the state will have changed
        // only restore locals here

        result += &stack_ctx.restore_context(true, false);

        *call_ret_idx += 1;
    } else {
        // emit just the loop header for GOTOs during fastcalls or for non-tainted loops
        if is_fastcall {
            result += &format!("{}\n", format!("{}_{}_fastcall:", format!("{}{}", "__", fn_name.replace(".", "")), label));
        } else {
            // Emit optimized loops for non-tainted cases 
            result += &format!("{}\n", format!("{}_{}_loop:", format!("{}{}", "__", fn_name.replace(".", "")), label));
        }
    }

    result
}

pub fn emit_block(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, _block: &wast::BlockType, _label: String, _branch_idx_u32: u32, _fn_name: &str, _function_id_map: HashMap<&str, u32>, is_fastcall: bool, _debug: bool) -> String {
    let mut result: String = String::from("");

    /*
    if !is_fastcall {
        result += &stack_ctx.save_context(false);
        stack_ctx.vstack_push_stack_frame();
        stack_ctx.vstack_push_stack_info(stack_ctx.stack_frame_size().try_into().unwrap());
    }
    */

    // we don't emit a label for block statements here, any br's goto the END of the block
    // we don't need to modify the sp here, we will do all stack unwinding in the br instr
    result
}

pub fn emit_if(writer: &opencl_writer::OpenCLCWriter, label: String, block: &wast::BlockType, control_stack: &mut Vec<ControlStackEntryType>, if_name_count: &mut u32, stack_ctx: &mut StackCtx) -> String {
    let mut result: String = String::from("");

    // Pop the top value on the stack as the conditional
    result += &format!("\tif ({}) {{\n", stack_ctx.vstack_pop(StackType::i32));

    // Get the type of the block
    let block_type = get_func_result(writer, &block.ty);
    // Allocate a register to store the result in after the block exits, if we have one
    // We pop this value back during the corresponding `end` instruction, since WASM does not allow hanging values
    let result_register = match block_type {
        Some(StackType::i32) => {
            Some(stack_ctx.vstack_alloc(StackType::i32))
        },
        Some(StackType::i64) => {
            Some(stack_ctx.vstack_alloc(StackType::i64))
        },
        Some(StackType::f32) => {
            Some(stack_ctx.vstack_alloc(StackType::f32))
        },
        Some(StackType::f64) => {
            Some(stack_ctx.vstack_alloc(StackType::f64))
        },
        None => None,
    };

    // for the control stack, we don't use the third parameter for blocks
    control_stack.push((label, 2, -1, *if_name_count, block_type, result_register));
    *if_name_count += 1;
    

    result
}

pub fn emit_else(_writer: &opencl_writer::OpenCLCWriter, control_stack: &mut Vec<ControlStackEntryType>, stack_ctx: &mut StackCtx) -> String {
    let mut result: String = String::from("");

    // If the most recent if statement has a result type, we need to set the value before continuing to the next branch
    let mut control_stack_copy = control_stack.clone();
    control_stack_copy.reverse();
    for (_, block_type, _, _, block_result_type, result_register) in control_stack_copy {
        // We found the matching if entry
        if block_type == 2 {
            match (block_result_type, result_register) {
                (Some(t), Some(result_register)) => {
                    let val = stack_ctx.vstack_pop(t);
                    result +=&format!("\t{} = {};\n", result_register, val);
                    break;
                },
                _ => (),
            }
        }
    }
    
    result +=&format!("\t}} else {{\n");

    result
}

pub fn emit_br_table(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, table_indicies: &wast::BrTableIndices, fn_name: &str, stack_sizes: &mut Vec<u32>, control_stack: &mut Vec<ControlStackEntryType>, function_id_map: HashMap<&str, u32>, is_fastcall: bool, debug: bool) -> String {
    let mut ret_str = String::from("");

    let indicies = &table_indicies.labels;

    // read the label_idx from stack, always i32
    let label_idx = stack_ctx.vstack_pop(StackType::i32);
    stack_sizes.pop().unwrap();

    // generate a switch case for each label index
    ret_str += &format!("\tswitch({}) {{\n", label_idx);

    for index in 0..indicies.len() {
        ret_str += &format!("\t\tcase {}:\n", index);
        // emit br i
        ret_str += &emit_br(writer, stack_ctx, indicies[index], fn_name, control_stack, function_id_map.clone(), is_fastcall, debug);
        ret_str += &format!("\t\t\tbreak;\n");
    }

    // we add the default index, if label_idx > than length l*
    ret_str += &format!("\t\tdefault:\n");
    // emit br i
    ret_str += &emit_br(writer, stack_ctx, table_indicies.default, fn_name, control_stack, function_id_map, is_fastcall, debug);
    ret_str += &format!("\t\t\tbreak;\n");

    ret_str += &format!("\t}}\n");

    ret_str
}
