use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;
use wast::V128Const;

pub fn emit_v128_const(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    const_type: &wast::V128Const,
    _debug: bool,
) -> String {
    let result_register = stack_ctx.vstack_alloc(StackType::u128);
    let mut result = String::from("");

    result += &format!("\t{{\n");

    match const_type {
        V128Const::I32x4(bytes) => {
            result += &format!("\t\tint4 *temp = &{};\n", result_register);
            result += &format!("\t\t*temp = (int4)({}, {}, {}, {});\n", bytes[0], bytes[1], bytes[2], bytes[3]);
        },
        _ => {
            panic!("Unimplemented const types for emit_v128_const(...)");
        }
    }

    result += &format!("\t}}\n");

    result
}