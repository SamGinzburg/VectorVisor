use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

pub enum VecBinOp {
    Add,
    Mul
}

pub fn f32x4_binop(_writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    binop: VecBinOp,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::u128);
    let reg2 = stack_ctx.vstack_pop(StackType::u128);
    let result_register = stack_ctx.vstack_alloc(StackType::u128);
    let mut result = String::from("");

    result += &format!("\t{{\n");
    result += &format!("\t\tfloat4 *op1 = &{};\n", reg1);
    result += &format!("\t\tfloat4 *op2 = &{};\n", reg2);
    result += &format!("\t\tfloat4 *res = &{};\n", result_register);

    result += &match binop {
        VecBinOp::Add => {
            format!(
                "\t\t*res = *op1 + *op2;\n"
            )
        },
        VecBinOp::Mul => {
            format!(
                "\t\t*res = *op1 * *op2;\n"
            )
        }
    };

    result += &format!("\t}}\n");

    result
}