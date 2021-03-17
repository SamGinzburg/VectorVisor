use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;

pub fn emit_select(writer: &opencl_writer::OpenCLCWriter, stack_sizes: &mut Vec<u32>, fn_name: &str, debug: bool) -> String {
    let mut ret_str = String::from("");

    // pop c
    let c = emit_read_u32("(ulong)(stack_u32+*sp-1)", "(ulong)(stack_u32)", "warp_idx");

    if stack_sizes.pop().unwrap() != 1 {
        panic!("select in fn: {}, the top of the stack must be of type i32", fn_name);
    }

    // we have to make sure that the values are the same size
    let size1 = stack_sizes.pop().unwrap();
    let size2 = stack_sizes.pop().unwrap();

    if size1 != size2 {
        panic!("Unequal sizes for select operation: {}", fn_name);
    }

    let write_val2;
    let write_val1;
    let sp_modifier;
    if size1 == 1 {
        // pop val 2
        let val2 = &emit_read_u32("(ulong)(stack_u32+*sp-2)", "(ulong)(stack_u32)", "warp_idx");
        // pop val 1
        let val1 = &emit_read_u32("(ulong)(stack_u32+*sp-3)", "(ulong)(stack_u32)", "warp_idx");
    
        write_val1 = emit_write_u32("(ulong)(stack_u32+*sp-3)", "(ulong)(stack_u32)", val1, "warp_idx");
        write_val2 = emit_write_u32("(ulong)(stack_u32+*sp-3)", "(ulong)(stack_u32)", val2, "warp_idx");

        sp_modifier = 2;
        stack_sizes.push(1);
    } else {
        // pop val 2
        let val2 = &emit_read_u64("(ulong)(stack_u32+*sp-3)", "(ulong)(stack_u32)", "warp_idx");
        // pop val 1
        let val1 = &emit_read_u64("(ulong)(stack_u32+*sp-5)", "(ulong)(stack_u32)", "warp_idx");
    
        write_val1 = emit_write_u64("(ulong)(stack_u32+*sp-5)", "(ulong)(stack_u32)", val1, "warp_idx");
        write_val2 = emit_write_u64("(ulong)(stack_u32+*sp-5)", "(ulong)(stack_u32)", val2, "warp_idx");
   
        sp_modifier = 3;
        stack_sizes.push(2);
    }

    ret_str += &format!("\t({} != 0) ? ({}) : ({});\n",
                        c, write_val1, write_val2);

    ret_str += &format!("\t{}{};\n", "*sp -= ", sp_modifier);

    ret_str
}
