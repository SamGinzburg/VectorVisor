use wast::Wat;
use wast::parser::{self, ParseBuffer};
use wast::ModuleKind::{Text, Binary};
use wast::ValType;
use wast::Instruction;

use std::path::Path;
use std::fmt;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::collections::HashMap;

pub struct OpenCLCWriter<'a> {
    types: Vec<wast::Type<'a>>,
    imports: Vec<wast::Import<'a>>,
    func_map: HashMap<&'a str, wast::Func<'a>>,
    tables: Vec<wast::Table<'a>>,
    memory: Vec<wast::Memory<'a>>,
    globals: Vec<wast::Global<'a>>,
    exports: Vec<wast::Export<'a>>,
    start: Vec<wast::Index<'a>>,
    elements: Vec<wast::Elem<'a>>,
    data: Vec<wast::Data<'a>>,
    parse_buffer: &'a ParseBuffer<'a>,
}

impl<'a> OpenCLCWriter<'_> {
    pub fn new(pb: &'a ParseBuffer) -> OpenCLCWriter<'a> {
        OpenCLCWriter {
            types: vec!(),
            imports: vec!(),
            func_map: HashMap::new(),
            tables: vec!(),
            memory: vec!(),
            globals: vec!(),
            exports: vec!(),
            start: vec!(),
            elements: vec!(),
            data: vec!(),
            parse_buffer: pb
        }
    }

    pub fn parse_file(&mut self) -> Result<bool, String> {
        let module = parser::parse::<Wat>(self.parse_buffer).unwrap();

        match module.module.kind {
            Text(t) => {
                for item in t {
                    match item {
                        wast::ModuleField::Type(t) => self.types.push(t),
                        wast::ModuleField::Import(i) => self.imports.push(i),
                        wast::ModuleField::Func(f) => {
                            match f.id {
                                Some(f_id) => self.func_map.insert(f_id.name(), f),
                                None => continue,
                            };
                        },
                        wast::ModuleField::Table(table) => self.tables.push(table),
                        wast::ModuleField::Memory(mem) => self.memory.push(mem),
                        wast::ModuleField::Global(global) => self.globals.push(global),
                        wast::ModuleField::Export(exp) => self.exports.push(exp),
                        wast::ModuleField::Start(s) => self.start.push(s),
                        wast::ModuleField::Elem(e) => self.elements.push(e),
                        wast::ModuleField::Data(d) => self.data.push(d),
                        _ => println!("unknown WASM operation found: {:?}", item)
                    }
                }
            },
            Binary(_) => println!("binary")
        }
        Ok(true)
    }

    fn emit_local(&self, local: &wast::Local, offsets: &HashMap<&str, u32>, debug: bool) -> String {
        /*
         * When emitting locals we know we have access to the global stack.
         * We zero-init all values.
         * 
         * We have an inefficient stack layout right now... we will fix later if it is needed
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
                                "stack_u32[*sp] = 0;",
                                "*sp += 1;"))
            },
            wast::ValType::I64 => {
                let local_id = match local.id {
                    Some(id) => id.name(),
                    None => panic!("Unexpected local without identifier"),
                };
                String::from(format!("\t{}\n\t{}\n\t{}\n",
                                format!("/* local id: {} */", local_id),
                                "*((ulong *)stack_u32+*sp) = 0;",
                                "*sp += 2;"))
            },
            wast::ValType::F32 => {
                let local_id = match local.id {
                    Some(id) => id.name(),
                    None => panic!("Unexpected local without identifier"),
                };
                String::from(format!("\t{}\n\t{}\n\t{}\n",
                                format!("/* local id: {} */", local_id),
                                "stack_u32[*sp] = 0;",
                                "*sp += 1;"))
            },
            wast::ValType::F64 => {
                let local_id = match local.id {
                    Some(id) => id.name(),
                    None => panic!("Unexpected local without identifier"),
                };
                String::from(format!("\t{}\n\t{}\n\t{}\n",
                                format!("/* local id: {} */", local_id),
                                "*((ulong *)stack_u32+*sp) = 0;",
                                "*sp += 2;"))
            },
            _ => panic!(),
        }
    }

    fn emit_i32_const(&self, val: &i32, debug: bool) -> String {
        format!("\tstack_u32[*sp] = (uint){};\n\t*sp += 1;\n", val)
    }

    fn emit_i64_const(&self, val: &i64, debug: bool) -> String {
        format!("\t{}{};\n\t*sp += 2;\n",
                "*(ulong *)(stack_u32+*sp) = (ulong)",
                val)
    }

    fn get_size_valtype(&self, t: &ValType) -> u32 {
        match t {
            wast::ValType::I32 => 1,
            wast::ValType::F32 => 1,
            wast::ValType::I64 => 2,
            wast::ValType::F64 => 2,
            _ => panic!("Unknown local size found"),
        }
    }

    fn emit_local_get(&self, id: &str, offsets: &HashMap<&str, u32>, type_info: &HashMap<&str, ValType>, debug: bool) -> String {
        let offset = offsets.get(id).unwrap();
        let t = type_info.get(id).unwrap();

        // stack_frames[*sfp - 1] start of stack frame
        match t {
            wast::ValType::I32 => {
                format!("\t{}\n\t{}\n",
                        format!("stack_u32[*sp] = *(uint *)(stack_u32 + stack_frames[*sfp] + {});", offset),
                        "*sp += 1;")
            },
            wast::ValType::I64 => {
                format!("\t{}\n\t{}\n",
                        format!("*((ulong *)(stack_u32+*sp)) = *(ulong *)(stack_u32 + stack_frames[*sfp] + {});", offset),
                        "*sp += 2;")
            },
            wast::ValType::F32 => {
                format!("\t{}\n",
                        format!("stack_u32[*sp] = *(uint *)(stack_u32 + stack_frames[*sfp] + {});", offset))
            },
            wast::ValType::F64 => {
                format!("\t{}\n\t{}\n",
                        format!("*((ulong *)(stack_u32+*sp)) = *(ulong *)(stack_u32 + stack_frames[*sfp] + {});", offset),
                        "*sp += 2;")
            },
            _ => panic!("emit_local_set type not handled")
        }
    }

    fn emit_local_set(&self, id: &str, offsets: &HashMap<&str, u32>, type_info: &HashMap<&str, ValType>, debug: bool) -> String {
        let offset = offsets.get(id).unwrap();
        let t = type_info.get(id).unwrap();
        dbg!(id);
        dbg!(offset);
        dbg!(t);
        match t {
            wast::ValType::I32 => {
                format!("\t{}\n",
                        format!("*(uint *)(stack_u32 + stack_frames[*sfp] + {}) = stack_u32[*sp - 1];", offset))
            },
            wast::ValType::I64 => {
                format!("\t{}\n",
                        format!("*(ulong *)(stack_u32 + stack_frames[*sfp] + {}) = *(ulong *)(stack_u32+*sp-2);", offset))
            },
            wast::ValType::F32 => {
                format!("\t{}\n",
                        format!("*(uint *)(stack_u32 + stack_frames[*sfp] + {}) = stack_u32[*sp - 1];", offset))
            },
            wast::ValType::F64 => {
                format!("\t{}\n",
                        format!("*(ulong *)(stack_u32 + stack_frames[*sfp] + {}) = *(ulong *)(stack_u32+*sp-2);", offset))
            },
            _ => panic!("emit_local_set type not handled")
        }
    }

    fn emit_local_tee(&self, id: &str, offsets: &HashMap<&str, u32>, type_info: &HashMap<&str, ValType>, debug: bool) -> String {
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
                        "stack_u32[*sp] = stack_u32[*sp - 1];",
                        "*sp += 1;",
                        format!("{}", self.emit_local_set(id, offsets, type_info, debug)))
            },
            wast::ValType::I64 => {
                format!("\t{}\n{}",
                        format!("{};\n\t{}", "*(ulong *)(stack_u32+*sp) = *(ulong *)(stack_u32+*sp-2)", "*sp += 2;"),
                        format!("{}", self.emit_local_set(id, offsets, type_info, debug)))
            },
            wast::ValType::F32 => {
                format!("\t{}\n\t{}\n{}",
                        "stack_u32[*sp] = stack_u32[*sp - 1];",
                        "*sp += 1;",
                        format!("{}", self.emit_local_set(id, offsets, type_info, debug)))
            },
            wast::ValType::F64 => {
                format!("\t{}\n{}",
                        format!("{};\n\t{}", "*(ulong *)(stack_u32+*sp) = *(ulong *)(stack_u32+*sp-2)", "*sp += 2;"),
                        format!("{}", self.emit_local_set(id, offsets, type_info, debug)))
            },
            _ => panic!("emit_local_tee type not handled")
        }
    }

    // binops have both values popped off the stack
    fn emit_i32_add(&self, debug: bool) -> String {
        format!("\t{}\n\t{}\n",
                "stack_u32[*sp-2] = stack_u32[*sp - 1] + stack_u32[*sp - 2];",
                "*sp -= 1;")
    }

    fn emit_i64_add(&self, debug: bool) -> String {
        format!("\t{}\n\t{}\n",
                "*(ulong*)(stack_u32+*sp-4) = *(ulong*)(stack_u32+*sp-2) + *(ulong*)(stack_u32+*sp-4);",
                "*sp -= 2")
    }

    fn emit_i32_lt_s(&self, debug: bool) -> String {
        format!("\t{}\n\t{}\n",
                "stack_u32[*sp-1] = (uint)((int)stack_u32[*sp - 1]) < ((int)stack_u32[*sp - 2]);",
                // move sp back by 1
                "*sp -= 1;")
    }

    fn emit_i64_lt_s(&self, debug: bool) -> String {
        format!("\t{}\n\t{}\n",
                "stack_u32[*sp-1] = stack_u32[*sp - 2] < stack_u32[*sp - 2];",
                // move sp back by 1
                "*sp -= 1;")
    }

    fn emit_fn_call(&self, idx: wast::Index, call_ret_map: &mut HashMap<&str, u32>, call_ret_idx: &mut u32, debug: bool) -> String {
        let id = match idx {
            wast::Index::Id(id) => id.name(),
            _ => panic!("Unable to get Id for function call!"),
        };

        dbg!(&self.func_map);
        // if the func has calling parameters, set those up
        // on the newly formed stack as well
        let func_type_signature = &self.func_map.get(id).unwrap().ty;
        let mut offset = 0;
        for parameter in func_type_signature.clone().inline.unwrap().params.to_vec() {
            dbg!(parameter);
            match parameter {
                (Some(id), _, t) => {
                    dbg!(id);
                    dbg!(t);
                    offset += self.get_size_valtype(&t);
                },
                _ => panic!("Unhandled parameter type")
            }
        }

        // for each function call, map the call to an index
        // we use this index later on to return back to the instruction after the call
        
        let ret_label: &'static str = Box::leak(format!("ret_from_{}_{}", id, call_ret_idx).into_boxed_str());
        call_ret_map.insert(ret_label, *call_ret_idx);

        // get the return type of the function
        let return_size = self.get_size_valtype(&func_type_signature.clone().inline.unwrap().results[0]);

        let result = if offset > 0 {
            format!("\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n{}\n\t{}\n",
            // move the stack pointer back by the offset required by calling parameters
            // the sp should point at the start of the arguments for the function
            format!("*sp -= {};", offset),
            // increment stack frame pointer
            "*sfp += 1;",
            // save the current stack pointer for unwinding later
            "stack_frames[*sfp] = *sp;",
            // save the callee return stub number
            format!("call_stack[*sfp] = {};", *call_ret_idx),
            // setup calling parameters for function
            format!("goto {};", id),
            format!("call_return_stub_{}:", *call_ret_idx),
            format!("*sp += {};", return_size))
        } else {
            format!("\t{}\n\t{}\n\t{}\n{}\n",
                    // increment stack frame pointer
                    "*sfp += 1;",
                    // save the current stack pointer for unwinding later
                    "stack_frames[*sfp] = *sp;",
                    // save the callee return stub number
                    // setup calling parameters for function
                    format!("goto {};", id),
                    format!("call_return_stub_{}:", 0))
        };
        *call_ret_idx += 1;

        result
    }

    // TODO: this needs to take the function type into account
    fn function_unwind(&self, func_ret_info: &Option<wast::FunctionType>, debug: bool) -> String {
        let mut final_str = String::from("");

        let results: Vec<wast::ValType> = match func_ret_info {
            Some(s) => (*s.results).to_vec(),
            None => vec![]
        };
        
        final_str += &format!("\t{}\n", "/* function unwind */");
        // for each value returned by the function, return it on the stack
        // keep track of the change to stack ptr from previous returns
        let mut sp_counter = 0;
        let mut offset = String::from("");
        for value in results {
            match value {
                wast::ValType::I32 => {
                    // compute the offset to read from the bottom of the stack
                    if sp_counter > 0 {
                        offset = format!("stack_u32[stack_frames[*sfp]] = stack_u32[*sp - {} - 1];", sp_counter);
                    } else {
                        offset = String::from("stack_u32[stack_frames[*sfp]] = stack_u32[*sp - 1];");
                    }
                    final_str += &format!("\t{}\n", offset);
                    sp_counter += 1;
                },
                wast::ValType::I64 => {
                    // compute the offset to read from the bottom of the stack
                    if sp_counter > 0 {
                        offset = format!("*(ulong *)(stack_u32+stack_frames[*sfp]]) = *(ulong *)(stack_u32+*sp-{}-2);", sp_counter);
                    } else {
                        offset = String::from("*(ulong *)(stack_u32+stack_frames[*sfp]) = *(ulong *)(stack_u32 + *sp - 2);");
                    }
                    final_str += &format!("\t{}\n", offset);
                    sp_counter += 2;
                },
                wast::ValType::F32 => {
                    // compute the offset to read from the bottom of the stack
                    if sp_counter > 0 {
                        offset = format!("stack_u32[stack_frames[*sfp]] = stack_u32[*sp - {} - 1];", sp_counter);
                    } else {
                        offset = String::from("stack_u32[stack_frames[*sfp]] = stack_u32[*sp - 1];");
                    }
                    final_str += &format!("\t{}\n", offset);
                    sp_counter += 1;
                },
                wast::ValType::F64 => {
                    // compute the offset to read from the bottom of the stack
                    if sp_counter > 0 {
                        offset = format!("*(ulong *)(stack_u32+stack_frames[*sfp - 1]]) = *(ulong *)(stack_u32+*sp-{}-2);", sp_counter);
                    } else {
                        offset = String::from("*(ulong *)(stack_u32+stack_frames[*sfp - 1]) = *(ulong *)(stack_u32 + *sp - 2);");
                    }
                    final_str += &format!("\t{}\n", offset);
                    sp_counter += 2;
                },
                _ => panic!("Unimplemented function return type!!!"),
            }
        }
        final_str += &format!("\t{}\n",
                                // reset the stack pointer to point at the end of the previous frame
                                "*sp = stack_frames[*sfp];");
        final_str += &format!("\t{}\n\t\t{}\n\t{}\n\t\t{}\n\t{}\n",
                                // check if *sfp == 0
                                "if (*sfp != 0) {",
                                // if *sfp != 0, that means we have to return to the previous stack frame
                                    "goto function_return_stub;",
                                "} else {",
                                // we are the top-level stack frame, and can now exit the program
                                    "return;",
                                "}");
        final_str
    }

    fn emit_instructions(&self, instr: &wast::Instruction,
                         offsets: &HashMap<&str, u32>,
                         type_info: &HashMap<&str, ValType>,
                         call_ret_map: &mut HashMap<&str, u32>,
                         call_ret_idx: &mut u32,
                         debug: bool) -> String {
        match instr {
            wast::Instruction::I32Const(val) => self.emit_i32_const(val, debug),
            wast::Instruction::I64Const(val) => self.emit_i64_const(val, debug),
            wast::Instruction::LocalGet(idx) => {
                match idx {
                    wast::Index::Id(id) => self.emit_local_get(id.name(), offsets, type_info, debug),
                    wast::Index::Num(_, _) => panic!("no support for Num index references in local.get yet"),
                }
            },
            wast::Instruction::LocalSet(idx) => {
                match idx {
                    wast::Index::Id(id) => self.emit_local_set(id.name(), offsets, type_info, debug),
                    wast::Index::Num(_, _) => panic!("no support for Num index references in local.get yet"),
                }
            },
            wast::Instruction::LocalTee(idx) => {
                match idx {
                    wast::Index::Id(id) => self.emit_local_tee(id.name(), offsets, type_info, debug),
                    wast::Index::Num(_, _) => panic!("no support for Num index references in local.get yet"),
                }
            },
            wast::Instruction::I32Add => {
                self.emit_i32_add(debug)
            },
            wast::Instruction::I64Add => {
                self.emit_i64_add(debug)
            },
            wast::Instruction::Call(idx) => {
                self.emit_fn_call(*idx, call_ret_map, call_ret_idx, debug)
            },
            wast::Instruction::I32LtS => self.emit_i32_lt_s(debug),
            _ => panic!("Instruction {:?} not yet implemented", instr)
        }
    }

    fn emit_function(&self, func: &wast::Func, call_ret_map: &mut HashMap<&str, u32>, call_ret_idx: &mut u32, debug: bool) -> String {
        let mut final_string = String::from("");

        // store the stack offset for all parameters and locals
        let mut local_parameter_stack_offset: HashMap<&str, u32> = HashMap::new();
        let mut local_type_info: HashMap<&str, ValType> = HashMap::new();

        dbg!("{:?}", func);
        // Function header
        match (&func.kind, &func.id, &func.ty) {
            (wast::FuncKind::Import(_), _, _) => {
                dbg!("InlineImport function");
                // In this case, we have an InlineImport of the form:
                // (func (type 3) (import "foo" "bar"))
                panic!("InlineImport functions not yet implemented");
            },
            (wast::FuncKind::Inline{locals, expression}, Some(id), typeuse) => {
                dbg!("InlineImport function");
                dbg!(id.name());
                dbg!("{:?}", locals);
                dbg!("{:?}", expression);

                let mut offset = 0;
                // get offsets for parameters, we record offsets from the start of the stack frame
                for parameter in typeuse.clone().inline.unwrap().params.to_vec() {
                    dbg!(parameter);
                    match parameter {
                        (Some(id), _, t) => {
                            dbg!(id);
                            local_parameter_stack_offset.insert(id.name(), offset);
                            local_type_info.insert(id.name(), t.clone());
                            offset += self.get_size_valtype(&t);
                        },
                        _ => panic!("Unhandled parameter type")
                    }
                }

                //stack_frames[*sfp - 1]; points to the head of the stack frame

                // get offsets for locals
                for local in locals {
                    local_parameter_stack_offset.insert(local.id.unwrap().name(), offset);
                    local_type_info.insert(local.id.unwrap().name(), local.ty.clone());
                    offset += self.get_size_valtype(&local.ty);
                }

                final_string += &format!("{}:\n", id.name());
                /*
                 * Stack setup for each function:
                 * parameters *then* locals onto the same stack
                 * when emitting code we index into the stack since we know
                 * exactly how many parameters, locals we have.
                 * 
                 * 
                 * Calling convention:
                 *      The caller of a function is always responsible for stack frame init + parameters. 
                 * In our case - if the caller is external to the runtime, then they have to set up the stack
                 * frame and increment sfp + pass arguments onto the frame appropriately.
                 * 
                 */

                // for each local, push them onto the stack

                for local in locals {
                    final_string += &self.emit_local(local.clone(), &local_parameter_stack_offset, debug);
                }

                // we are now ready to execute instructions!
                for instruction in expression.instrs.iter() {
                    final_string += &self.emit_instructions(instruction,
                                                            &local_parameter_stack_offset,
                                                            &local_type_info,
                                                            call_ret_map,
                                                            call_ret_idx,
                                                            debug);
                }

                // to unwind from the function we unwind the call stack by moving the stack pointer
                // and returning the last value on the stack 
                final_string += &self.function_unwind(&typeuse.inline, debug);
            },
            (_, _, _) => panic!("Inline function must always have a valid identifier in wasm")
        };
        final_string
    }

    pub fn write_opencl_file(&self, filename: &str, debug: bool) -> () {
        /*
        if Path::new(filename).exists() {
            // cannot proceed with file creation
            //panic!("path exists already!");
        }
        */

        let mut output = File::create(filename).unwrap();

        // if we are running in debug C-mode, we must define the openCL types
        if debug {
            write!(output, "{}", format!("#include <stdlib.h>\n"));
            write!(output, "{}", format!("#define uchar unsigned char\n"));
            write!(output, "{}", format!("#define ulong unsigned long\n"));
            write!(output, "{}", format!("#define uint unsigned int\n"));
        }

        /*
         * Generate code for each function in the file first
         */
        if debug {
            write!(output, "void wasm_entry(uint *stack_u32,
                                            ulong *stack_u64,
                                            uint *heap_u32,
                                            ulong *heap_u64,
                                            uint *stack_frames,
                                            ulong *sp,
                                            ulong *sfp,
                                            ulong *call_stack,
                                            uint entry_point) {{\n");
        } else {
            let header = format!("__kernel void wasm_entry(__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}) {{\n",
                                    "uint  *stack_u32_global,",
                                    "ulong *stack_u64_global,",
                                    "uint  *heap_u32_global,",
                                    "ulong *heap_u64_global,",
                                    "uint  *stack_frames_global,",
                                    "ulong *sp_global,",
                                    "ulong *sfp_global,",
                                    "ulong *call_stack,",
                                    "uint  *entry_point_global");

            write!(output, "{}", header);
            // TODO: for the openCL launcher, pass the memory stride as a function parameter
            write!(output, "\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n",
                           "uint  *stack_u32    = (uint*)stack_u32_global+(get_global_id(0) * 1024 * 16);",
                           "ulong *stack_u64    = (ulong*)stack_u32;",
                           "uint  *heap_u32     = (uint *)heap_u32_global+(get_global_id(0) * 1024 * 16);",
                           "ulong *heap_u64     = (ulong *)heap_u32;",
                           "uint  *stack_frames = (uint*)stack_frames_global+(get_global_id(0) * 1024 * 16);",
                           // only an array of N elements, where N=warp size
                           "ulong *sp           = (ulong *)sp_global+(get_global_id(0));",
                           // the stack frame pointer is used for both the stack frame, and call stack as they are
                           // essentially the same structure, except they hold different values
                           "ulong *sfp          = (ulong*)sfp_global+(get_global_id(0) * 1024 * 16);",
                           // holds the numeric index of the return label for where to jump after a function call
                           "ulong *call_stack   = (ulong*)call_stack+(get_global_id(0) * 1024 * 16);",
                           "uint  entry_point   = entry_point_global[get_global_id(0)];");
        }
        
        // for each function in the program, set up a table mapping function idx to the label
        // this is needed for starting/resuming the application.
        
        // for each function, assign an ID -> index mapping
        let mut function_idx_label: HashMap<&str, u32> = HashMap::new();
        let mut count = 0;
        let funcs = self.func_map.values();
        for function in funcs.clone() {
            match function.id {
                Some(id) => {
                    function_idx_label.insert(id.name(), count);
                }
                None => {
                    println!("import function without ID -- cannot add label");
                },
            }
            count += 1;
        }
        write!(output, "\t{}\n", "uint caller;");
        write!(output, "\t{}\n", "switch (entry_point) {");
        for key in function_idx_label.keys() {
            write!(output, "\t\tcase {}:\n", function_idx_label.get(key).unwrap());
            write!(output, "\t\t\tgoto {};\n", key);
            write!(output, "\t\t\tbreak;\n");
        } 
        write!(output, "\t}}\n");

        let mut call_ret_map: &mut HashMap<&str, u32> = &mut HashMap::new();
        let mut call_ret_idx: &mut u32 = &mut 0;
        for function in funcs {
            let func = self.emit_function(function, call_ret_map, call_ret_idx, debug);
            write!(output, "{}", func);
        }

        // generate the function call return table
        
        write!(output, "{}\n", "function_return_stub:");
        write!(output, "\t{}\n", "switch (call_stack[*sfp]) {");
        for count in 0..*call_ret_idx {
            write!(output, "\t\tcase {}:\n", count);
            write!(output, "\t\t\t*sfp -= 1;\n");
            write!(output, "\t\t\tgoto call_return_stub_{};\n", count);
            write!(output, "\t\t\tbreak;\n");
        } 
        write!(output, "\t}}\n");


        write!(output, "}}\n");

        if debug {
            write!(output, "{}", format!("int main(int argc, char *argv[]) {{\n"));
            write!(output, "{}", format!("\tuint *stack_u32 = calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tulong *stack_u64 = (ulong *)stack_u32;\n"));
            write!(output, "{}", format!("\tuint *heap_u32 = (uint *)calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tulong *heap_u64 = (ulong *)calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tuint *stack_frames = calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tuint *call_stack = calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tulong sp = 0;\n"));
            write!(output, "{}", format!("\tulong sfp = 0;\n"));
            write!(output, "{}", format!("\tstack_frames[sfp] = sp;\n"));
            write!(output, "{}", format!("\tstack_u64[0] = 0x1;\n"));
            write!(output, "{}", format!("\tsp += 2;\n"));

            // TODO when calling the function get the entry_point for main

            write!(output, "{}", format!("{}",
                    format!("\twasm_entry(stack_u32, stack_u64, heap_u32, heap_u64, stack_frames, &sp, &sfp, call_stack, {});\n",
                            function_idx_label.get("_main").unwrap())));
            // now check the result
            write!(output, "{}", format!("\tprintf(\"%d\\n\", stack_u32[sp]);\n"));

            write!(output, "}}\n\n");
        }

    }
}

impl fmt::Debug for OpenCLCWriter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenCLCWriter")
        .field("types", &self.types)
        .field("imports", &self.imports)
        .field("func_map", &self.func_map)
        .field("tables", &self.tables)
        .field("memory", &self.memory)
        .field("globals", &self.globals)
        .field("start", &self.start)
        .field("elements", &self.elements)
        .field("data", &self.data)
        .finish()
    }
}


/*
reference from wasm2c: 

void CWriter::WriteCHeader() {
  stream_ = h_stream_;
  std::string guard = GenerateHeaderGuard();
  Write("#ifndef ", guard, Newline());
  Write("#define ", guard, Newline());
  Write(s_header_top);
  WriteImports();
  WriteExports(WriteExportsKind::Declarations);
  Write(s_header_bottom);
  Write(Newline(), "#endif  /* ", guard, " */", Newline());
}

void CWriter::WriteCSource() {
  stream_ = c_stream_;
  WriteSourceTop();
  WriteFuncTypes();
  WriteFuncDeclarations();
  WriteGlobals();
  WriteMemories();
  WriteTables();
  WriteFuncs();
  WriteDataInitializers();
  WriteElemInitializers();
  WriteExports(WriteExportsKind::Definitions);
  WriteInitExports();
  WriteInit();
}

Result CWriter::WriteModule(const Module& module) {
  WABT_USE(options_);
  module_ = &module;
  WriteCHeader();
  WriteCSource();
  return result_;
}

*/