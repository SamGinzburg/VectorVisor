use crate::opencl_writer;
use crate::opencl_writer::ValType;
use std::collections::HashMap;

pub fn emit_local_get(writer: &opencl_writer::OpenCLCWriter, id: &str, offsets: &HashMap<&str, u32>, type_info: &HashMap<&str, ValType>, debug: bool) -> String {
    let offset = offsets.get(id).unwrap();
    let t = type_info.get(id).unwrap();

    // stack_frames[*sfp - 1] start of stack frame
    match t {
        wast::ValType::I32 => {
            format!("\t{}\n\t{}\n",
                    format!("write_u32((ulong)(stack_u32+*sp), read_u32((ulong)(stack_u32+{}+read_u32((ulong)(stack_frames+*sfp), warp_idx)), warp_idx), warp_idx);", offset),
                    "*sp += 1;")
        },
        wast::ValType::I64 => {
            format!("\t{}\n\t{}\n",
                    format!("write_u64((ulong)(stack_u32+*sp), read_u64((ulong)(stack_u32+{}+read_u64((ulong)(stack_frames+*sfp), warp_idx)), warp_idx), warp_idx);", offset),
                    "*sp += 2;")
        },
        wast::ValType::F32 => {
            format!("\t{}\n\t{}\n",
                    format!("write_u32((ulong)(stack_u32+*sp), read_u32((ulong)(stack_u32+{}+read_u32((ulong)(stack_frames+*sfp), warp_idx)), warp_idx), warp_idx);", offset),
                    "*sp += 1;")
        },
        wast::ValType::F64 => {
            format!("\t{}\n\t{}\n",
                    format!("write_u64((ulong)(stack_u32+*sp), read_u64((ulong)(stack_u32+{}+read_u64((ulong)(stack_frames+*sfp), warp_idx)), warp_idx), warp_idx);", offset),
                    "*sp += 2;")
        },
        _ => panic!("emit_local_set type not handled")
    }
}

pub fn emit_local_set(writer: &opencl_writer::OpenCLCWriter, id: &str, offsets: &HashMap<&str, u32>, type_info: &HashMap<&str, ValType>, debug: bool) -> String {
    let offset = offsets.get(id).unwrap();
    let t = type_info.get(id).unwrap();
    dbg!(id);
    dbg!(offset);
    dbg!(t);
    match t {
        wast::ValType::I32 => {
            format!("\t{}\n",
                    format!("write_u32((ulong)(stack_u32+{}+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                       read_u32((ulong)(stack_u32+*sp-1), warp_idx), warp_idx);",
                            offset))
        },
        wast::ValType::I64 => {
            format!("\t{}\n",
                    format!("write_u64((ulong)(stack_u32+{}+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                    read_u64((ulong)(stack_u32+*sp-2), warp_idx), warp_idx);",
                            offset))
        },
        wast::ValType::F32 => {
            format!("\t{}\n",
                    format!("write_u32((ulong)(stack_u32+{}+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                       read_u32((ulong)(stack_u32+*sp-1), warp_idx), warp_idx);",
                            offset))
        },
        wast::ValType::F64 => {
            format!("\t{}\n",
                    format!("write_u64((ulong)(stack_u32+{}+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                    read_u64((ulong)(stack_u32+*sp-2), warp_idx), warp_idx);",
                            offset))
        },
        _ => panic!("emit_local_set type not handled")
    }
}

pub fn emit_local_tee(writer: &opencl_writer::OpenCLCWriter, id: &str, offsets: &HashMap<&str, u32>, type_info: &HashMap<&str, ValType>, debug: bool) -> String {
    /*
     * peak the top of the stack, push the most recent value again
     * call local.set [x]
     */
    let offset = offsets.get(id).unwrap();
    let t = type_info.get(id).unwrap();
    dbg!(id);
    dbg!(offset);
    dbg!(t);
    match t {
        wast::ValType::I32 => {
            format!("\t{}\n\t{}\n{}",
                    "write_u32((ulong)(stack_u32+*sp), read_u32((ulong)(stack_u32+*sp-1), warp_idx), warp_idx);",
                    "*sp += 1;",
                    format!("{}", emit_local_set(writer, id, offsets, type_info, debug)))
        },
        wast::ValType::I64 => {
            format!("\t{}\n{}",
                    format!("{}\n\t{}",
                            "write_u64((ulong)(stack_u32+*sp), read_u64((ulong)(stack_u32+*sp-2), warp_idx), warp_idx);",
                            "*sp += 2;"),
                    format!("{}", emit_local_set(writer, id, offsets, type_info, debug)))
        },
        wast::ValType::F32 => {
            format!("\t{}\n\t{}\n{}",
                    "write_u32((ulong)(stack_u32+*sp), read_u32((ulong)(stack_u32+*sp-1), warp_idx), warp_idx);",
                    "*sp += 1;",
                    format!("{}", emit_local_set(writer, id, offsets, type_info, debug)))
        },
        wast::ValType::F64 => {
            format!("\t{}\n{}",
                    format!("{}\n\t{}",
                            "write_u64((ulong)(stack_u32+*sp), read_u64((ulong)(stack_u32+*sp-2), warp_idx), warp_idx);",
                            "*sp += 2;"),
                    format!("{}", emit_local_set(writer, id, offsets, type_info, debug)))
        },
        _ => panic!("emit_local_tee type not handled")
    }
}

pub fn emit_local(writer: &opencl_writer::OpenCLCWriter, local: &wast::Local, offsets: &HashMap<&str, u32>, debug: bool) -> String {
    /*
     * When emitting locals we know we have access to the global stack.
     * We zero-init all values.
     * 
     */
    match local.ty {
        wast::ValType::I32 => {
            let local_id = match local.id {
                Some(id) => id.name(),
                None => panic!("Unexpected local without identifier"),
            };
            String::from(format!("\t{}\n\t{}\n\t{}\n",
                            format!("/* local id: {} */", local_id),
                            "write_u32((ulong)(stack_u32+*sp), (uint)0, warp_idx);",
                            "*sp += 1;"))
        },
        wast::ValType::I64 => {
            let local_id = match local.id {
                Some(id) => id.name(),
                None => panic!("Unexpected local without identifier"),
            };
            String::from(format!("\t{}\n\t{}\n\t{}\n",
                            format!("/* local id: {} */", local_id),
                            "write_u64((ulong)(stack_u32+*sp), (ulong)0, warp_idx);",
                            "*sp += 2;"))
        },
        wast::ValType::F32 => {
            let local_id = match local.id {
                Some(id) => id.name(),
                None => panic!("Unexpected local without identifier"),
            };
            String::from(format!("\t{}\n\t{}\n\t{}\n",
                            format!("/* local id: {} */", local_id),
                            "write_u32((ulong)(stack_u32+*sp), (uint)0, warp_idx);",
                            "*sp += 1;"))
        },
        wast::ValType::F64 => {
            let local_id = match local.id {
                Some(id) => id.name(),
                None => panic!("Unexpected local without identifier"),
            };
            String::from(format!("\t{}\n\t{}\n\t{}\n",
                            format!("/* local id: {} */", local_id),
                            "write_u64((ulong)(stack_u32+*sp), (ulong)0, warp_idx);",
                            "*sp += 2;"))
        },
        _ => panic!(),
    }
}

pub fn emit_i32_const(writer: &opencl_writer::OpenCLCWriter, val: &i32, debug: bool) -> String {
    format!("\t{}{}, warp_idx);\n\t{}\n",
            "write_u32((ulong)(stack_u32+*sp), (uint)",
            val,
            "*sp += 1;")
}

pub fn emit_i64_const(writer: &opencl_writer::OpenCLCWriter, val: &i64, debug: bool) -> String {
    format!("\t{}{}, warp_idx);\n\t{}\n",
            "write_u64((ulong)(stack_u32+*sp), (ulong)",
            val,
            "*sp += 2;")
}