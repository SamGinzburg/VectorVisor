use crate::opencl_writer;
use crate::opencl_writer::Regex;
use std::collections::HashMap;

// TODO: double check the semantics of this? 
pub fn emit_return(writer: &opencl_writer::OpenCLCWriter, fn_name: &str, debug: bool) -> String {
    format!("\tgoto {}_return;\n", fn_name)
}

// this function is semantically equivalent to function_unwind
pub fn emit_br(writer: &opencl_writer::OpenCLCWriter, idx: wast::Index, fn_name: &str, prev_stack_size: u32, debug: bool) -> String {
    let mut ret_str = String::from("");

    let branch_id = match idx {
        wast::Index::Id(id) => id.name(),
        _ => panic!("Branch specified in terms of numerical index instead of Id"),
    };

    let re = Regex::new(r"\d+").unwrap();
    // we can use the branch index to save to global state
    let branch_idx: &str = re.captures(branch_id).unwrap().get(0).map_or("", |m| m.as_str());

    // debug comment
    ret_str += &format!("\t{}\n", format!("/* br {} */", branch_id));

    // first we want to pop the result value off of the stack, and push it
    dbg!(prev_stack_size);
    /*
    match prev_stack_size {
        1 => {
            // first push the value back
            // next, move the stack pointer
            ret_str += &format!("\t{}\n\t{}\n",
                                format!("stack_u32[branch_value_stack_state[{}]] = stack_u32[*sp - 1];", branch_idx),
                                format!("*sp = stack_u32[branch_value_stack_state[{}]];", branch_idx));
        },
        2 => {
            panic!("u64 br l not yet implemented");
            ret_str += &format!("\t{}\n",
                                "*(ulong*)(stack_u32+*sp-4) = *(ulong*)(stack_u32+*sp-2) + *(ulong*)(stack_u32+*sp-4);");
        },
        _ => panic!("Unable to determine size of the previous item on stack"),
    };
    */

    ret_str += &format!("\t{}\n", format!("goto {};", branch_id));

    ret_str
}

pub fn emit_br_if(writer: &opencl_writer::OpenCLCWriter, idx: wast::Index, fn_name: &str, prev_stack_size: u32, debug: bool) -> String {
    let mut ret_str = String::from("");

    // br_if is just an if statement, if cond is true => br l else continue
    ret_str += &format!("\tif ({} != 0) {{\n", "read_u32((ulong)(stack_u32+*sp-1), warp_idx)");
    ret_str += &emit_br(writer, idx, fn_name, prev_stack_size, debug);
    ret_str += &format!("\t}}\n");

    ret_str
}

// semantically, the end statement pops from the control stack,
// in our compiler, this is a no-op
pub fn emit_end<'a>(writer: &opencl_writer::OpenCLCWriter<'a>, id: &Option<wast::Id<'a>>, label: &str, block_type: u32, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    dbg!(id);
    dbg!(label);
    dbg!(block_type);
    println!("emit end!");
    // after a block ends, we need to unwind the stack!
    let re = Regex::new(r"\d+").unwrap();
    // we can use the branch index to save to global state
    let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
    dbg!(branch_idx);
    let branch_idx_u32 = branch_idx.parse::<u32>().unwrap();
    if branch_idx_u32 > 1024 {
        panic!("Only up to 1024 branches per function are supported");
    }

    // if the end statement corresponds to a block -> we want to put the label *here* and not at the top
    // of the block, otherwise for loops we jump back to the start of the loop!
    // 0 -> block (label goes here, at the end statement)
    // 1-> loop (label was already inserted at the top, this is a no-op here)
    if block_type == 0 {
        format!("\n{}:\n\t{}\n", label,
                format!("*sp = read_u16((ulong)(branch_value_stack_state+(*sfp*64)+{}+({}*4096)), warp_idx);",
                        branch_idx_u32, function_id_map.get(fn_name).unwrap()))
    } else {
        let mut result = String::from("");
        result += &format!("\t/* END (loop: {}) */\n", label);
        
        // pop the control flow stack entry (reset the stack to the state it was in before the loop)
        result += &format!("\t*sp = read_u16((ulong)(branch_value_stack_state+(*sfp*64)+{}+({}*4096)), warp_idx);\n",
                            branch_idx_u32, function_id_map.get(fn_name).unwrap());

        result
    }
}

// basically the same as emit_block, except we have to reset the stack pointer
// at the *top* of the block, since we are doing a backwards jump not a forward jump
pub fn emit_loop(writer: &opencl_writer::OpenCLCWriter, block: &wast::BlockType, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    let mut result: String = String::from("");
    let label = block.label.unwrap().name();

    // first we have to save the current stack pointer
    // to reset the stack if we jump to this label
    dbg!(label);
    let re = Regex::new(r"\d+").unwrap();
    // we can use the branch index to save to global state
    let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
    dbg!(branch_idx);
    let branch_idx_u32 = branch_idx.parse::<u32>().unwrap();
    if branch_idx_u32 > 1024 {
        panic!("Only up to 1024 branches per function are supported");
    }

    // create a new stack frame for the block, store stack frame pointer in local
    // function private data

    // we have to emulate a 2-D array, since openCL does not support double pts in v1.2
    // the format is (64 x 64 * number of functions),
    // so [..........] 4096 entries per function consecutively
    // lookups are done as: branch_value_stack_state[(*sfp * 64) + idx + (func_id * 4096)]
    // sfp = stack frame ptr, idx = branch ID, func_id = the numerical id of the function

    result += &format!("\t{}\n",
                        format!("write_u16((ulong)(loop_value_stack_state+(*sfp*64)+{}+({} *4096)), (ushort)*sp, warp_idx);",
                        branch_idx_u32, function_id_map.get(fn_name).unwrap()));

    // emit a label here for the END instruction to jump back here to restart the loop
    result += &format!("{}:\n", label);
    // the stack pointer should be reset by the BR/BR_IF instruction, so no need to touch it here

    result
}

pub fn emit_block(writer: &opencl_writer::OpenCLCWriter, block: &wast::BlockType, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    let mut result: String = String::from("");
    let label = block.label.unwrap().name();

    // first we have to save the current stack pointer
    // to reset the stack if we jump to this label
    dbg!(label);
    let re = Regex::new(r"\d+").unwrap();
    // we can use the branch index to save to global state
    let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
    dbg!(branch_idx);
    let branch_idx_u32 = branch_idx.parse::<u32>().unwrap();
    if branch_idx_u32 > 1024 {
        panic!("Only up to 1024 branches per function are supported");
    }

    // create a new stack frame for the block, store stack frame pointer in local
    // function private data


    // we have to emulate a 2-D array, since openCL does not support double pts in v1.2
    // the format is (64 x 64 * number of functions),
    // so [..........] 4096 entries per function consecutively
    // lookups are done as: branch_value_stack_state[(*sfp * 64) + idx + (func_id * 4096)]
    // sfp = stack frame ptr, idx = branch ID, func_id = the numerical id of the function

    result += &format!("\t{}\n",
                        // write_
                       format!("branch_value_stack_state[(*sfp * 64) + {} + ({} * 4096)] = *sp;",
                       branch_idx_u32, function_id_map.get(fn_name).unwrap()));
    // we don't emit a label for block statements here, any br's goto the END of the block
    // we don't need to modify the sp here, we will do all stack unwinding in the br instr
    result
}
