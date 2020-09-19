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
    funcs: Vec<wast::Func<'a>>,
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
            funcs: vec!(),
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
                        wast::ModuleField::Func(f) => self.funcs.push(f),
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

    fn emit_local(&self, local: &wast::Local, debug: bool) -> String {
        /*
         * When emitting locals we know we have access to the global stack.
         * We zero-init all values.
         * 
         * We have an inefficient stack layout right now... we will fix later if it is needed
         * 
         */
        if debug {
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
                    String::from("\tstack_u64[*sp] = 0;\n\t*sp += 2;\n")
                },
                wast::ValType::F32 => {
                    String::from("\tstack_u32[*sp] = 0;\n\t*sp += 1;\n")
                },
                wast::ValType::F64 => {
                    String::from("\tstack_u64[*sp] = 0;\n\t*sp += 2;\n")
                },
                _ => panic!(),
            }
        } else {
            String::from("")
        }
    }

    fn emit_i32_const(&self, val: &i32, debug: bool) -> String {
        if debug {
            format!("\tstack_u32[*sp] = (uint){};\n\t*sp += 1;\n", val)
        } else {
            format!("\tstack_u32[*sp] = (uint){};\n\t*sp += 1;\n", val)
        }
    }

    fn emit_i64_const(&self, val: &i64, debug: bool) -> String {
        if debug {
            format!("\t{}{};\n\t*sp += 2;\n",
                    "*(ulong *)(stack_u32+*sp) = (ulong)",
                    val)
        } else {
            format!("\t{}{};\n\t*sp += 2;\n",
                    "*(ulong *)(stack_u32+*sp) = (ulong)",
                    val)
        }
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
        if debug {
            let offset = offsets.get(id).unwrap();
            let t = type_info.get(id).unwrap();
            dbg!(id);
            dbg!(offset);
            dbg!(t);
            // stack_frames[*sfp - 1] start of stack frame
            match t {
                wast::ValType::I32 => {
                    format!("\t{}\n\t{}\n",
                            format!("stack_u32[*sp] = *(uint *)(stack_u32 + stack_frames[*sfp - 1] + {});", offset),
                            "*sp += 1;")
                },
                wast::ValType::I64 => {
                    format!("\t{}\n\t{}\n",
                            format!("stack_u64[*sp] = *(uint *)(stack_u32 + stack_frames[*sfp - 1] + {});", offset),
                            "*sp += 2;")
                },
                wast::ValType::F32 => {
                    format!("\t{}\n",
                            format!("stack_u32[*sp] = *(uint *)(stack_u32 + stack_frames[*sfp - 1] + {});", offset))
                },
                wast::ValType::F64 => {
                    format!("\t{}\n\t{}\n",
                            format!("stack_u64[*sp] = *(uint *)(stack_u32 + stack_frames[*sfp - 1] + {});", offset),
                            "*sp += 2;")
                },
                _ => panic!("emit_local_set type not handled")
            }
        } else {
            format!("\t{};\n\t*sp += 2;\n",
                    "*(ulong *)(stack_u32+*sp) = (ulong)")
        }

    }

    fn emit_local_set(&self, id: &str, offsets: &HashMap<&str, u32>, type_info: &HashMap<&str, ValType>, debug: bool) -> String {
        if debug {
            let offset = offsets.get(id).unwrap();
            let t = type_info.get(id).unwrap();
            dbg!(id);
            dbg!(offset);
            dbg!(t);
            // stack_frames[*sfp - 1] start of stack frame
            match t {
                wast::ValType::I32 => {
                    format!("\t{}\n",
                            format!("*(uint *)(stack_u32 + stack_frames[*sfp - 1] + {}) = stack_u32[*sp - 1];", offset))
                },
                wast::ValType::I64 => {
                    format!("\t{}\n",
                            format!("*(ulong *)(stack_u32 + stack_frames[*sfp - 1] + {}) = *(ulong *)stack_u32[*sp - 2];", offset))
                },
                wast::ValType::F32 => {
                    format!("\t{}\n",
                            format!("*(uint *)(stack_u32 + stack_frames[*sfp - 1] + {}) = stack_u32[*sp - 1];", offset))
                },
                wast::ValType::F64 => {
                    format!("\t{}\n",
                            format!("*(ulong *)(stack_u32 + stack_frames[*sfp - 1] + {}) = *(ulong *)stack_u32[*sp - 2];", offset))
                },
                _ => panic!("emit_local_set type not handled")
            }
        } else {
            format!("\t{};\n\t*sp += 2;\n",
                    "*(ulong *)(stack_u32+*sp) = (ulong)")
        }
    }

    // TODO: this code will be called right before "calling" a function
    fn function_prelude(&self, debug: bool) -> String {
        if debug {
            // when entering a function we need to set up the stack frame for it
            format!("\t{}\n",
                    "")
        } else {
            format!("")
        }
    }

    // TODO: this needs to take the function type into account
    fn function_unwind(&self, func_ret_info: &Option<wast::FunctionType>, debug: bool) -> String {
        let mut final_str = String::from("");

        let results: Vec<wast::ValType> = match func_ret_info {
            Some(s) => (*s.results).to_vec(),
            None => vec![]
        };
        
        if debug {
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
                            offset = format!("stack_u32[stack_frames[*sfp - 1]] = stack_u32[*sp - {} - 1];", sp_counter);
                        } else {
                            offset = String::from("stack_u32[stack_frames[*sfp - 1]] = stack_u32[*sp - 1];");
                        }
                        final_str += &format!("\t{}\n", offset);
                        sp_counter += 1;
                    },
                    wast::ValType::I64 => {
                        // compute the offset to read from the bottom of the stack
                        if sp_counter > 0 {
                            offset = format!("stack_u64[stack_frames[*sfp - 1]] = stack_u64[*sp - {} - 2];", sp_counter);
                        } else {
                            offset = String::from("stack_u64[stack_frames[*sfp - 1]] = *(ulong *)(stack_u32 + *sp - 2);");
                        }
                        final_str += &format!("\t{}\n", offset);
                        sp_counter += 2;
                    },
                    wast::ValType::F32 => {
                        // compute the offset to read from the bottom of the stack
                        if sp_counter > 0 {
                            offset = format!("stack_u32[stack_frames[*sfp - 1]] = stack_u32[*sp - {} - 1];", sp_counter);
                        } else {
                            offset = String::from("stack_u32[stack_frames[*sfp - 1]] = stack_u32[*sp - 1];");
                        }
                        final_str += &format!("\t{}\n", offset);
                        sp_counter += 1;
                    },
                    wast::ValType::F64 => {
                        // compute the offset to read from the bottom of the stack
                        if sp_counter > 0 {
                            offset = format!("stack_u64[stack_frames[*sfp - 1]] = stack_u64[*sp - {} - 2];", sp_counter);
                        } else {
                            offset = String::from("stack_u64[stack_frames[*sfp - 1]] = *(ulong *)(stack_u32 + *sp - 2);");
                        }
                        final_str += &format!("\t{}\n", offset);
                        sp_counter += 2;
                    },
                    _ => panic!("Unimplemented function return type!!!"),
                }
            }
            final_str += &format!("\t{}\n\t{}\n",
                                  // reset the stack pointer to point at the end of the previous frame
                                  "*sp = stack_frames[*sfp - 1];",
                                  // now reset the stack frame pointer
                                  "*sfp -= 1;");
            final_str += &format!("\t{}\n",
                                  // check if we have reached the last function in the callstack
                                  // if so - return to the host
                                  "if (*sp == 0) return;");
        } else {
            final_str += &format!("");
        }
        final_str
    }

    fn emit_instructions(&self, instr: &wast::Instruction,
                         offsets: &HashMap<&str, u32>,
                         type_info: &HashMap<&str, ValType>,
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
            _ => panic!("Instruction {:?} not yet implemented", instr)
        }
    }

    fn emit_function(&self, func: &wast::Func, debug: bool) -> String {
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
                 *      The caller of a function is always responsible for stack frame init. 
                 * In our case - if the caller is external to the runtime, then they have to set up the stack
                 * frame and increment sfp + pass arguments onto the frame appropriately.
                 * 
                 */

                // for each local, push them onto the stack
                for local in locals {
                    final_string += &self.emit_local(local.clone(), debug);
                }

                // we are now ready to execute instructions!
                for instruction in expression.instrs.iter() {
                    final_string += &self.emit_instructions(instruction,
                                                            &local_parameter_stack_offset,
                                                            &local_type_info,
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
        write!(output, "void wasm_entry(uint *stack_u32, ulong *stack_u64, uint *heap_u32, ulong *heap_u64, uint *stack_frames, ulong *sp, ulong *sfp, uint entry_point) {{\n");
        
        // for each function in the program, set up a table mapping function idx to the label
        // this is needed for starting/resuming the application.
        
        // for each function, assign an ID -> index mapping
        let mut function_idx_label: HashMap<&str, u32> = HashMap::new();

        for function in &self.funcs {
            match function.id {
                Some(id) => {
                    function_idx_label.insert(id.name(),0);
                }
                None => {
                    println!("import function without ID -- cannot add label");
                },
            }
        }

        write!(output, "\t{}\n", "switch (entry_point) {");
        for key in function_idx_label.keys() {
            write!(output, "\t\tcase {}:\n", function_idx_label.get(key).unwrap());
            write!(output, "\t\t\tgoto {};\n", key);
            write!(output, "\t\t\tbreak;\n");
        } 
        write!(output, "\t}}\n");

        for function in &self.funcs {
            let func = self.emit_function(function, debug);
            write!(output, "{}", func);
        }
        write!(output, "}}\n");

        if debug {
            write!(output, "{}", format!("int main(int argc, char *argv[]) {{\n"));
            write!(output, "{}", format!("\tuint *stack_u32 = calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tulong *stack_u64 = (ulong *)stack_u32;\n"));
            write!(output, "{}", format!("\tuint *heap_u32 = (uint *)calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tulong *heap_u64 = (ulong *)calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tuint *stack_frames = calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tulong sp = 0;\n"));
            write!(output, "{}", format!("\tulong sfp = 1;\n"));
            write!(output, "{}", format!("\tstack_frames[sfp - 1] = sp;\n"));

            // demo: pass 42 as an argument
            //write!(output, "{}", format!("\tstack_u32[sp] = 42;\n"));
            //write!(output, "{}", format!("\tsp += 1;\n"));
            write!(output, "{}", format!("\twasm_entry(stack_u32, stack_u64, heap_u32, heap_u64, stack_frames, &sp, &sfp, 0);\n"));
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
        .field("funcs", &self.funcs)
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