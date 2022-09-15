use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;
use wast::core::*;

pub fn i8x16shuffle(_writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    lanes: &I8x16Shuffle,
    _debug: bool,
) -> String {
    let reg1 = stack_ctx.vstack_pop(StackType::u128);
    let reg2 = stack_ctx.vstack_pop(StackType::u128);
    let result_register = stack_ctx.vstack_alloc(StackType::u128);
    let mut result = String::from("");    
    let mut mask = String::from("");    

    for value in lanes.lanes {
        mask += &format!("{},", value);
    }

    result += &format!("\t{} = (ulong2)shuffle2((uchar16){}, (uchar16){}, (uchar16)({}));\n", result_register, reg1, reg2, &mask[0..mask.len()-1]);        

    result
}