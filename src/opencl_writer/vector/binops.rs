use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

pub enum VecBinOp {
    Add,
    Sub,
    Mul,
    Div,
    // Relops merged here for convenience
    GtS,
    GtU,
    GeU,
    GeS,
    LtU,
    LtS,
    LeS,
    LeU,
    NotEquals,
    Equals,
    MaxU,
}

pub enum VecByScalarOp {
    ShrU,
    ShrS,
    Shl,
}

#[derive(Debug)]
pub enum VecOpType {
    Int64,
    UInt64,
    Float32,
    Int32,
    UInt32,
    Int16,
    UInt16,
    Int8,
    UInt8,
}

pub enum V128BinOp {
    Xor,
    And,
    Or,
}

// Shr/Shl
pub fn vec_x_by_scalar_binop(_writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    binop: VecByScalarOp,
    op_type: VecOpType,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_peak(StackType::u128, 0);
    let reg2 = stack_ctx.vstack_pop(StackType::i32);

    let mut result = String::from("");

    result += &format!("\t{{\n");
    match op_type {
        VecOpType::Int64 => {
            result += &format!("\t\tlong2 *op1 = (long2*)(&{});\n", reg1);
            result += &format!("\t\tlong2 op2 = (long2)({});\n", reg2);
        },
        VecOpType::UInt64 => {
            result += &format!("\t\tulong2 *op1 = (ulong2*)(&{});\n", reg1);
            result += &format!("\t\tulong2 op2 = (ulong2)({});\n", reg2);
        },
        VecOpType::Int32 => {
            result += &format!("\t\tint4 *op1 = (int4*)(&{});\n", reg1);
            result += &format!("\t\tint4 op2 = (int4)({});\n", reg2);
        },
        VecOpType::UInt32 => {
            result += &format!("\t\tuint4 *op1 = (uint4*)(&{});\n", reg1);
            result += &format!("\t\tuint4 op2 = (uint4)({});\n", reg2);
        },
        VecOpType::Int16 => {
            result += &format!("\t\tshort8 *op1 = (short8*)(&{});\n", reg1);
            result += &format!("\t\tshort8 op2 = (short8)({});\n", reg2);
        },
        VecOpType::UInt16 => {
            result += &format!("\t\tushort8 *op1 = (ushort8*)(&{});\n", reg1);
            result += &format!("\t\tushort8 op2 = (ushort8)({});\n", reg2);
        },
        VecOpType::Int8 => {
            result += &format!("\t\tchar16 *op1 = (char16*)(&{});\n", reg1);
            result += &format!("\t\tchar16 op2 = (char16)({});\n", reg2);
        },
        VecOpType::UInt8 => {
            result += &format!("\t\tuchar16 *op1 = (uchar16*)(&{});\n", reg1);
            result += &format!("\t\tuchar16 op2 = (uchar16)({});\n", reg2);
        },
        _ => panic!("Type {:?}, not implemented for vec_x_by_scalar_binop", op_type),
    }

    result += &match binop {
        VecByScalarOp::ShrU | VecByScalarOp::ShrS => {
            format!(
                "\t\t*op1 = *op1 >> op2;\n"
            )
        },
        VecByScalarOp::Shl => {
            format!(
                "\t\t*op1 = *op1 << op2;\n"
            )
        },
    };

    result += &format!("\t}}\n");

    result
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
    let typecast = match op_type {
        VecOpType::UInt64 => {
            result += &format!("\t\tulong2 *op1 = (ulong2*)&{};\n", reg1);
            result += &format!("\t\tulong2 *op2 = (ulong2*)&{};\n", reg2);
            result += &format!("\t\tulong2 *res = (ulong2*)&{};\n", result_register);
            format!("ulong2")
        },
        VecOpType::Int64 => {
            result += &format!("\t\tlong2 *op1 = (long2*)&{};\n", reg1);
            result += &format!("\t\tlong2 *op2 = (long2*)&{};\n", reg2);
            result += &format!("\t\tlong2 *res = (long2*)&{};\n", result_register);
            format!("long2")
        },
        VecOpType::Float32 => {
            result += &format!("\t\tfloat4 *op1 = (float4*)&{};\n", reg1);
            result += &format!("\t\tfloat4 *op2 = (float4*)&{};\n", reg2);
            result += &format!("\t\tfloat4 *res = (float4*)&{};\n", result_register);
            format!("float4")
        },
        VecOpType::Int32 => {
            result += &format!("\t\tint4 *op1 = (int4*)&{};\n", reg1);
            result += &format!("\t\tint4 *op2 = (int4*)&{};\n", reg2);
            result += &format!("\t\tint4 *res = (int4*)&{};\n", result_register);
            format!("int4")
        },
        VecOpType::UInt32 => {
            result += &format!("\t\tuint4 *op1 = (uint4*)&{};\n", reg1);
            result += &format!("\t\tuint4 *op2 = (uint4*)&{};\n", reg2);
            result += &format!("\t\tuint4 *res = (uint4*)&{};\n", result_register);
            format!("uint4")
        },
        VecOpType::Int16 => {
            result += &format!("\t\tshort8 *op1 = (short8*)&{};\n", reg1);
            result += &format!("\t\tshort8 *op2 = (short8*)&{};\n", reg2);
            result += &format!("\t\tshort8 *res = (short8*)&{};\n", result_register);
            format!("short8")
        },
        VecOpType::UInt16 => {
            result += &format!("\t\tushort8 *op1 = (ushort8*)&{};\n", reg1);
            result += &format!("\t\tushort8 *op2 = (ushort8*)&{};\n", reg2);
            result += &format!("\t\tushort8 *res = (ushort8*)&{};\n", result_register);
            format!("ushort8")
        },
        VecOpType::Int8 => {
            result += &format!("\t\tchar16 *op1 = (char16*)&{};\n", reg1);
            result += &format!("\t\tchar16 *op2 = (char16*)&{};\n", reg2);
            result += &format!("\t\tchar16 *res = (char16*)&{};\n", result_register);
            format!("int8")
        },
        VecOpType::UInt8 => {
            result += &format!("\t\tuchar16 *op1 = (uchar16*)&{};\n", reg1);
            result += &format!("\t\tuchar16 *op2 = (uchar16*)&{};\n", reg2);
            result += &format!("\t\tuchar16 *res = (uchar16*)&{};\n", result_register);
            format!("uint8")
        },
    };

    result += &match binop {
        VecBinOp::Add => {
            format!(
                "\t\t*res = *op1 + *op2;\n"
            )
        },
        VecBinOp::Sub => {
            format!(
                "\t\t*res = *op1 - *op2;\n"
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
        VecBinOp::Equals => {
            format!(
                "\t\t*res = *op1 == *op2;\n"
            )
        },
        VecBinOp::Div => {
            format!(
                "\t\t*res = *op1 / *op2;\n"
            )
        },
        VecBinOp::MaxU => {
            format!(
                "\t\t*res = max(*op1, *op2);\n"
            )
        },
        VecBinOp::GtS | VecBinOp::GtU => {
            format!(
                "\t\t*res = *op1 > *op2;\n"
            )
        },
        VecBinOp::GeU | VecBinOp::GeS => {
            format!(
                "\t\t*res = ({})(*op1 >= *op2);\n",
                typecast
            )
        },
        VecBinOp::LtU | VecBinOp::LtS => {
            format!(
                "\t\t*res = *op1 < *op2;\n"
            )
        },
        VecBinOp::LeU | VecBinOp::LeS => {
            format!(
                "\t\t*res = *op1 <= *op2;\n"
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
        V128BinOp::Or => {
            format!(
                "\t\t*res = *op1 | *op2;\n"
            )
        },
    };

    result += &format!("\t}}\n");

    result
}
