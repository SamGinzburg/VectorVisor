use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

pub fn f32x4_splat(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    _debug: bool,
) -> String {
    let reg = stack_ctx.vstack_pop(StackType::f32);
    let result_register = stack_ctx.vstack_alloc(StackType::u128);
    let mut result = String::from("");

    result += &format!("\t{{\n");
    result += &format!("\t\tfloat4 *temp = &{};\n", result_register);
    result += &format!(
        "\t\t*temp = (float4)({}, {}, {}, {});\n",
        reg, reg, reg, reg
    );
    result += &format!("\t}}\n");

    result
}
