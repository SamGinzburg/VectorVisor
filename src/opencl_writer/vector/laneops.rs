use crate::opencl_writer;
use crate::opencl_writer::StackCtx;
use crate::opencl_writer::StackType;

pub fn extract_lane(
    _writer: &opencl_writer::OpenCLCWriter,
    stack_ctx: &mut StackCtx,
    lanetype: StackType,
    laneval: u8,
    _debug: bool,
) -> String {
    let reg = stack_ctx.vstack_pop(StackType::u128);

    let result_register = stack_ctx.vstack_alloc(lanetype.clone());

    let mut result = String::from("");

    result += &format!("\t{{\n");

    match lanetype {
        StackType::i32 => {
            result += &format!("\t\tuint4 *temp = &{};\n", reg);
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
        StackType::i64 => {
            result += &format!("\t\tulong2 *temp = &{};\n", reg);
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
        StackType::f32 => {
            result += &format!("\t\tfloat4 *temp = &{};\n", reg);
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
        StackType::f64 => {
            result += &format!("\t\tdouble2 *temp = &{};\n", reg);
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
        StackType::u128 => panic!("Invalid type (v128) for extract_line"),
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
            result += &format!("\t\tuint4 *temp = &{};\n", reg);
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
            result += &format!("\t\tulong2 *temp = &{};\n", reg);
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
            result += &format!("\t\tfloat4 *temp = &{};\n", reg);
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
            result += &format!("\t\tdouble2 *temp = &{};\n", reg);
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
