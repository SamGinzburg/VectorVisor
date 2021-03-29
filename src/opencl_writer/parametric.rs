use crate::opencl_writer;
use crate::opencl_writer::mem_interleave::emit_read_u32;
use crate::opencl_writer::mem_interleave::emit_write_u32;
use crate::opencl_writer::mem_interleave::emit_read_u64;
use crate::opencl_writer::mem_interleave::emit_write_u64;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

pub fn emit_select(writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, stack_sizes: &mut Vec<u32>, fn_name: &str, debug: bool) -> String {
    let mut ret_str = String::from("");

    let c = stack_ctx.vstack_pop(StackType::i32);

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
    if size1 == 1 {
        let val2 = stack_ctx.vstack_pop(StackType::i32);
        let val1 = stack_ctx.vstack_pop(StackType::i32);
        let result_register = stack_ctx.vstack_alloc(StackType::i32);

        write_val1 = format!("{} = {}", result_register, val1);
        write_val2 = format!("{} = {}", result_register, val2);

        stack_sizes.push(1);
    } else {
        let val2 = stack_ctx.vstack_pop(StackType::i64);
        let val1 = stack_ctx.vstack_pop(StackType::i64);
        let result_register = stack_ctx.vstack_alloc(StackType::i64);

        write_val1 = format!("{} = {}", result_register, val1);
        write_val2 = format!("{} = {}", result_register, val2);

        stack_sizes.push(2);
    }

    ret_str += &format!("\t({} != 0) ? ({}) : ({});\n",
                        c, write_val1, write_val2);

    ret_str
}
