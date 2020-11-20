use crate::opencl_writer;
use crate::opencl_writer::Regex;
use crate::opencl_writer::mem_interleave::emit_read_u32;

use std::collections::HashMap;

// TODO: double check the semantics of this? 
pub fn emit_return(writer: &opencl_writer::OpenCLCWriter, fn_name: &str, debug: bool) -> String {
    // strip illegal chars from fn name
    format!("\tgoto {}_return;\n", fn_name.replace(".", "").replace("$", ""))
}

// this function is semantically equivalent to function_unwind
pub fn emit_br(writer: &opencl_writer::OpenCLCWriter, idx: wast::Index, fn_name: &str, debug: bool) -> String {
    let mut ret_str = String::from("");

    let branch_id = match idx {
        wast::Index::Id(id) => id.name(),
        _ => panic!("Branch specified in terms of numerical index instead of Id"),
    };

    // debug comment
    ret_str += &format!("\t{}\n", format!("/* br {}_{} */", fn_name.replace(".", "").replace("$", ""), branch_id));
    // strip illegal chars from function name
    ret_str += &format!("\t{}\n", format!("goto {}_{};", fn_name.replace(".", "").replace("$", ""), branch_id));

    ret_str
}

pub fn emit_br_if(writer: &opencl_writer::OpenCLCWriter, idx: wast::Index, fn_name: &str, debug: bool) -> String {
    let mut ret_str = String::from("");

    // br_if is just an if statement, if cond is true => br l else continue
    ret_str += &format!("\tif ({} != 0) {{\n", "read_u32((ulong)(stack_u32+*sp-1), (ulong)(stack_u32), warp_idx)");
    ret_str += &emit_br(writer, idx, fn_name, debug);
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
            format!("\n{}_{}:\n\t{}\n", fn_name.replace(".", "").replace("$", ""), label,
                format!("*sp = read_u16((ulong)(((char*)branch_value_stack_state)+(*sfp*128)+({}*2)+({}*4096)), (ulong)(branch_value_stack_state), warp_idx);",
                        branch_idx_u32, function_id_map.get(fn_name).unwrap()))
        } else {
            format!("\n{}_{}:\n\t{}\n", fn_name.replace(".", "").replace("$", ""), label,
                format!("*sp = read_u16((ulong)(((global char*)branch_value_stack_state)+(*sfp*128)+({}*2)+({}*4096)), (ulong)(branch_value_stack_state), warp_idx);",
                        branch_idx_u32, function_id_map.get(fn_name).unwrap()))
        }
    } else {
        let mut result = String::from("");
        result += &format!("\t/* END (loop: {}_{}) */\n", fn_name.replace(".", "").replace("$", ""), label);
        
        // pop the control flow stack entry (reset the stack to the state it was in before the loop)
        if debug {
            result += &format!("\t*sp = read_u16((ulong)(((char*)branch_value_stack_state)+(*sfp*128)+({}*2)+({}*4096)), (ulong)(branch_value_stack_state), warp_idx);\n",
                        branch_idx_u32, function_id_map.get(fn_name).unwrap());
        } else {
            result += &format!("\t*sp = read_u16((ulong)(((global char*)branch_value_stack_state)+(*sfp*128)+({}*2)+({}*4096)), (ulong)(branch_value_stack_state), warp_idx);\n",
                        branch_idx_u32, function_id_map.get(fn_name).unwrap());
        }

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
    let re = Regex::new(r"\d+").unwrap();
    // we can use the branch index to save to global state
    let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
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

    if debug {
        result += &format!("\t{}\n",
                            format!("write_u16((ulong)(((char*)loop_value_stack_state)+(*sfp*128)+({}*2)+({}*4096)), (ulong)(branch_value_stack_state), (ushort)*sp, warp_idx);",
                            branch_idx_u32, function_id_map.get(fn_name).unwrap()));
    } else {
        result += &format!("\t{}\n",
                            format!("write_u16((ulong)(((global char*)loop_value_stack_state)+(*sfp*128)+({}*2)+({}*4096)), (ulong)(branch_value_stack_state), (ushort)*sp, warp_idx);",
                            branch_idx_u32, function_id_map.get(fn_name).unwrap()));
    }

    // emit a label here for the END instruction to jump back here to restart the loop
    result += &format!("{}_{}:\n", fn_name.replace(".", "").replace("$", ""), label);
    // the stack pointer should be reset by the BR/BR_IF instruction, so no need to touch it here

    result
}

pub fn emit_block(writer: &opencl_writer::OpenCLCWriter, block: &wast::BlockType, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
    let mut result: String = String::from("");
    let label = block.label.unwrap().name();

    // first we have to save the current stack pointer
    // to reset the stack if we jump to this label
    let re = Regex::new(r"\d+").unwrap();
    // we can use the branch index to save to global state
    let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
    let branch_idx_u32 = branch_idx.parse::<u32>().unwrap();
    if branch_idx_u32 > 1024 {
        panic!("Only up to 1024 branches per function are supported");
    }

    // create a new stack frame for the block, store stack frame pointer in local
    // function private data


    // we have to emulate a 2-D array, since openCL does not support double ptrs in v1.2
    // the format is (64 x 64 * number of functions),
    // so [..........] 4096 entries per function consecutively
    // lookups are done as: branch_value_stack_state[(*sfp * 64) + idx + (func_id * 4096)]
    // sfp = stack frame ptr, idx = branch ID, func_id = the numerical id of the function

    if debug {
        result += &format!("\t{}\n",
                    format!("write_u16((ulong)(((char*)loop_value_stack_state)+(*sfp*128)+({}*2)+({}*4096)), (ulong)(branch_value_stack_state), (ushort)*sp, warp_idx);",
                            branch_idx_u32, function_id_map.get(fn_name).unwrap()));
    } else {
        result += &format!("\t{}\n",
                    format!("write_u16((ulong)(((global char*)loop_value_stack_state)+(*sfp*128)+({}*2)+({}*4096)), (ulong)(branch_value_stack_state), (ushort)*sp, warp_idx);",
                            branch_idx_u32, function_id_map.get(fn_name).unwrap()));
    }

    // we don't emit a label for block statements here, any br's goto the END of the block
    // we don't need to modify the sp here, we will do all stack unwinding in the br instr
    result
}


pub fn emit_br_table(writer: &opencl_writer::OpenCLCWriter, table_indicies: &wast::BrTableIndices, fn_name: &str, debug: bool) -> String {
    let mut ret_str = String::from("");

    let indicies = &table_indicies.labels;

    // read the label_idx from stack, always i32
    let label_idx = emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx");

    // generate a switch case for each label index
    ret_str += &format!("\tswitch({}) {{\n", label_idx);

    for index in 0..indicies.len() {
        ret_str += &format!("\t\tcase {}:\n", index);
        // decrement the stack
        ret_str += &format!("\t\t\t{}\n", "*sp -= 1;");
        // emit br i
        ret_str += &emit_br(writer, indicies[index], fn_name, debug);
        ret_str += &format!("\t\t\tbreak;\n");
    }

    // we add the default index, if label_idx > than length l*
    ret_str += &format!("\t\tdefault:\n");
    // decrement the stack
    ret_str += &format!("\t\t\t{}\n", "*sp -= 1;");
    // emit br i
    ret_str += &emit_br(writer, table_indicies.default, fn_name, debug);
    ret_str += &format!("\t\t\tbreak;\n");

    ret_str += &format!("\t}}\n");

    ret_str
}