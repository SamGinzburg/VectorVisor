use wast::Wat;
use wast::parser::{self, ParseBuffer};
use wast::ModuleKind::{Text, Binary};
use wast::ValType;
use wast::Instruction;

use std::path::Path;
use std::fmt;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

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

    pub fn emit_local(&self, local: &wast::Local, debug: bool) -> String {
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
                    String::from("\tstack_u32[*sp] = 0;\n\t*sp += 2;\n")
                },
                wast::ValType::I64 => {
                    String::from("\tstack_u64[*sp] = 0;\n\t*sp += 2;\n")
                },
                wast::ValType::F32 => {
                    String::from("\tstack_u32[*sp] = 0;\n\t*sp += 2;\n")
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
            format!("\tstack_u32[*sp] = (uint){};\n\t*sp += 2;\n", val)
        } else {
            format!("\tstack_u32[*sp] = (uint){};\n\t*sp += 2;\n", val)
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
    fn function_unwind(&self, debug: bool) -> String {
        if debug {
            // stack_frames[*sfp] contains the stack pointer pointing to the start of the stack frame
            // we want to use this as the index to write the function return value
            format!("\t{}\n\t{}\n\t{}\n",
                    "stack_u64[stack_frames[*sfp - 1] / 2] = stack_u64[(*sp / 2) - 1];",
                    // now that we have set the return value, we want to reset the stack pointer
                    // it ends up pointing right after the returned value from "call"
                    "*sp = (stack_frames[*sfp - 1] / 2) + 1;",
                    // now reset the stack frame pointer
                    "*sfp -= 1;")
        } else {
            format!("")
        }
    }

    pub fn emit_instructions(&self, instr: &wast::Instruction, debug: bool) -> String {
        match instr {
            wast::Instruction::I32Const(val) => self.emit_i32_const(val, debug),
            _ => panic!("Instruction {:?} not yet implemented", instr)
        }
    }

    pub fn write_opencl_file(&self, filename: &str, debug: bool) -> () {

        if Path::new(filename).exists() {
            // cannot proceed with file creation
            panic!("path exists already!");
        }

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
        for function in &self.funcs {
            dbg!("{:?}", function);
            // Function header
            match (&function.kind, &function.id) {
                (wast::FuncKind::Import(_), _) => {
                    dbg!("InlineImport function");
                    // In this case, we have an InlineImport of the form:
                    // (func (type 3) (import "foo" "bar"))
                    continue
                },
                (wast::FuncKind::Inline{locals, expression}, Some(id)) => {
                    dbg!("InlineImport function");
                    dbg!(id.name());
                    dbg!("{:?}", locals);
                    dbg!("{:?}", expression);
                    write!(output, "{}", format!("void {} (uint *stack_u32, ulong *stack_u64, uint *heap_u32, ulong *heap_u64, uint *stack_frames, ulong *sp, ulong *sfp) {{\n", id.name()));

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
                        write!(output, "{}", self.emit_local(local.clone(), debug));
                    }
 
                    // we are now ready to execute instructions!
                    for instruction in expression.instrs.iter() {
                        write!(output, "{}", self.emit_instructions(instruction, debug));
                    }

                    // to unwind from the function we unwind the call stack by moving the stack pointer
                    // and returning the last value on the stack 
                    write!(output, "{}", self.function_unwind(debug));

                    write!(output, "}}\n\n");
                },
                (_, _) => panic!("Inline function must always have a valid identifier in wasm")
            }
        }

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
            write!(output, "{}", format!("\tstack_u32[sp] = 42;\n"));
            write!(output, "{}", format!("\tsp += 1;\n"));
            write!(output, "{}", format!("\t_main(stack_u32, stack_u64, heap_u32, heap_u64, stack_frames, &sp, &sfp);\n"));
            // now check the result
            write!(output, "{}", format!("\tprintf(\"%d\", stack_u32[sp]);\n"));

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