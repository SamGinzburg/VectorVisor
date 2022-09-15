use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

pub enum SplatType {
    Float32,
    Float64,
    Int16,
    Int32,
    Int64
}

pub fn vec_splat(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    splat_type: SplatType,
    _debug: bool,
) -> String {
    let result_register = stack_ctx.vstack_alloc(StackType::u128);
    let mut result = String::from("");

    result += &format!("\t{{\n");

    match splat_type {

        SplatType::Float32 => {
            let reg = stack_ctx.vstack_pop(StackType::f32);
            result += &format!("\t\tfloat4 *temp = &{};\n", result_register);
            result += &format!(
                "\t\t*temp = (float4)({}, {}, {}, {});\n",
                reg, reg, reg, reg
            );
        },
        SplatType::Float64 => {
            let reg = stack_ctx.vstack_pop(StackType::f64);

            result += &format!("\t\tdouble2 *temp = &{};\n", result_register);
            result += &format!(
                "\t\t*temp = (double2)({}, {});\n",
                reg, reg
            );
        },
        SplatType::Int16 => {
            let reg = stack_ctx.vstack_pop(StackType::i32);

            result += &format!("\t\tushort8 *temp = &{};\n", result_register);
            result += &format!(
                "\t\t*temp = (ushort8)({}, {}, {}, {}, {}, {}, {}, {});\n",
                reg, reg, reg, reg, reg, reg, reg, reg
            );
        },
        SplatType::Int32 => {
            let reg = stack_ctx.vstack_pop(StackType::i32);

            result += &format!("\t\tuint4 *temp = &{};\n", result_register);
            result += &format!(
                "\t\t*temp = (uint4)({}, {}, {}, {});\n",
                reg, reg, reg, reg
            );
        },
        SplatType::Int64 => {
            let reg = stack_ctx.vstack_pop(StackType::i64);

            result += &format!("\t\tulong2 *temp = &{};\n", result_register);
            result += &format!(
                "\t\t*temp = (ulong2)({}, {});\n",
                reg, reg
            );
        },
    }

    result += &format!("\t}}\n");

    result
}