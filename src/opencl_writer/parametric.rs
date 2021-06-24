use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

pub fn emit_select(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, stack_sizes: &mut Vec<u32>, fn_name: &str, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let c = stack_ctx.vstack_pop(StackType::i32);

    if stack_sizes.pop().unwrap() != 1 {
        panic!("select in fn: {}, the top of the stack must be of type i32", fn_name);
    }

    // we have to make sure that the values are the same size
    stack_sizes.pop().unwrap();
    stack_sizes.pop().unwrap();

    let type1 = stack_ctx.vstack_peak_type(0);
    let type2 = stack_ctx.vstack_peak_type(1);

    if type1 != type2 {
        panic!("Unequal sizes for select operation: {}, StackCtx: {:?}", fn_name, stack_ctx);
    }

    let val2 = stack_ctx.vstack_pop(type1.clone());
    let val1 = stack_ctx.vstack_pop(type1.clone());
    let result_register = stack_ctx.vstack_alloc(type1.clone());

    let write_val1 = format!("{} = {}", result_register, val1);
    let write_val2 = format!("{} = {}", result_register, val2);

    match type1.clone() {
        StackType::i32 | StackType::f32 => stack_sizes.push(1),
        StackType::i64 | StackType::f64 => stack_sizes.push(2),
    }

    ret_str += &format!("\t({} != 0) ? ({}) : ({});\n",
                        c, write_val1, write_val2);

    ret_str
}
