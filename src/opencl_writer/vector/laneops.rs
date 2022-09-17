use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

#[derive(Clone)]
pub enum ExtractLane {
    UInt8,
    Int8,
    UInt16,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
}

pub enum NarrowType {
    Int8,
    UInt8
}

pub enum NarrowLaneType {
    Int16
}

fn extract_lane_result(ty: ExtractLane) -> StackType {
    match ty {
        ExtractLane::UInt8 => StackType::i32,
        ExtractLane::Int8 => StackType::i32,
        ExtractLane::UInt16 => StackType::i32,
        ExtractLane::Int16 => StackType::i32,
        ExtractLane::Int32 => StackType::i32,
        ExtractLane::Int64 => StackType::i64,
        ExtractLane::Float32 => StackType::f32,
        ExtractLane::Float64 => StackType::f64,
    }
}

pub fn extract_lane(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    lanetype: ExtractLane,
    laneval: u8,
    _debug: bool,
) -> String {
    let reg = stack_ctx.vstack_pop(StackType::u128);

    let result_register = stack_ctx.vstack_alloc(extract_lane_result(lanetype.clone()));

    let mut result = String::from("");

    result += &format!("\t{{\n");

    match lanetype {
        ExtractLane::UInt8 => {
            result += &format!("\t\tuchar *temp = (uchar*)&{};\n", reg);
            result += &format!(
                "\t\t{} = temp[{}];\n",
                result_register, laneval
            );
        },
        ExtractLane::Int8 => {
            result += &format!("\t\tuchar *temp = (uchar*)&{};\n", reg);
            result += &format!(
                "\t\t{} = temp[{}];\n",
                result_register, laneval
            );
        },
        ExtractLane::UInt16 => {
            result += &format!("\t\tushort *temp = (ushort*)&{};\n", reg);
            result += &format!(
                "\t\t{} = temp[{}];\n",
                result_register, laneval
            );
        },
        ExtractLane::Int16 => {
            result += &format!("\t\tshort *temp = (short*)&{};\n", reg);
            result += &format!(
                "\t\t{} = temp[{}];\n",
                result_register, laneval
            );
        },
        ExtractLane::Int32 => {
            result += &format!("\t\tuint4 *temp = (uint4*)(&{});\n", reg);
            match laneval {
                0 => {
                    result += &format!(
                        "\t\t{} = (*temp).x;\n",
                        result_register
                    );
                },
                1 => {
                    result += &format!(
                        "\t\t{} = (*temp).y;\n",
                        result_register
                    );
                },
                2 => {
                    result += &format!(
                        "\t\t{} = (*temp).z;\n",
                        result_register
                    );
                },
                3 => {
                    result += &format!(
                        "\t\t{} = (*temp).w;\n",
                        result_register
                    );
                },
                _ => panic!("invalid lane idx")
            }
        },
        ExtractLane::Int64 => {
            result += &format!("\t\tulong2 *temp = (ulong2*)(&{});\n", reg);
            match laneval {
                0 => {
                    result += &format!(
                        "\t\t{} = (*temp).x;\n",
                        result_register
                    );
                },
                1 => {
                    result += &format!(
                        "\t\t{} = (*temp).y;\n",
                        result_register
                    );
                },
                _ => panic!("invalid lane idx")
            }
        },
        ExtractLane::Float32 => {
            result += &format!("\t\tfloat4 *temp = (float4*)(&{});\n", reg);
            match laneval {
                0 => {
                    result += &format!(
                        "\t\t{} = (*temp).x;\n",
                        result_register
                    );
                },
                1 => {
                    result += &format!(
                        "\t\t{} = (*temp).y;\n",
                        result_register
                    );
                },
                2 => {
                    result += &format!(
                        "\t\t{} = (*temp).z;\n",
                        result_register
                    );
                },
                3 => {
                    result += &format!(
                        "\t\t{} = (*temp).w;\n",
                        result_register
                    );
                },
                _ => panic!("invalid lane idx")
            }
        },
        ExtractLane::Float64 => {
            result += &format!("\t\tdouble2 *temp = (double2*)(&{});\n", reg);
            match laneval {
                0 => {
                    result += &format!(
                        "\t\t{} = (*temp).x;\n",
                        result_register
                    );
                },
                1 => {
                    result += &format!(
                        "\t\t{} = (*temp).y;\n",
                        result_register
                    );
                },
                _ => panic!("invalid lane idx")
            }
        },
        _ => panic!("Invalid type for extract_lane"),
    }

    result += &format!("\t}}\n");

    result
}


pub fn replace_lane(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    lanetype: StackType,
    laneval: u8,
    _debug: bool,
) -> String {
    let reg = stack_ctx.vstack_peak(StackType::u128, 0);
    let newval = match lanetype {
        StackType::i32 => stack_ctx.vstack_pop(StackType::i32),
        StackType::i64 => stack_ctx.vstack_pop(StackType::i64),
        StackType::f32 => stack_ctx.vstack_pop(StackType::f32),
        StackType::f64 => stack_ctx.vstack_pop(StackType::f64),
        StackType::u128 => panic!("lanetype u128 for replace_lane, should not happen"),
    };

    let mut result = String::from("");

    result += &format!("\t{{\n");

    match lanetype {
        StackType::i32 => {
            result += &format!("\t\tuint4 *temp = (uint4*)(&{});\n", reg);
            match laneval {
                0 => {
                    result += &format!(
                        "\t\t(*temp).x = {};\n",
                        newval
                    );
                },
                1 => {
                    result += &format!(
                        "\t\t(*temp).y = {};\n",
                        newval
                    );
                },
                2 => {
                    result += &format!(
                        "\t\t(*temp).z = {};\n",
                        newval
                    );
                },
                3 => {
                    result += &format!(
                        "\t\t(*temp).w = {};\n",
                        newval
                    );
                },
                _ => panic!("invalid lane idx")
            }
        },
        StackType::i64 => {
            result += &format!("\t\tulong2 *temp = (ulong2*)(&{});\n", reg);
            match laneval {
                0 => {
                    result += &format!(
                        "\t\t(*temp).x = {};\n",
                        newval
                    );
                },
                1 => {
                    result += &format!(
                        "\t\t(*temp).y = {};\n",
                        newval
                    );
                },
                _ => panic!("invalid lane idx")
            }
        },
        StackType::f32 => {
            result += &format!("\t\tfloat4 *temp = (float4*)(&{});\n", reg);
            match laneval {
                0 => {
                    result += &format!(
                        "\t\t(*temp).x = {};\n",
                        newval
                    );
                },
                1 => {
                    result += &format!(
                        "\t\t(*temp).y = {};\n",
                        newval
                    );
                },
                2 => {
                    result += &format!(
                        "\t\t(*temp).z = {};\n",
                        newval
                    );
                },
                3 => {
                    result += &format!(
                        "\t\t(*temp).w = {};\n",
                        newval
                    );
                },
                _ => panic!("invalid lane idx")
            }
        },
        StackType::f64 => {
            result += &format!("\t\tdouble2 *temp = (double2*)(&{});\n", reg);
            match laneval {
                0 => {
                    result += &format!(
                        "\t\t(*temp).x = {};\n",
                        newval
                    );
                },
                1 => {
                    result += &format!(
                        "\t\t(*temp).y = {};\n",
                        newval
                    );
                },
                _ => panic!("invalid lane idx")
            }
        },
        StackType::u128 => panic!("Invalid type (v128) for extract_line"),
    }

    result += &format!("\t}}\n");

    result
}

pub fn narrow(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    narrow_type: NarrowType,
    lane_type: NarrowLaneType,
    _debug: bool,
) -> String {
    let mut result = String::from("");
    let reg1 = stack_ctx.vstack_pop(StackType::u128);
    let reg2 = stack_ctx.vstack_pop(StackType::u128);
    let result_register = stack_ctx.vstack_alloc(StackType::u128);

    result += &format!("\t{{\n");

    match (narrow_type, lane_type)  {
        (NarrowType::Int8, NarrowLaneType::Int16) => {
            result += &format!("\t\tchar16 temp = (char16)(0);\n");
            result += &format!("\t\tchar *temp_ptr = (char*)(&temp);\n");
            result += &format!("\t\tshort *temp1 = (short*)(&{});\n", reg1);
            result += &format!("\t\tshort *temp2 = (short*)(&{});\n", reg2);

            for idx in 0..8 {
                result += &format!("\t\ttemp_ptr[{}] = (char)(temp1[{}]);\n", idx, idx);
                result += &format!("\t\ttemp_ptr[{}+8] = (char)(temp2[{}]);\n", idx, idx);
            }

            result += &format!("\t\t{} = temp;\n", result_register);
        },
        (NarrowType::UInt8, NarrowLaneType::Int16) => {
            result += &format!("\t\tuchar16 temp = (uchar16)(0);\n");
            result += &format!("\t\tuchar *temp_ptr = (uchar*)(&temp);\n");
            result += &format!("\t\tushort *temp1 = (ushort*)(&{});\n", reg1);
            result += &format!("\t\tushort *temp2 = (ushort*)(&{});\n", reg2);

            for idx in 0..8 {
                result += &format!("\t\ttemp_ptr[{}] = (uchar)(temp1[{}]);\n", idx, idx);
                result += &format!("\t\ttemp_ptr[{}+8] = (uchar)(temp2[{}]);\n", idx, idx);
            }
        },
    }

    result += &format!("\t\t{} = as_ulong2(temp);\n", result_register);
    result += &format!("\t}}\n");

    result
}
