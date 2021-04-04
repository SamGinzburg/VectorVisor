use crate::opencl_writer;
use crate::opencl_writer::Regex;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;
use crate::opencl_writer::function_unwind;
use crate::opencl_writer::WasmHypercallId;

use std::collections::HashMap;
use std::convert::TryInto;

// TODO: double check the semantics of this? 
pub fn emit_return(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, fn_name: &str, hypercall_id_count: &mut u32, debug: bool) -> String {
    let mut ret_str = String::from("");
    
    let fn_type = &writer.func_map.get(&fn_name).unwrap().ty.inline;

    if fn_name.to_string() == "_start" {
        // emit modified func unwind for _start
        ret_str += &function_unwind(&writer, stack_ctx, fn_name, &fn_type, true, debug);
        ret_str += &writer.emit_hypercall(WasmHypercallId::proc_exit, stack_ctx, hypercall_id_count, fn_name.to_string(), true, debug);
    } else {
        // to unwind from the function we unwind the call stack by moving the stack pointer
        // and returning the last value on the stack 
        ret_str += &function_unwind(writer, stack_ctx, fn_name, &fn_type, false, debug);
    }

    ret_str
}

// this function is semantically equivalent to function_unwind
pub fn emit_br(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, idx: wast::Index, fn_name: &str, control_stack: &mut Vec<(String, u32, i32)>, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    let mut ret_str = String::from("");

    // we need to do linear scans for blocks that are pre-named
    let mut temp_map: HashMap<String, (String, u32, i32)> = HashMap::new();
    for (label, block_type, reentry) in control_stack.clone() {
        temp_map.insert(label.to_string(), (label.to_string(), block_type, reentry));
    }

    let (block_name, block_type, loop_header_reentry) = match idx {
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
        // If we are targeting a forward branch, just emit the goto

        /*
         * If our branch is used to break out of a loop by targeting a label after the loop end,
         * then we have to cleanup the stack pointer ourselves.
         * 
         * We loop over the control stack to count how many loops we are currently inside
         * In the stack context we track the space allocated for each loop (we also pop from this stack on 'end' statements)
         * 
         * We then subtract the stack pointer appropriately
         */
        let mut control_stack_copy = control_stack.clone();
        control_stack_copy.reverse();
        let mut loop_count: u32 = 0;
        for (label, is_loop, _) in control_stack_copy.iter() {
            if *is_loop == 1 {
                loop_count += 1;
            }
            if label == block_name {
                break;
            }
        }

        // Get the value to decrement *sp by in the case of this jump
        ret_str += &format!("\t*sp -= {};\n", stack_ctx.vstack_get_loop_stack_delta(loop_count));

        ret_str += &format!("\t{}\n", format!("goto {}_{};", format!("{}{}", "__", fn_name.replace(".", "")), block_name));
    } else {
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
        //ret_str += &format!("\t{}\n", format!("goto {}_{};", format!("{}{}", "__", fn_name.replace(".", "")), block_name));
    }
    
    
    ret_str
}

pub fn emit_br_if(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, idx: wast::Index, fn_name: &str, stack_sizes: &mut Vec<u32>, control_stack: &mut Vec<(String, u32, i32)>, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    let mut ret_str = String::from("");
    
    stack_sizes.pop().unwrap();

    let reg = stack_ctx.vstack_pop(StackType::i32);

    ret_str += &format!("\tif ({} != 0) {{\n", reg);
    ret_str += &emit_br(writer, stack_ctx, idx, fn_name, control_stack, function_id_map, debug);
    ret_str += &format!("\t}}\n");

    ret_str
}

// semantically, the end statement pops from the control stack,
// in our compiler, this is a no-op
pub fn emit_end<'a>(writer: &opencl_writer::OpenCLCWriter<'a>, stack_ctx: &mut StackCtx, id: &Option<wast::Id<'a>>, label: &str, block_type: u32, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {

    // unwind the stack frame
    stack_ctx.vstack_pop_stack_frame();

    // after a block ends, we need to unwind the stack!
    let re = Regex::new(r"\d+").unwrap();
    // we can use the branch index to save to global state

    let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
    let branch_idx_u32 = branch_idx.parse::<u32>().unwrap();
    if branch_idx_u32 > 1024 {
        panic!("Only up to 1024 branches per function are supported");
    }

    // if the end statement corresponds to a block -> we want to put the label *here* and not at the top
    // of the block, otherwise for loops we jump back to the start of the loop!
    // 0 -> block (label goes here, at the end statement)
    // 1-> loop (label was already inserted at the top, this is a no-op here)
    if block_type == 0 {
        /*
        format!("\n{}_{}:\n\t{}\n", format!("{}{}", "__", fn_name.replace(".", "")), label,
            format!("*sp = read_u32((ulong)(((global char*)branch_value_stack_state)+(*sfp*512)+({}*4)), (ulong)(branch_value_stack_state), warp_idx);",
                    branch_idx_u32))
        */
        format!("\n{}_{}:\n", format!("{}{}", "__", fn_name.replace(".", "")), label)
    } else {
        let mut result = String::from("");
        result += &format!("\t/* END (loop: {}_{}) */\n", format!("{}{}", "__", fn_name.replace(".", "")), label);
        
        // reset the stack pointer to the value before the loop
        /*
        result += &format!("\t*sp = read_u32((ulong)(((global char*)loop_value_stack_state)+(*sfp*512)+({}*4)), (ulong)(loop_value_stack_state), warp_idx);\n",
                    branch_idx_u32);
        */
        // remove the stack space we allocated to save the loop context
        let stack_frame_size = stack_ctx.stack_frame_size();
        result += &format!("\t*sp -= {};\n", stack_frame_size);
        // pop the loop stack tracking data
        stack_ctx.vstack_pop_loop_stack_info();
        // restore the intermediate values after ending the loop
        result += &stack_ctx.restore_context(false, true);

        result
    }
}

// basically the same as emit_block, except we have to reset the stack pointer
// at the *top* of the block, since we are doing a backwards jump not a forward jump
pub fn emit_loop(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, block: &wast::BlockType, label: String, branch_idx_u32: u32, fn_name: &str, function_id_map: HashMap<&str, u32>, call_ret_idx: &mut u32, debug: bool) -> String {
    let mut result: String = String::from("");

    // we have to emulate a 2-D array, since openCL does not support double pts in v1.2
    // For each stack frame, we store a max of 32 loop stack values
    // As of now, this value is hardcoded (can be easily expanded later), but in practice it is rare to see more than 32 loops inside the same function

    /*
    result += &format!("\t{}\n",
                        format!("write_u32((ulong)(((global char*)loop_value_stack_state)+(*sfp*512)+({}*4)), (ulong)(loop_value_stack_state), *sp, warp_idx);",
                        branch_idx_u32));
    */

    stack_ctx.vstack_push_stack_frame();

    // We have to save the context, since this is the entry point for a function call
    // TODO: optimize this by checking if we actually call a function inside the loop
    // we can replace with a GOTO in certain situations
    // This will have *huge* speedups for small loops
    stack_ctx.vstack_push_loop_stack_info(stack_ctx.stack_frame_size().try_into().unwrap());
    result += &stack_ctx.save_context(false);

    // we convert our loop into a recursive call here - the loop header is treated as a function call re-entry point
    result += &format!("{}_call_return_stub_{}:\n", format!("{}{}", "__", fn_name.replace(".", "")), *call_ret_idx);

    // We have to issue a restore here because on subsequent invocations the state will have changed
    // only restore locals here
    result += &stack_ctx.restore_context(true, false);

    // pop the control flow stack entry (reset the stack to the state it was in before the loop)
    /*
    result += &format!("\t*sp = read_u32((ulong)(((global char*)loop_value_stack_state)+(*sfp*512)+({}*4)), (ulong)(loop_value_stack_state), warp_idx);\n",
                branch_idx_u32);
    */

    *call_ret_idx += 1;

    result
}

pub fn emit_block(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, block: &wast::BlockType, label: String, branch_idx_u32: u32, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    let mut result: String = String::from("");

    stack_ctx.vstack_push_stack_frame();

    // we have to emulate a 2-D array, since openCL does not support double ptrs in v1.2
    // the format is (64 x 64 * number of functions),
    // so [..........] 4096 entries per function consecutively
    // lookups are done as: branch_value_stack_state[(*sfp * 64) + idx + (func_id * 4096)]
    // sfp = stack frame ptr, idx = branch ID, func_id = the numerical id of the function

    result += &format!("\t{}\n",
                format!("write_u32((ulong)(((global char*)branch_value_stack_state)+(*sfp*512)+({}*4)), (ulong)(branch_value_stack_state), *sp, warp_idx);",
                        branch_idx_u32));

    // we don't emit a label for block statements here, any br's goto the END of the block
    // we don't need to modify the sp here, we will do all stack unwinding in the br instr
    result
}


pub fn emit_br_table(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, table_indicies: &wast::BrTableIndices, fn_name: &str, stack_sizes: &mut Vec<u32>, control_stack: &mut Vec<(String, u32, i32)>, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
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
        ret_str += &emit_br(writer, stack_ctx, indicies[index], fn_name, control_stack, function_id_map.clone(), debug);
        ret_str += &format!("\t\t\tbreak;\n");
    }

    // we add the default index, if label_idx > than length l*
    ret_str += &format!("\t\tdefault:\n");
    // emit br i
    ret_str += &emit_br(writer, stack_ctx, table_indicies.default, fn_name, control_stack, function_id_map, debug);
    ret_str += &format!("\t\t\tbreak;\n");

    ret_str += &format!("\t}}\n");

    ret_str
}