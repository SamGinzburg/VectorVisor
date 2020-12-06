use crate::opencl_writer;
use crate::opencl_writer::Regex;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u64;

use std::collections::HashMap;

// TODO: double check the semantics of this? 
pub fn emit_return(writer: &opencl_writer::OpenCLCWriter, fn_name: &str, debug: bool) -> String {
    // strip illegal chars from fn name
    format!("\tgoto {}_return;\n", format!("{}{}", "__", fn_name.replace(".", "")))
}

// this function is semantically equivalent to function_unwind
pub fn emit_br(writer: &opencl_writer::OpenCLCWriter, idx: wast::Index, fn_name: &str, control_stack: &mut Vec<(String, u32, i32)>, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
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
        ret_str += &format!("\t{}\n", format!("goto {}_{};", format!("{}{}", "__", fn_name.replace(".", "")), block_name));
    } else {
        // If we are targeting a loop, we have to emit a return instead, to convert the iterative loop into a recursive function call
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

pub fn emit_br_if(writer: &opencl_writer::OpenCLCWriter, idx: wast::Index, fn_name: &str, stack_sizes: &mut Vec<u32>, control_stack: &mut Vec<(String, u32, i32)>, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    let mut ret_str = String::from("");

    // br_if is just an if statement, if cond is true => br l else continue
    // pop the value first
    ret_str += &format!("\t{}\n",
                        format!("*sp -= {};", stack_sizes.pop().unwrap()));
    ret_str += &format!("\tif ({} != 0) {{\n", "read_u32((ulong)(stack_u32+*sp), (ulong)(stack_u32), warp_idx)");
    ret_str += &emit_br(writer, idx, fn_name, control_stack, function_id_map, debug);
    ret_str += &format!("\t}}\n");

    ret_str
}

// semantically, the end statement pops from the control stack,
// in our compiler, this is a no-op
pub fn emit_end<'a>(writer: &opencl_writer::OpenCLCWriter<'a>, id: &Option<wast::Id<'a>>, label: &str, block_type: u32, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
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
        if debug {
            format!("\n{}_{}:\n\t{}\n", format!("{}{}", "__", fn_name.replace(".", "")), label,
                format!("*sp = read_u32((ulong)(((char*)branch_value_stack_state)+(*sfp*128)+({}*4)+({}*4096)), (ulong)(branch_value_stack_state), warp_idx);",
                        branch_idx_u32, function_id_map.get(fn_name).unwrap()))
        } else {
            format!("\n{}_{}:\n\t{}\n", format!("{}{}", "__", fn_name.replace(".", "")), label,
                format!("*sp = read_u32((ulong)(((global char*)branch_value_stack_state)+(*sfp*128)+({}*4)+({}*4096)), (ulong)(branch_value_stack_state), warp_idx);",
                        branch_idx_u32, function_id_map.get(fn_name).unwrap()))
        }
    } else {
        let mut result = String::from("");
        result += &format!("\t/* END (loop: {}_{}) */\n", format!("{}{}", "__", fn_name.replace(".", "")), label);
        
        // pop the control flow stack entry (reset the stack to the state it was in before the loop)
        if debug {
            result += &format!("\t*sp = read_u32((ulong)(((char*)loop_value_stack_state)+(*sfp*128)+({}*4)+({}*4096)), (ulong)(loop_value_stack_state), warp_idx);\n",
                        branch_idx_u32, function_id_map.get(fn_name).unwrap());
        } else {
            result += &format!("\t*sp = read_u32((ulong)(((global char*)loop_value_stack_state)+(*sfp*128)+({}*4)+({}*4096)), (ulong)(loop_value_stack_state), warp_idx);\n",
                        branch_idx_u32, function_id_map.get(fn_name).unwrap());
        }

        result
    }
}

// basically the same as emit_block, except we have to reset the stack pointer
// at the *top* of the block, since we are doing a backwards jump not a forward jump
pub fn emit_loop(writer: &opencl_writer::OpenCLCWriter, block: &wast::BlockType, label: String, branch_idx_u32: u32, fn_name: &str, function_id_map: HashMap<&str, u32>, call_ret_idx: &mut u32, debug: bool) -> String {
    let mut result: String = String::from("");

    // we have to emulate a 2-D array, since openCL does not support double pts in v1.2
    // the format is (64 x 64 * number of functions),
    // so [..........] 4096 entries per function consecutively
    // lookups are done as: loop_value_stack_state[(*sfp * 64) + idx + (func_id * 4096)]
    // sfp = stack frame ptr, idx = branch ID, func_id = the numerical id of the function

    if debug {
        result += &format!("\t{}\n",
                            format!("write_u32((ulong)(((char*)loop_value_stack_state)+(*sfp*128)+({}*4)+({}*4096)), (ulong)(loop_value_stack_state), *sp, warp_idx);",
                            branch_idx_u32, function_id_map.get(fn_name).unwrap()));
    } else {
        result += &format!("\t{}\n",
                            format!("write_u32((ulong)(((global char*)loop_value_stack_state)+(*sfp*128)+({}*4)+({}*4096)), (ulong)(loop_value_stack_state), *sp, warp_idx);",
                            branch_idx_u32, function_id_map.get(fn_name).unwrap()));
    }

    // emit a label here for the END instruction to jump back here to restart the loop
    //result += &format!("{}_{}:\n", format!("{}{}", "__", fn_name.replace(".", "")), label);

    // we convert our loop into a recursive call here - the loop header is treated as a function call re-entry point

    result += &format!("{}_call_return_stub_{}:\n", format!("{}{}", "__", fn_name.replace(".", "")), *call_ret_idx);

    // pop the control flow stack entry (reset the stack to the state it was in before the loop)
    if debug {
        result += &format!("\t*sp = read_u32((ulong)(((char*)loop_value_stack_state)+(*sfp*128)+({}*4)+({}*4096)), (ulong)(loop_value_stack_state), warp_idx);\n",
                    branch_idx_u32, function_id_map.get(fn_name).unwrap());
    } else {
        result += &format!("\t*sp = read_u32((ulong)(((global char*)loop_value_stack_state)+(*sfp*128)+({}*4)+({}*4096)), (ulong)(loop_value_stack_state), warp_idx);\n",
                    branch_idx_u32, function_id_map.get(fn_name).unwrap());
    }

    *call_ret_idx += 1;

    result
}

pub fn emit_block(writer: &opencl_writer::OpenCLCWriter, block: &wast::BlockType, label: String, branch_idx_u32: u32, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    let mut result: String = String::from("");

    // we have to emulate a 2-D array, since openCL does not support double ptrs in v1.2
    // the format is (64 x 64 * number of functions),
    // so [..........] 4096 entries per function consecutively
    // lookups are done as: branch_value_stack_state[(*sfp * 64) + idx + (func_id * 4096)]
    // sfp = stack frame ptr, idx = branch ID, func_id = the numerical id of the function

    if debug {
        result += &format!("\t{}\n",
                    format!("write_u16((ulong)(((char*)branch_value_stack_state)+(*sfp*128)+({}*4)+({}*4096)), (ulong)(branch_value_stack_state), *sp, warp_idx);",
                            branch_idx_u32, function_id_map.get(fn_name).unwrap()));
    } else {
        result += &format!("\t{}\n",
                    format!("write_u16((ulong)(((global char*)branch_value_stack_state)+(*sfp*128)+({}*4)+({}*4096)), (ulong)(branch_value_stack_state), *sp, warp_idx);",
                            branch_idx_u32, function_id_map.get(fn_name).unwrap()));
    }

    // we don't emit a label for block statements here, any br's goto the END of the block
    // we don't need to modify the sp here, we will do all stack unwinding in the br instr
    result
}


pub fn emit_br_table(writer: &opencl_writer::OpenCLCWriter, table_indicies: &wast::BrTableIndices, fn_name: &str, stack_sizes: &mut Vec<u32>, control_stack: &mut Vec<(String, u32, i32)>, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    let mut ret_str = String::from("");

    let indicies = &table_indicies.labels;

    // read the label_idx from stack, always i32
    let label_idx = emit_read_u32("(ulong)(stack_u32+*sp)", "(ulong)(stack_u32)", "warp_idx");

    // pop the value we are branching on
    ret_str += &format!("\t{}\n",
                        format!("*sp -= {};", stack_sizes.pop().unwrap()));
    // generate a switch case for each label index
    ret_str += &format!("\tswitch({}) {{\n", label_idx);

    for index in 0..indicies.len() {
        ret_str += &format!("\t\tcase {}:\n", index);
        // emit br i
        ret_str += &emit_br(writer, indicies[index], fn_name, control_stack, function_id_map.clone(), debug);
        ret_str += &format!("\t\t\tbreak;\n");
    }

    // we add the default index, if label_idx > than length l*
    ret_str += &format!("\t\tdefault:\n");
    // decrement the stack
    ret_str += &format!("\t\t\t{}\n", "*sp -= 1;");
    // emit br i
    ret_str += &emit_br(writer, table_indicies.default, fn_name, control_stack, function_id_map, debug);
    ret_str += &format!("\t\t\tbreak;\n");

    ret_str += &format!("\t}}\n");

    ret_str
}