use crate::opencl_writer;
use crate::opencl_writer::format_fn_name;
use crate::opencl_writer::function_unwind;
use crate::opencl_writer::get_func_result;
use crate::opencl_writer::mem_interleave::emit_write_u64_aligned;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;
use crate::opencl_writer::WasmHypercallId;

use std::collections::HashMap;
use std::convert::TryInto;

use wast::core::*;
use wast::token::Id;
use wast::token::Index;

/*
 * Every time we encounter a Loop, Block, or If statement, we store the entry on the control stack
 * We store the label,
 */

#[derive(Clone, Copy, PartialEq)]
pub enum WasmBlockType {
    BasicBlock,
    LoopBlock,
    IfBlock,
}

pub type ControlStackEntryType = (
    String,
    WasmBlockType,
    i32,
    u32,
    Option<StackType>,
    Option<String>,
);

// TODO: double check the semantics of this?
pub fn emit_return(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    fn_name: &str,
    start_fn_name: String,
    hypercall_id_count: &mut u32,
    is_fastcall: bool,
    debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let fn_type = &writer.func_map.get(&fn_name.to_string()).unwrap().ty.inline;

    if fn_name.to_string() == start_fn_name {
        // emit modified func unwind for _start
        ret_str += &function_unwind(
            &writer,
            stack_ctx,
            fn_name,
            &fn_type,
            true,
            is_fastcall,
            debug,
        );
        ret_str += &writer.emit_hypercall(
            WasmHypercallId::proc_exit,
            stack_ctx,
            hypercall_id_count,
            fn_name.to_string(),
            true,
            debug,
        );
    } else {
        // to unwind from the function we unwind the call stack by moving the stack pointer
        // and returning the last value on the stack
        ret_str += &function_unwind(
            writer,
            stack_ctx,
            fn_name,
            &fn_type,
            false,
            is_fastcall,
            debug,
        );
    }

    ret_str
}

// this function is semantically equivalent to function_unwind
pub fn emit_br(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    idx: Index,
    fn_name: &str,
    control_stack: &mut Vec<ControlStackEntryType>,
    function_id_map: HashMap<&str, u32>,
    is_fastcall: bool,
    from_br_table: bool,
    _debug: bool,
) -> String {
    let mut ret_str = String::from("");

    // we need to do linear scans for blocks that are pre-named
    let mut temp_map: HashMap<String, ControlStackEntryType> = HashMap::new();
    for (label, block_type, reentry, loop_or_block_idx, block_result_type, result_register) in
        control_stack.clone()
    {
        temp_map.insert(
            label.to_string(),
            (
                label.to_string(),
                block_type,
                reentry,
                loop_or_block_idx,
                block_result_type,
                result_register,
            ),
        );
    }

    let (
        block_name,
        block_type,
        loop_header_reentry,
        block_or_loop_idx,
        block_result_type,
        result_register,
    ) = match idx {
        Index::Id(id) => temp_map.get(id.name()).unwrap(),
        Index::Num(value, _) => control_stack
            .get(control_stack.len() - 1 - value as usize)
            .unwrap(),
    };

    if *block_type == WasmBlockType::LoopBlock && *loop_header_reentry < 0 {
        panic!("Invalid loop re-entry point");
    }

    // First, determine if the branch is a forward branch or a backwards branch (targeting a loop header)
    // block = 0, loop = 1

    match *block_type {
        WasmBlockType::BasicBlock => {
            // Check for return values, if this branch targets a block with a return value, we need to set that return value
            // We pop the most recent value on the stack and set the result register to be equal to that
            match (block_result_type, result_register) {
                (Some(stack_size), Some(result)) => {
                    // We peak the previous value, we don't pop it!
                    let val = stack_ctx.vstack_peak(stack_size.clone(), 0);
                    ret_str += &format!("\t{} = {};\n", result, val);
                }
                _ => (),
            }

            if !is_fastcall {
                // we have to save locals on exiting the stack frame, unless we do it in br_table
                if !from_br_table {
                    ret_str += &stack_ctx.save_context(true, false);
                }
                ret_str += &format!(
                    "\t{}\n",
                    format!(
                        "goto {}_{};",
                        format!("{}{}", "__", format_fn_name(&fn_name)),
                        block_name
                    )
                );
            } else {
                ret_str += &format!(
                    "\t{}\n",
                    format!(
                        "goto {}_{}_fastcall;",
                        format!("{}{}", "__", format_fn_name(&fn_name)),
                        block_name
                    )
                );
            }
        }
        WasmBlockType::LoopBlock => {
            // For loops, we need to check if we are targeting a tainted loop
            let is_loop_tainted =
                stack_ctx.is_loop_tainted((*block_or_loop_idx).try_into().unwrap());
            if !is_fastcall && is_loop_tainted {
                // If we are targeting a loop, we have to emit a return instead, to convert the iterative loop into a recursive function call
                // save the context, since we are about to call a function (ourself)
                // in br_table we unconditionally save the current locals
                if !from_br_table {
                    ret_str += &stack_ctx.save_context(true, false);
                }

                ret_str += &format!("\t{}\n", "*sfp += 1;");
                // increment the stack frame pointer & save the label of the loop header so we return to it
                ret_str += &format!(
                    "\t{}\n",
                    &format!(
                        "{};",
                        emit_write_u64_aligned(
                            "(ulong)(call_stack+*sfp)",
                            "(ulong)(call_stack)",
                            &format!("{}", *loop_header_reentry),
                            "warp_idx"
                        )
                    )
                );

                // set our re-entry target to ourself
                ret_str += &format!(
                    "\t{}\n",
                    format!("*entry_point = {};", function_id_map.get(fn_name).unwrap())
                );
                // set is_calling to false to perform the recursive call to ourself
                // upon re-entry, we will pop off the top call_stack value which will be pointing at our loop header
                ret_str += &format!("\t{}\n", "*is_calling = 0;");
                ret_str += &format!("\t{}\n", "return;");
            } else if !is_loop_tainted && !is_fastcall {
                ret_str += &format!(
                    "\t{}\n",
                    format!(
                        "goto {}_{}_loop;",
                        format!("{}{}", "__", format_fn_name(&fn_name)),
                        block_name
                    )
                );
            } else {
                ret_str += &format!(
                    "\t{}\n",
                    format!(
                        "goto {}_{}_fastcall;",
                        format!("{}{}", "__", format_fn_name(&fn_name)),
                        block_name
                    )
                );
            }
        }
        WasmBlockType::IfBlock => {
            panic!("IfBlock not handled for br statements yet: {:?}", stack_ctx);
        }
    }

    ret_str
}

pub fn emit_br_if(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    idx: Index,
    fn_name: &str,
    control_stack: &mut Vec<ControlStackEntryType>,
    function_id_map: HashMap<&str, u32>,
    is_fastcall: bool,
    debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let reg = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\tif ({} != 0) {{\n", reg);
    ret_str += &emit_br(
        writer,
        stack_ctx,
        idx,
        fn_name,
        control_stack,
        function_id_map,
        is_fastcall,
        false,
        debug,
    );
    ret_str += &format!("\t}}\n");

    ret_str
}

pub fn emit_end<'a>(
    _writer: &opencl_writer::OpenCLCWriter<'a>,
    stack_ctx: &mut StackCtx,
    _id: &Option<Id<'a>>,
    label: &str,
    block_type: WasmBlockType,
    fn_name: &str,
    result_type: Option<StackType>,
    result_register: Option<String>,
    is_fastcall: bool,
    loop_idx: u32,
    _debug: bool,
) -> String {
    let mut result = String::from("");

    // after a block ends, we need to unwind the stack!
    // let re = Regex::new(r"\d+").unwrap();
    // we can use the branch index to save to global state

    //let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
    //let branch_idx_u32 = branch_idx.parse::<u32>().unwrap();

    // This check has to happen before we pop the stack frame
    // We also have to pop the return value here, before the pop the whole stack frame!
    let (has_hanging_value, ret_val) = match (result_type.clone(), block_type) {
        (Some(ty), _) => {
            let hanging_val = stack_ctx.vstack_check_for_hanging_value(ty.clone());
            let r_val = if hanging_val {
                Some(stack_ctx.vstack_pop(ty))
            } else {
                None
            };
            (hanging_val, r_val)
        }
        (_, _) => (false, None),
    };

    // before unwinding the stack frame, save the current locals for blocks & loops
    if !is_fastcall && block_type == WasmBlockType::BasicBlock {
        result += &stack_ctx.save_context(true, false);
    } else if !is_fastcall && block_type == WasmBlockType::LoopBlock {
        // For loops that are tainted, we emit a normal save ctx, for optimized loops we want to
        // save everything that would have been saved while nested
        let is_tainted = stack_ctx.is_loop_tainted((loop_idx).try_into().unwrap());
        if is_tainted {
            result += &stack_ctx.save_context(true, false);
        } else {
            // If we are closing an optimized loop, we have to decrement the counter here
            stack_ctx.close_opt_loop();
            result += &stack_ctx.save_context(true, false);
        }
    }

    // unwind the stack frame
    stack_ctx.vstack_pop_stack_frame(block_type == WasmBlockType::IfBlock);
    stack_ctx.vstack_pop_stack_info();

    /*
     * For loops we have the following edge case:
     * loop (result i32)
     *  br 0
     * end
     *
     * Where we have an infinite loop that returns an i32.
     * We check to see if anything is on the stack first, then we return a value if we can.
     */
    result += &match (ret_val, result_register) {
        (Some(return_value), Some(result_register)) => {
            if block_type == WasmBlockType::LoopBlock && !has_hanging_value {
                String::from("")
            } else {
                format!("\t{} = {};\n", result_register, return_value)
            }
        }
        (_, _) => String::from(""),
    };

    // if the end statement corresponds to a block -> we want to put the label *here* and not at the top
    // of the block, otherwise for loops we jump back to the start of the loop!
    // 0 -> block (label goes here, at the end statement)
    // 1 -> loop (label was already inserted at the top, this is a no-op here)
    // 2 -> if statement (insert closing bracket)
    if block_type == WasmBlockType::BasicBlock {
        if !is_fastcall {
            result += &format!(
                "\n{}_{}:\n",
                format!("{}{}", "__", format_fn_name(&fn_name)),
                label
            );
        } else {
            result += &format!(
                "\n{}_{}_fastcall:\n",
                format!("{}{}", "__", format_fn_name(&fn_name)),
                label
            );
        }
    } else if block_type == WasmBlockType::LoopBlock {
        result += &format!(
            "\t/* END (loop: {}_{}) */\n",
            format!("{}{}", "__", format_fn_name(&fn_name)),
            label
        );
    } else if block_type == WasmBlockType::IfBlock {
        result += &format!("\t{}_{}_end:\n", format_fn_name(&fn_name), label);
    }

    if !is_fastcall && block_type == WasmBlockType::LoopBlock {
        let is_tainted = stack_ctx.is_loop_tainted((loop_idx).try_into().unwrap());
        // we only want to load values if we saved them previously for optimized loops!
        // this is because we only emit the save for tainted loops
        if is_tainted {
            result += &stack_ctx.restore_context_with_result_val(false, false, result_type);
        }
    } else if !is_fastcall && block_type == WasmBlockType::BasicBlock {
        // for blocks only restore locals
        result += &stack_ctx.restore_context_with_result_val(false, false, result_type);
    }

    result
}

// basically the same as emit_block, except we have to reset the stack pointer
// at the *top* of the block, since we are doing a backwards jump not a forward jump
pub fn emit_loop(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    control_stack: &mut Vec<ControlStackEntryType>,
    block: &BlockType,
    label: String,
    loop_branch_idx: &mut u32,
    fn_name: &str,
    _function_id_map: HashMap<&str, u32>,
    call_ret_idx: &mut u32,
    is_fastcall: bool,
    is_loop_tainted: bool,
    _debug: bool,
) -> String {
    let mut result: String = String::from("");

    // Get the type of the block
    let block_type = get_func_result(&writer, &block.ty);

    let result_register = match block_type {
        Some(StackType::i32) => Some(stack_ctx.vstack_alloc(StackType::i32)),
        Some(StackType::i64) => Some(stack_ctx.vstack_alloc(StackType::i64)),
        Some(StackType::f32) => Some(stack_ctx.vstack_alloc(StackType::f32)),
        Some(StackType::f64) => Some(stack_ctx.vstack_alloc(StackType::f64)),
        Some(StackType::u128) => Some(stack_ctx.vstack_alloc(StackType::u128)),
        None => None,
    };

    // the third parameter in the control stack stores loop header entry points
    control_stack.push((
        label.to_string(),
        WasmBlockType::LoopBlock,
        (*call_ret_idx).try_into().unwrap(),
        *loop_branch_idx,
        block_type,
        result_register,
    ));
    *loop_branch_idx += 1;

    if !is_fastcall && is_loop_tainted {
        // We need to save before we push the new stack frame
        result += &stack_ctx.save_context(false, false);
        stack_ctx.vstack_push_stack_frame(false, false);
        stack_ctx.vstack_push_stack_info(stack_ctx.stack_frame_size().try_into().unwrap());

        // we convert our loop into a recursive call here - the loop header is treated as a function call re-entry point
        result += &format!(
            "{}_call_return_stub_{}:\n",
            format!("{}{}", "__", format_fn_name(&fn_name)),
            *call_ret_idx
        );

        // We have to issue a restore here because on subsequent invocations the state will have changed
        // only restore locals here

        result += &stack_ctx.restore_context(true, false);

        *call_ret_idx += 1;
    } else {
        // we need to save locals in these cases as well
        if !is_fastcall {
            result += &stack_ctx.save_context(true, false);
        }

        // save a stack frame but don't save the context here
        stack_ctx.vstack_push_stack_frame(true, false);
        stack_ctx.vstack_push_stack_info(stack_ctx.stack_frame_size().try_into().unwrap());

        // emit just the loop header for GOTOs during fastcalls or for non-tainted loops
        if is_fastcall {
            result += &format!(
                "{}\n",
                format!(
                    "{}_{}_fastcall:",
                    format!("{}{}", "__", format_fn_name(&fn_name)),
                    label
                )
            );
        } else {
            // Emit optimized loops for non-tainted cases
            // We want to load all the values we could need, so we disable liveness analysis here
            result += &stack_ctx.restore_context(true, false);
            result += &format!(
                "{}\n",
                format!(
                    "{}_{}_loop:",
                    format!("{}{}", "__", format_fn_name(&fn_name)),
                    label
                )
            );
            // Set the vstack internal tracking state so we don't emit save/restore points inside
            // of the optimized loop
            stack_ctx.open_opt_loop();
        }
    }

    result
}

pub fn emit_block(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _block: &BlockType,
    _label: String,
    _branch_idx_u32: u32,
    _fn_name: &str,
    _function_id_map: HashMap<&str, u32>,
    is_fastcall: bool,
    _debug: bool,
) -> String {
    let mut result: String = String::from("");

    if !is_fastcall {
        result += &stack_ctx.save_context(false, false);
    }

    stack_ctx.vstack_push_stack_frame(false, false);
    stack_ctx.vstack_push_stack_info(stack_ctx.stack_frame_size().try_into().unwrap());

    // we need to load the locals at the top of each block
    if !is_fastcall {
        result += &stack_ctx.restore_context(true, false);
    }

    // we don't emit a label for block statements here, any br's goto the END of the block
    // we don't need to modify the sp here, we will do all stack unwinding in the br instr
    result
}

pub fn emit_if(
    writer: &opencl_writer::OpenCLCWriter,
    label: String,
    fn_name: String,
    block: &BlockType,
    control_stack: &mut Vec<ControlStackEntryType>,
    if_name_count: &mut u32,
    stack_ctx: &mut StackCtx,
) -> String {
    let mut result: String = String::from("");

    // Pop the top value on the stack as the conditional
    result += &format!("\tif (!{}) {{\n", stack_ctx.vstack_pop(StackType::i32));
    // If jump to the else block (if we have one)
    if stack_ctx.if_has_else((*if_name_count).try_into().unwrap()) {
        result += &format!("\t\tgoto {}_{}_else;\n", format_fn_name(&fn_name), label);
    } else {
        // if we don't have an else block, jump to end
        result += &format!("\t\tgoto {}_{}_end;\n", format_fn_name(&fn_name), label);
    }

    result += &format!("\t}}\n");

    // Get the type of the block
    let block_type = get_func_result(writer, &block.ty);
    // Allocate a register to store the result in after the block exits, if we have one
    // We pop this value back during the corresponding `end` instruction, since WASM does not allow hanging values
    let result_register = match block_type {
        Some(StackType::i32) => Some(stack_ctx.vstack_alloc(StackType::i32)),
        Some(StackType::i64) => Some(stack_ctx.vstack_alloc(StackType::i64)),
        Some(StackType::f32) => Some(stack_ctx.vstack_alloc(StackType::f32)),
        Some(StackType::f64) => Some(stack_ctx.vstack_alloc(StackType::f64)),
        Some(StackType::u128) => Some(stack_ctx.vstack_alloc(StackType::u128)),
        None => None,
    };

    // Now save the stack frame
    stack_ctx.vstack_push_stack_frame(true, true);
    stack_ctx.vstack_push_stack_info(stack_ctx.stack_frame_size().try_into().unwrap());

    // for the control stack, we don't use the third parameter for blocks
    control_stack.push((
        label,
        WasmBlockType::IfBlock,
        -1,
        *if_name_count,
        block_type,
        result_register,
    ));
    *if_name_count += 1;

    result
}

pub fn emit_else(
    _writer: &opencl_writer::OpenCLCWriter,
    fn_name: String,
    control_stack: &mut Vec<ControlStackEntryType>,
    stack_ctx: &mut StackCtx,
) -> String {
    let mut result: String = String::from("");
    let mut else_label = None;

    // If the most recent if statement has a result type, we need to set the value before continuing to the next branch
    let mut control_stack_copy = control_stack.clone();
    control_stack_copy.reverse();
    for (if_label, block_type, _, _, block_result_type, result_register) in control_stack_copy {
        // We found the matching if entry
        if block_type == WasmBlockType::IfBlock {
            match (block_result_type, result_register) {
                (Some(t), Some(result_register)) => {
                    else_label = Some(if_label);
                    let val = stack_ctx.vstack_pop(t);
                    result += &format!("\t{} = {};\n", result_register, val);
                    break;
                }
                _ => {
                    // If the if statement doesn't return a value, we still have to emit the else
                    else_label = Some(if_label);
                    break;
                }
            }
        }
    }

    match else_label {
        Some(label) => {
            // If we just ran the first code block of the If block, then jump to the end
            result += &format!("\tgoto {}_{}_end;\n", format_fn_name(&fn_name), label);
            // Else, put a label here for the header of the If block to jump to the second code block
            result += &format!("\t{}_{}_else:\n", format_fn_name(&fn_name), label);
        }
        None => (),
    }

    result
}

pub fn emit_br_table(
    writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    table_indicies: &BrTableIndices,
    fn_name: &str,
    control_stack: &mut Vec<ControlStackEntryType>,
    function_id_map: HashMap<&str, u32>,
    is_fastcall: bool,
    debug: bool,
) -> String {
    let mut ret_str = String::from("");

    let indicies = &table_indicies.labels;

    // read the label_idx from stack, always i32
    let label_idx = stack_ctx.vstack_pop(StackType::i32);

    // save locals
    if !is_fastcall {
        ret_str += &stack_ctx.save_context(true, false);
    }

    // generate a switch case for each label index
    ret_str += &format!("\tswitch({}) {{\n", label_idx);

    for index in 0..indicies.len() {
        ret_str += &format!("\t\tcase {}:\n", index);
        // emit br i
        ret_str += &emit_br(
            writer,
            stack_ctx,
            indicies[index],
            fn_name,
            control_stack,
            function_id_map.clone(),
            is_fastcall,
            true,
            debug,
        );
        ret_str += &format!("\t\t\tbreak;\n");
    }

    // we add the default index, if label_idx > than length l*
    ret_str += &format!("\t\tdefault:\n");
    // emit br i
    ret_str += &emit_br(
        writer,
        stack_ctx,
        table_indicies.default,
        fn_name,
        control_stack,
        function_id_map,
        is_fastcall,
        true,
        debug,
    );
    ret_str += &format!("\t\t\tbreak;\n");

    ret_str += &format!("\t}}\n");

    ret_str
}
