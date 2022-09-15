use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

pub enum VecBinOp {
    Add,
    Mul,
    NotEquals,
    Shl,
    Div,
}

pub enum VecOpType {
    Float32,
    Int32,
    Int16,
}

pub enum V128BinOp {
    Xor,
    And,
}

pub fn vec_x_by_y_binop(_writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    binop: VecBinOp,
    op_type: VecOpType,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::u128);
    let reg2 = stack_ctx.vstack_pop(StackType::u128);
    let result_register = stack_ctx.vstack_alloc(StackType::u128);
    let mut result = String::from("");

    result += &format!("\t{{\n");
    match op_type {
        VecOpType::Float32 => {
            result += &format!("\t\tfloat4 *op1 = &{};\n", reg1);
            result += &format!("\t\tfloat4 *op2 = &{};\n", reg2);
            result += &format!("\t\tfloat4 *res = &{};\n", result_register);        
        },
        VecOpType::Int32 => {
            result += &format!("\t\tint4 *op1 = &{};\n", reg1);
            result += &format!("\t\tint4 *op2 = &{};\n", reg2);
            result += &format!("\t\tint4 *res = &{};\n", result_register);        
        },
        VecOpType::Int16 => {
            result += &format!("\t\tushort8 *op1 = &{};\n", reg1);
            result += &format!("\t\tushort8 *op2 = &{};\n", reg2);
            result += &format!("\t\tushort8 *res = &{};\n", result_register);        
        },
    }

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
        },
        VecBinOp::NotEquals => {
            format!(
                "\t\t*res = *op1 != *op2;\n"
            )
        },
        VecBinOp::Shl => {
            format!(
                "\t\t*res = *op1 << *op2;\n"
            )
        },
        VecBinOp::Div => {
            format!(
                "\t\t*res = *op1 / *op2;\n"
            )
        },
    };

    result += &format!("\t}}\n");

    result
}


pub fn v128_binop(_writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    binop: V128BinOp,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::u128);
    let reg2 = stack_ctx.vstack_pop(StackType::u128);
    let result_register = stack_ctx.vstack_alloc(StackType::u128);
    let mut result = String::from("");

    result += &format!("\t{{\n");
    result += &format!("\t\tulong2 *op1 = &{};\n", reg1);
    result += &format!("\t\tulong2 *op2 = &{};\n", reg2);
    result += &format!("\t\tulong2 *res = &{};\n", result_register);

    result += &match binop {
        V128BinOp::Xor => {
            format!(
                "\t\t*res = *op1 ^ *op2;\n"
            )
        },
        V128BinOp::And => {
            format!(
                "\t\t*res = *op1 & *op2;\n"
            )
        },
    };

    result += &format!("\t}}\n");

    result
}