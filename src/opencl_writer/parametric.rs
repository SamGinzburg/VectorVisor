use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

pub fn emit_select(_writer: &opencl_writer::OpenCLCWriter, stack_ctx: &mut StackCtx, fn_name: &str, _debug: bool) -> String {
    let mut ret_str = String::from("");

    let c = stack_ctx.vstack_pop(StackType::i32);

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

    ret_str += &format!("\t({} != 0) ? ({}) : ({});\n",
                        c, write_val1, write_val2);

    ret_str
}
