use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

pub enum VecUnop {
    Not,
}

pub fn vec_unop(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    unop: VecUnop,
    _debug: bool,
) -> String {
    let mut result = String::from("");
    let reg1 = stack_ctx.vstack_peak(StackType::u128, 0);

    result += &match unop {
        VecUnop::Not => {
            format!("\t{} = ~{};\n", reg1, reg1)
        }
    };

    result
}
