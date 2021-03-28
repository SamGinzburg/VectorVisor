mod binops;
mod stackops;
mod control_flow;
mod functions;
mod mem_interleave;
mod relops;
mod wasi_helpers;
mod globals;
mod memargs;
mod testops;
mod convops;
mod parametric;
mod unops;
mod vstack;

use relops::*;
use mem_interleave::*;
use functions::*;
use stackops::*;
use binops::*;
use control_flow::*;
use wasi_helpers::*;
use globals::*;
use memargs::*;
use testops::*;
use convops::*;
use parametric::*;
use unops::*;
use vstack::*;

use wast::Wat;
use wast::parser::{self, ParseBuffer};
use wast::ModuleKind::{Text, Binary};
use wast::ValType;
use regex::Regex;

use std::fmt;
//use std::fs::File;
//use std::io::Write;
use std::fmt::Write;
use std::collections::HashMap;
use std::convert::TryInto;

use lazy_static::lazy_static;

/*
 * This hashmap contains the WASI calls that have already been implemented!
 */
lazy_static! {
    static ref WASI_SNAPSHOT_PREVIEW1: HashMap<&'static str, bool> = {
        let mut m = HashMap::new();
        m.insert("fd_write", true);               // 0
        m.insert("proc_exit", true);              // 1
        m.insert("environ_sizes_get", true);      // 2
        m.insert("environ_get", true);            // 3
        m.insert("fd_prestat_get", true);         // 4
        m.insert("fd_prestat_dir_name", true);    // 5
        m.insert("random_get", true);             // 6
        m.insert("serverless_invoke", true);      // 9999
        m.insert("serverless_response", true);    // 10000
        m
    };
}

#[derive(Clone)]
enum WasmHypercallId {
    fd_write              = 0,
    proc_exit             = 1,
    environ_sizes_get     = 2,
    environ_get           = 3,
    fd_prestat_get        = 4,
    fd_prestat_dir_name   = 5,
    random_get            = 6,
    serverless_invoke     = 9999,
    serverless_response   = 10000,
}

pub struct OpenCLCWriter<'a> {
    types: HashMap<String, wast::TypeDef<'a>>,
    imports_map: HashMap<&'a str, (&'a str, Option<&'a str>, wast::ItemSig<'a>)>,
    // map of item.id -> (module, field)
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
            types: HashMap::new(),
            imports_map: HashMap::new(),
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
        let mut type_count = 0;
        match module.module.kind {
            Text(t) => {
                for item in t {
                    match item {
                        wast::ModuleField::Type(t) => {
                            let id = match t.id {
                                Some(i) => i.name().to_string(),
                                _ => format!("t{}", type_count)
                            };
                            self.types.insert(id, t.def);
                            type_count += 1;
                        },
                        wast::ModuleField::Import(i) => {
                            match i.clone().item.id {
                                Some(id) => self.imports_map.insert(id.name(), (i.module, i.field, i.item)),
                                None => continue,
                            };
                        },
                        wast::ModuleField::Func(f) => {
                            match f.id {
                                Some(f_id) => self.func_map.insert(f_id.name(), f),
                                None => {
                                    continue
                                },
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

    fn get_size_valtype(&self, t: &ValType) -> u32 {
        match t {
            wast::ValType::I32 => 1,
            wast::ValType::F32 => 1,
            wast::ValType::I64 => 2,
            wast::ValType::F64 => 2,
            _ => panic!("Unknown local size found"),
        }
    }

    fn emit_hypercall(&self, hypercall_id: WasmHypercallId, hypercall_id_count: &mut u32, fn_name: String, is_proc_exit_start: bool, debug: bool) -> String {
        let mut ret_str = String::from("");
        // set the hypercall ret flag flag + r
        ret_str += &format!("\t{}\n", format!("*hypercall_number = {};", hypercall_id.clone() as u32));

        // run the hypercall setup code - to marshall data for the VMM to read

        // hypercalls that are omitted from this table are implied to not require any data transfer via the hcall buffer 
        match hypercall_id {
            WasmHypercallId::fd_write => ret_str += &emit_fd_write_call_helper(self, debug),
            WasmHypercallId::fd_prestat_get => ret_str += &emit_fd_prestat_get_helper(self, debug),
            WasmHypercallId::fd_prestat_dir_name => ret_str += &emit_fd_prestat_dir_name_helper(self, debug),
            WasmHypercallId::serverless_invoke => ret_str += &emit_serverless_invoke_pre(self, debug),
            WasmHypercallId::serverless_response => ret_str += &emit_serverless_response_pre(self, debug),
            WasmHypercallId::random_get => ret_str += &emit_random_get_pre(self, debug),
            _ => (),
        }
        // insert return (we exit back to the VMM)
        ret_str += &format!("\t{}\n\t{}\n",
                            format!("*hypercall_continuation = {};", hypercall_id_count),
                            "return;");

        // proc_exit in _start is special cased since we add it ourselves
        if !is_proc_exit_start {
            // insert return label, the VMM will return to right after the return
            ret_str += &format!("{}_hypercall_return_stub_{}:\n", format!("{}{}", "__", fn_name.replace(".", "")), hypercall_id_count);
        }

        // after the hypercall, we need to reset values on re-entry, and possible copy data back from the hcall buf
        // skipped hypercall entries here are no-ops
        match hypercall_id {
            WasmHypercallId::fd_write => {
                ret_str += &emit_fd_write_post(&self, debug);
            },
            WasmHypercallId::environ_sizes_get => {
                ret_str += &emit_environ_sizes_get_post(&self, debug);
            },
            WasmHypercallId::environ_get => {
                ret_str += &emit_environ_get_post(&self, debug);
            },
            WasmHypercallId::fd_prestat_get => {
                ret_str += &emit_fd_prestat_get_post(&self, debug);
            },
            WasmHypercallId::fd_prestat_dir_name => {
                ret_str += &emit_fd_prestat_dir_name_post(&self, debug);
            },
            WasmHypercallId::serverless_invoke => {
                ret_str += &emit_serverless_invoke_post(&self, debug);
            },
            WasmHypercallId::serverless_response => {
                ret_str += &emit_serverless_response_post(&self, debug);
            },
            WasmHypercallId::random_get => {
                ret_str += &emit_random_get_post(&self, debug);
            },
            _ => (),
        }

        // increment hypercall_id_count
        *hypercall_id_count += 1;
        ret_str
    }

    fn emit_instructions(&self,
                         // instruction to emit
                         instr: &wast::Instruction,
                         // sizes of current stack items
                         stack_sizes: &mut Vec<u32>,
                         // the offset of parameters on the current callstack
                         parameter_offset: i32,
                         // map of local/parameter IDs to offset from stack frame start
                         offsets: &HashMap<String, u32>,
                         type_info: &HashMap<String, ValType>,
                         call_ret_map: &mut HashMap<&str, u32>,
                         call_ret_idx: &mut u32,
                         // function name
                         fn_name: &str,
                         // stack of control flow operations (blocks, loops)
                         control_stack: &mut Vec<(String, u32, i32)>,
                         // map of function names to IDs
                         function_id_map: HashMap<&str, u32>,
                         // count of how many hypercalls have been encountered (used for re-entry)
                         hypercall_id_count: &mut u32,
                         // map of indexes in the indirect call table ($T0) to function indicies
                         indirect_call_mapping: &HashMap<u32, &wast::Index>,
                         // map of global identifiers to (offset, size of global)
                         global_mappings: &HashMap<String, (u32, u32)>,
                         // the current function
                         func: &wast::Func,
                         // if we are parsing WASM code with blocks without names, we have to make up names
                         // When doing so we simply name the block "$block{block_name_count}"
                         block_name_count: &mut u32,
                         loop_name_count: &mut u32,
                         // emit OpenCL C (False) or standard C for debugging on the CPU (True)
                         debug: bool) -> String {
        match instr {
            wast::Instruction::Drop => {
                // based on the previous stack size, decrement sp
                // we don't need to handle all cases, only the common case provided by LLVM output
                // which is when drop follows a function call

                format!("\t{}{};\n",
                        "*sp -= ", stack_sizes.pop().unwrap())
            }
            wast::Instruction::I32Store(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore_i32(self, memarg, debug)
            },
            wast::Instruction::I32Store8(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore8_i32(self, memarg, debug)
            },
            wast::Instruction::I64Store8(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore8_i64(self, memarg, debug)
            },
            wast::Instruction::I64Store16(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore16_i64(self, memarg, debug)
            },
            wast::Instruction::I32Store16(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore16_i32(self, memarg, debug)
            },
            wast::Instruction::I64Store32(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore32_i64(self, memarg, debug)
            },
            wast::Instruction::I32Load(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_i32(self, memarg, debug)
            },
            wast::Instruction::I32Load8u(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_i32_8u(self, memarg, debug)
            },
            wast::Instruction::I64Load16u(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_memload_i64_16u(self, memarg, debug)
            },
            wast::Instruction::I32Load16u(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_i32_16u(self, memarg, debug)
            },
            wast::Instruction::I32Load16s(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_i32_16s(self, memarg, debug)
            },
            wast::Instruction::I32Load8s(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_i32_8s(self, memarg, debug)
            },
            wast::Instruction::I64Load8u(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_memload_i64_8u(self, memarg, debug)
            },
            wast::Instruction::I64Load32u(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_memload_i64_32u(self, memarg, debug)
            },
            wast::Instruction::I64Load(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_memload_i64(self, memarg, debug)
            },
            wast::Instruction::F64Load(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                // we can reuse the i64 load code
                emit_memload_i64(self, memarg, debug)
            },
            wast::Instruction::I64Store(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore_i64(self, memarg, debug)
            },
            wast::Instruction::F64Store(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                // we can reuse the i64 code for f64
                emit_memstore_i64(self, memarg, debug)
            },
            wast::Instruction::GlobalGet(idx) => {
                match idx {
                    wast::Index::Id(id) => {
                        emit_global_get(self, id.name(), global_mappings, stack_sizes, debug)
                    },
                    wast::Index::Num(value, _) => {
                        emit_global_get(self, &format!("g{}", value), global_mappings, stack_sizes, debug)
                    },
                }
            },
            wast::Instruction::GlobalSet(idx) => {
                match idx {
                    wast::Index::Id(id) => {
                        emit_global_set(self, id.name(), global_mappings, stack_sizes, debug)
                    },
                    wast::Index::Num(value, _) => {
                        emit_global_set(self, &format!("g{}", value), global_mappings, stack_sizes, debug)
                    },
                }
            },
            wast::Instruction::I32Const(val) => {
                stack_sizes.push(1);
                emit_i32_const(self, val, debug)
            },
            wast::Instruction::I64Const(val) => {
                stack_sizes.push(2);
                emit_i64_const(self, val, debug)
            },
            wast::Instruction::F32Const(val) => {
                stack_sizes.push(1);
                emit_f32_const(self, &val.bits, debug)
            },
            wast::Instruction::F64Const(val) => {
                stack_sizes.push(2);
                emit_f64_const(self, &val.bits, debug)
            },
            wast::Instruction::LocalGet(idx) => {
                match idx {
                    wast::Index::Id(id) => {
                        emit_local_get(self, parameter_offset, id.name(), offsets, type_info, stack_sizes, debug)
                    },
                    wast::Index::Num(value, _) => {
                        emit_local_get(self, parameter_offset, &format!("{}", value), offsets, type_info, stack_sizes, debug)
                    },
                }
            },
            wast::Instruction::LocalSet(idx) => {
                match idx {
                    wast::Index::Id(id) => {
                        emit_local_set(self, parameter_offset, id.name(), offsets, type_info, stack_sizes, debug)
                    },
                    wast::Index::Num(value, _) => {
                        emit_local_set(self, parameter_offset, &format!("{}", value), offsets, type_info, stack_sizes, debug)
                    },
                }
            },
            wast::Instruction::LocalTee(idx) => {
                match idx {
                    wast::Index::Id(id) => {
                        emit_local_tee(self, parameter_offset, id.name(), offsets, type_info, stack_sizes, debug)
                    },
                    wast::Index::Num(value, _) => {
                        emit_local_tee(self, parameter_offset, &format!("{}", value), offsets, type_info, stack_sizes, debug)
                    },
                }
            },
            /*
             * Binops pop 2 vals and push 1 back on, so we need to pop twice
             */
            wast::Instruction::I32Add => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_add(self, debug)
            },
            wast::Instruction::I32Mul => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_mul(self, debug)
            },
            wast::Instruction::I64Mul => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_mul(self, debug)
            },
            wast::Instruction::I32Sub => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_sub(self, debug)
            },
            wast::Instruction::I64Add => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_add(self, debug)
            },
            wast::Instruction::F64Add => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_add(self, debug)
            },
            wast::Instruction::F64Div => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_div(self, debug)
            },
            wast::Instruction::F64Mul => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_mul(self, debug)
            },
            wast::Instruction::F64Neg => {
                emit_f64_neg(self, debug)
            },
            wast::Instruction::F64Ne => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f64_ne(self, debug)
            },
            wast::Instruction::F64Lt => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f64_lt(self, debug)
            },
            wast::Instruction::F64Le => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f64_le(self, debug)
            },
            wast::Instruction::I64LtU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_lt_u(self, debug)
            },
            wast::Instruction::I64Eq => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_eq(self, debug)
            },
            wast::Instruction::F64Eq => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f64_eq(self, debug)
            },
            wast::Instruction::I32TruncF64U => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_trunc_f64_u(self, debug)
            },
            wast::Instruction::I64Ne => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_ne(self, debug)
            },
            wast::Instruction::I64DivU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_div_u(self, debug)
            },
            wast::Instruction::I32Eqz => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_eqz(self, debug)
            },
            wast::Instruction::I64Eqz => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_eqz(self, debug)
            },
            wast::Instruction::I32And => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_and(self, debug)
            },
            wast::Instruction::I64And => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_and(self, debug)
            },
            wast::Instruction::I32Ne => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_ne(self, debug)
            },
            wast::Instruction::I32LtU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_lt_u(self, debug)
            },
            wast::Instruction::I32LtS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_lt_s(self, debug)
            },
            wast::Instruction::I64LtS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_lt_s(self, debug)
            },
            wast::Instruction::I32GtU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_gt_u(self, debug)
            },
            wast::Instruction::I64GtU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_gt_u(self, debug)
            },
            wast::Instruction::I64GtS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_gt_s(self, debug)
            },
            wast::Instruction::I32GtS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_gt_s(self, debug)
            },
            wast::Instruction::I32LeU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_le_u(self, debug)
            },
            wast::Instruction::I32LeS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_le_s(self, debug)
            },
            wast::Instruction::I64LeU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_le_u(self, debug)
            },
            wast::Instruction::I64LeS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_le_s(self, debug)
            },
            wast::Instruction::I32GeU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_ge_u(self, debug)
            },
            wast::Instruction::I32GeS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_ge_s(self, debug)
            },
            wast::Instruction::I64GeU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_ge_u(self, debug)
            },
            wast::Instruction::I64GeS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_ge_s(self, debug)
            },
            wast::Instruction::I32Xor => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_xor(self, debug)
            },
            wast::Instruction::I32WrapI64 => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_wrap_i64(self, debug)
            },
            wast::Instruction::I64ExtendI32S => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_extend_i32_s(self, debug)
            },
            wast::Instruction::I64ExtendI32U => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_extend_i32_u(self, debug)
            },
            wast::Instruction::Call(idx) => {
                let id = match idx {
                    wast::Index::Id(id) => id.name(),
                    _ => panic!("Unable to get Id for function call: {:?}", idx),
                };

                // check function to see if it is imported
                // if the function is imported - AND it is a WASI function,
                // emit the special call stub
                if self.imports_map.contains_key(id) {
                    match self.imports_map.get(id) {
                        Some((wasi_api, Some(wasi_fn_name), _)) => {
                            // okay, now we check to see if the WASI call is supported by the compiler
                            // if not -> panic, else, emit the call

                            match (wasi_api, WASI_SNAPSHOT_PREVIEW1.get(wasi_fn_name)) {
                                // ignore WASI API scoping for now
                                (_, Some(true)) => {
                                    match wasi_fn_name {
                                        &"fd_write"               => self.emit_hypercall(WasmHypercallId::fd_write, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"proc_exit"              => self.emit_hypercall(WasmHypercallId::proc_exit, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"environ_sizes_get"      => self.emit_hypercall(WasmHypercallId::environ_sizes_get, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"environ_get"            => self.emit_hypercall(WasmHypercallId::environ_get, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"fd_prestat_get"         => self.emit_hypercall(WasmHypercallId::fd_prestat_get, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"fd_prestat_dir_name"    => self.emit_hypercall(WasmHypercallId::fd_prestat_dir_name, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"random_get"             => self.emit_hypercall(WasmHypercallId::random_get, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"serverless_invoke"      => self.emit_hypercall(WasmHypercallId::serverless_invoke, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"serverless_response"    => self.emit_hypercall(WasmHypercallId::serverless_response, hypercall_id_count, fn_name.to_string(), false, debug),
                                        _ => panic!("Unidentified WASI fn name: {:?}", wasi_fn_name),
                                    }
                                },
                                _ => panic!("WASI import not found, this probably means the hypercall is not yet implemented: {:?}", wasi_fn_name)
                            }
                        },
                        _ => panic!("Unsupported hypercall found {:?}", self.imports_map.get(id))
                    }
                } else {
                    // else, this is a normal function call
                    // if self.func_map.get(id) is none, we have an import
                    // right now we only support WASI imports
                    match self.func_map.get(id) {
                        Some(_) => {
                            let func_type_signature = &self.func_map.get(id).unwrap().ty;
                            emit_fn_call(&self, fn_name.to_string(), *idx, call_ret_map, call_ret_idx, &function_id_map, stack_sizes, false, debug)
                        },
                        // we have an import that isn't a system call...
                        None => String::from("")
                    }
                }
            },
            wast::Instruction::CallIndirect(call_indirect) => {
                // we don't need to do table lookups because we are assuming that there can be at most 1 table
                /*
                let table: &str = match call_indirect.table {
                    wast::Index::Id(id) => id.name(),
                    wast::Index::Num(_, _) => panic!(""),
                };
                */
                emit_call_indirect(&self, call_indirect, fn_name.to_string(), parameter_offset, indirect_call_mapping, call_ret_map, call_ret_idx, function_id_map, stack_sizes, debug)
            },
            wast::Instruction::I32Eq => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_eq(self, debug)
            },
            wast::Instruction::I32Or => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_or(self, debug)
            },
            wast::Instruction::I32ShrU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_shr_u(self, debug)
            },
            wast::Instruction::I64ShrU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_shr_u(self, debug)
            },
            wast::Instruction::I32ShrS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_shr_s(self, debug)
            },
            wast::Instruction::I32Shl => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_shl(self, debug)
            },
            wast::Instruction::I64Shl => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_shl(self, debug)
            },
            wast::Instruction::I32DivU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_div_u(self, debug)
            },
            wast::Instruction::I32DivS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_div_s(self, debug)
            },
            wast::Instruction::I64DivS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_div_s(self, debug)
            },
            wast::Instruction::I32RemU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_rem_u(self, debug)
            },
            wast::Instruction::I64RemU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_rem_u(self, debug)
            },
            wast::Instruction::I32RemS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_rem_s(self, debug)
            },
            wast::Instruction::I64RemS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_rem_s(self, debug)
            },
            wast::Instruction::I64ShrS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_shr_s(self, debug)
            },
            wast::Instruction::I64Xor => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_xor(self, debug)
            },
            wast::Instruction::I64Or => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_or(self, debug)
            },
            wast::Instruction::I32Rotl => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_rotl(self, debug)
            },
            wast::Instruction::I64Rotl => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_rotl(self, debug)
            },
            wast::Instruction::I64Sub => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_sub(self, debug)
            },
            wast::Instruction::I64ReinterpretF64 => {
                // no-op
                format!("")
            },
            wast::Instruction::F64ReinterpretI64 => {
                // no-op
                format!("")
            },
            wast::Instruction::F32ReinterpretI32 => {
                // no-op
                format!("")
            },
            wast::Instruction::I32ReinterpretF32 => {
                // no-op
                format!("")
            },
            wast::Instruction::F64ConvertI32S => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_convert_i32(self, debug)
            },
            wast::Instruction::F64ConvertI32U => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_convert_i32u(self, debug)
            },
            wast::Instruction::F64ConvertI64U => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_convert_i64u(self, debug)
            },
            wast::Instruction::I32Clz => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_clz(self, debug)
            },
            wast::Instruction::I32Popcnt => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_popcnt(self, debug)
            },
            wast::Instruction::I64Clz => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_clz(self, debug)
            },
            wast::Instruction::I32Ctz => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_ctz(self, debug)
            },
            wast::Instruction::I64Ctz => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_ctz(self, debug)
            },
            // control flow instructions
            wast::Instruction::Block(b) => {
                // if a block doesn't have a label, we have to make one up
                let label: String = match b.label {
                    Some(id) => id.name().to_string().clone(),
                    _ => format!("b{}", block_name_count),
                };
                *block_name_count += 1;
                // for the control stack, we don't use the third parameter for blocks
                control_stack.push((label.to_string(), 0, -1));
                emit_block(&self, b, label, *block_name_count-1, fn_name, function_id_map, debug)
            },
            wast::Instruction::Loop(b) => {
                let label: String = match b.label {
                    Some(id) => id.name().to_string().clone(),
                    _ => format!("l{}", loop_name_count),
                };
                *loop_name_count += 1;
                // the third parameter in the control stack stores loop header entry points
                control_stack.push((label.to_string(), 1, (*call_ret_idx).try_into().unwrap()));
                emit_loop(&self, b, label, *loop_name_count-1, fn_name, function_id_map, call_ret_idx, debug)
            }
            // if control_stack.pop() panics, that means we were parsing an incorrectly defined
            // wasm file, each block/loop must have a matching end!
            wast::Instruction::End(id) => {
                let (label, t, _) = control_stack.pop().unwrap();
                emit_end(&self, id, &label, t, fn_name, function_id_map, debug)
            },
            wast::Instruction::Select(_) => {
                emit_select(self, stack_sizes, fn_name, debug)
            },
            wast::Instruction::MemoryGrow(arg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_mem_grow(self, arg, debug)
            },
            wast::Instruction::MemorySize(arg) => {
                stack_sizes.push(1);
                emit_mem_size(self, arg, debug)
            },
            wast::Instruction::Return => emit_return(self, fn_name, debug),
            wast::Instruction::Br(idx) => emit_br(self, *idx, fn_name, control_stack, function_id_map, debug),
            wast::Instruction::BrIf(idx) => emit_br_if(self, *idx, fn_name, stack_sizes, control_stack, function_id_map, debug),
            wast::Instruction::BrTable(table_idxs) => emit_br_table(self, table_idxs, fn_name, stack_sizes, control_stack, function_id_map, debug),
            wast::Instruction::Unreachable => {
                let skip_label = if debug {
                    false
                } else {
                    true
                };
                self.emit_hypercall(WasmHypercallId::proc_exit, hypercall_id_count, fn_name.to_string(), skip_label, debug)
            },
            _ => panic!("Instruction {:?} not yet implemented, in func: {:?}", instr, func.id)
        }
    }

    fn emit_function(&self,
                     func: &wast::Func,
                     call_ret_map: &mut HashMap<&str, u32>,
                     call_ret_idx: &mut u32,
                     function_id_map: HashMap<&str, u32>,
                     hypercall_id_count: &mut u32,
                     indirect_call_mapping: &HashMap<u32, &wast::Index>, 
                     global_mappings: &HashMap<String, (u32, u32)>,
                     force_inline: bool,
                     debug_call_print: bool,
                     is_gpu: bool,
                     debug: bool) -> String {
        let mut final_string = String::from(""); 
        *call_ret_idx = 0;
        *hypercall_id_count = 0;

        // store the stack offset for all parameters and locals
        let mut local_parameter_stack_offset: HashMap<String, u32> = HashMap::new();
        let mut local_type_info: HashMap<String, ValType> = HashMap::new();

        // Function header
        match (&func.kind, &func.id, &func.ty) {
            (wast::FuncKind::Import(_), _, _) => {
                // In this case, we have an InlineImport of the form:
                // (func (type 3) (import "foo" "bar"))
                panic!("InlineImport functions not yet implemented");
            },
            (wast::FuncKind::Inline{locals, expression}, Some(id), typeuse) => {
                let mut offset = 0;
                let mut param_idx: u32 = 0;
                // get offsets for parameters, we record offsets from the start of the stack frame
                match typeuse.clone().inline {
                    Some(params) => {
                        for parameter in params.params.to_vec() {
                            match parameter {
                                (Some(id), _, t) => {
                                    local_parameter_stack_offset.insert(id.name().to_string(), offset);
                                    local_type_info.insert(id.name().to_string(), t.clone());
                                    offset += self.get_size_valtype(&t);
                                },
                                // if there is no id, we have to name the parameter ourselves!
                                (None, _, t) => {
                                    local_parameter_stack_offset.insert(format!("{}", param_idx), offset);
                                    local_type_info.insert(format!("{}", param_idx), t.clone());
                                    offset += self.get_size_valtype(&t);
                                },
                                _ => panic!("Unhandled parameter type")
                            }
                            param_idx += 1;
                        }
        
                    },
                    None => {
                        ()
                    },
                }

                // store the offset of the parameters (for offset computation later)
                let param_offset: i32 = 0 - offset as i32;

                // we reuse param_idx because that is the default numbering convention in WASM
                // if you have 1 param 1 local, you have p0 then l1

                // get offsets for locals
                for local in locals {
                    let local_id = match local.id {
                        Some(name) => name.name().to_string(),
                        None => format!("{}", param_idx),
                    };
                    local_parameter_stack_offset.insert(local_id.clone(), offset);
                    local_type_info.insert(local_id.clone(), local.ty.clone());
                    
                    offset += self.get_size_valtype(&local.ty);
                    param_idx += 1;
                }

                // Now that we have the type info for the parameters and locals, we can generate the stack context

                // First, generate the stack context for the function
                let stack_ctx = StackCtx::initialize_context(&self, &expression.instrs, &local_type_info);

                // emit the necessary intermediate values
                final_string += &stack_ctx.emit_intermediates();

                // function entry point
                // strip illegal chars from function name
                let inline = if force_inline {
                    format!("inline")
                } else {
                    format!("")
                };

                final_string += &format!("{}{} {{\n", inline,
                        self.generate_function_prelude(&format!("{}{}", "__", id.name().replace(".", "")),
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        false,
                                                        debug));

                if debug_call_print {
                    write!(final_string, "\t\tprintf(\"*sfp = %d\\n\", *sfp);\n");
                    write!(final_string, "\t\tprintf(\"*sp = %d\\n\", *sp);\n");
                    write!(final_string, "\t\tprintf(\"*hypercall_number = %d\\n\", *hypercall_number);\n");
                    write!(final_string, "\t\tprintf(\"*hypercall_continuation = %d\\n\", *hypercall_continuation);\n");
                    write!(final_string, "\t\tprintf(\"read_u32(stack_frames+*sfp) = %d\\n\", read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx));\n");
                    write!(final_string, "\t\tprintf(\"read_u64(call_stack+*sfp) = %d\\n\", read_u64((ulong)(call_stack+*sfp), (ulong)(call_stack), warp_idx));\n");
                }

                /*
                 * First, before emitting the function call & hypercall return tables,
                 * we need to do an analysis pass on the instructions to:
                 * 1) Identify all function calls in the function
                 * 2) Identify all hypercalls in the function
                 * 3) Convert all loop continue statements to recursive function calls
                 * 
                 * To better understand why this pass is necessary, see opencl_writer/control_flow.rs
                 */
                let num_function_calls: &mut u32 = &mut 0;
                let num_hypercalls: &mut u32 = &mut 0;

                for instruction in expression.instrs.iter() {
                    match instruction {
                        wast::Instruction::Call(idx) => {
                            let id = match idx {
                                wast::Index::Id(id) => id.name(),
                                _ => panic!("Unable to get Id for function call: {:?}", idx),
                            };
                            // if the function is a hypercall, incr that count, else incr func call count
                            if self.imports_map.contains_key(id) {
                                *num_hypercalls += 1;
                            } else {
                                *num_function_calls += 1;
                            }
                        },
                        wast::Instruction::CallIndirect(_) => {
                            // we generate a switch-case for each possible function in table $T0
                            // so we have to generate re-entry points for all of them as well
                            for (_, _) in indirect_call_mapping {
                                *num_function_calls += 1;
                            }
                        },
                        wast::Instruction::Unreachable => {
                            // the POCL compiler seems to want this, NVIDIA compiler doesn't?
                            if !is_gpu {
                                *num_hypercalls += 1;
                            }
                        },
                        wast::Instruction::Loop(_) => {
                            // if we find a loop, we will treat the back-branch of each loop
                            // as a function call, see opencl_writer/control_flow.rs for more details on why we do this
                            *num_function_calls += 1;
                        },
                        _ => (),
                    }
                }

                // upon entry, first check to see if we are returning from a hypercall
                // hypercall_number is set to -1 after completing the hypercall

                write!(final_string, "\t{}\n", "if (*hypercall_number == -1) {");
                write!(final_string, "\t\t{}\n", "*hypercall_number = -2;");
                if *num_hypercalls > 0 {
                    write!(final_string, "\t\t{}\n", "switch (*hypercall_continuation) {");
                    for count in 0..*num_hypercalls {
                        write!(final_string, "\t\t\tcase {}:\n", count);
                        if debug_call_print {
                            write!(final_string, "\t\t\t\tprintf(\"goto: {}_hypercall_return_stub_{}\\n\");\n", format!("{}{}", "__", id.name().replace(".", "")), count);
                        }
                        write!(final_string, "\t\t\t\tgoto {}_hypercall_return_stub_{};\n", format!("{}{}", "__", id.name().replace(".", "")), count);
                        write!(final_string, "\t\t\t\tbreak;\n");
                    }
                    write!(final_string, "\t\t}}\n");
                }

                write!(final_string, "\t}}\n");

                // after checking for hypercalls, check if we are unwinding the call stack
                // (returning from another function)

                write!(final_string, "\t{}\n", "if (!*is_calling) {");
                if *num_function_calls > 0 {
                    write!(final_string, "\t\t{}\n",
                        format!("switch ({}) {{", emit_read_u64("(ulong)(call_stack+*sfp)", "(ulong)(call_stack)", "warp_idx")));
                    for count in 0..*num_function_calls {
                        write!(final_string, "\t\t\tcase {}:\n", count);
                        write!(final_string, "\t\t\t\t*sfp -= 1;\n");
                        if debug_call_print {
                            write!(final_string, "\t\t\t\tprintf(\"goto: {}_call_return_stub_{}\\n\");\n", format!("{}{}", "__", id.name().replace(".", "")), count);
                        }
                        write!(final_string, "\t\t\t\tgoto {}_call_return_stub_{};\n", format!("{}{}", "__", id.name().replace(".", "")), count);
                        //write!(final_string, "\t\t\t\tbreak;\n");
                    }
                    write!(final_string, "\t\t}}\n");
                }

                write!(final_string, "\t}}\n");

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
                    final_string += &emit_local(&self, local.clone(), debug);
                }

                // keep a stack of control-flow labels
                // for blocks we need to put the label at the "end" statement, while loops always jump back
                let mut control_stack: Vec<(String, u32, i32)> = vec![];
                
                // keep a stack of the size of previous stack operations
                // this is needed to implement drop/select
                let stack_sizes: &mut Vec<u32> = &mut vec![];

                // used for generating names for anonymous blocks
                let block_name_count: &mut u32  = &mut 0;
                let loop_name_count: &mut u32  = &mut 0;

                // get the list of instructions first, to solve a lifetime mismatch error
                // (we can't just iterate because the control stack would have a different lifetime)

                for instruction in expression.instrs.iter() {
                    final_string += &self.emit_instructions(instruction,
                                                            stack_sizes,
                                                            param_offset,
                                                            &local_parameter_stack_offset,
                                                            &local_type_info,
                                                            call_ret_map,
                                                            call_ret_idx,
                                                            id.name(),
                                                            &mut control_stack,
                                                            function_id_map.clone(),
                                                            hypercall_id_count,
                                                            indirect_call_mapping,
                                                            global_mappings,
                                                            func,
                                                            block_name_count,
                                                            loop_name_count,
                                                            // if we are compiling a CPU kernel
                                                            // we have to force this to true, even if we aren't
                                                            // actually emitting "debug" code
                                                            !is_gpu || debug);
                }

                // If we are emitting the start function, just emit a proc_exit here
                if id.name().to_string() == "_start" {
                    // emit modified func unwind for _start
                    final_string += &function_unwind(&self, id.name(), &typeuse.inline, true, debug);
                    final_string += &self.emit_hypercall(WasmHypercallId::proc_exit, hypercall_id_count, id.name().to_string(), true, debug);
                } else {
                    // to unwind from the function we unwind the call stack by moving the stack pointer
                    // and returning the last value on the stack 
                    final_string += &function_unwind(&self, id.name(), &typeuse.inline, false, debug);
                }
            },
            (_, _, _) => panic!("Inline function must always have a valid identifier in wasm")
        };

        // end function
        final_string += &format!("}}\n");

        final_string
    }

    fn emit_memcpy_arr(&self, debug: bool) -> String {
        let mut result = String::from("");
        let mut counter = 0;
        let mut offset_val = 0;
        let mut arr_len = 0;
        for temp in &self.data {
            result += &format!("\t{}{}[] = {{\n", "uchar data_segment_data_", counter);
            match temp {
                wast::Data {span, id, kind, data} => {
                    match kind {
                        wast::DataKind::Active{memory, offset} => {
                            match offset.instrs[0] {
                                wast::Instruction::I32Const(val) => {
                                    offset_val = val;
                                    result += &String::from("\t\t");
                                    for element in data.concat() {
                                        result += &format!("{},", element);
                                        arr_len += 1;
                                    }
                                },
                                _ => panic!("Unknown data offset value"),
                            }
                        },
                        wast::DataKind::Passive => panic!("wast::Passive datatype kind found"),
                    }
                },
                _ => panic!("Unknown data type"),
            };
            result += &format!("\n\t{}\n", "};");
            // now emit the memcpy instructions to copy to the heap

            result += &format!("\t{}\n", format!("for(uint idx = 0; idx < {}; idx++) {{", arr_len));

            result += &format!("\t\t{}\n",
                format!("write_u8((ulong)((global char*)heap_u32 + {} + idx), (ulong)(heap_u32), data_segment_data_{}[idx], warp_idx);",
                    offset_val,
                    counter));

            result += &String::from("\t}\n");

            arr_len = 0;
            counter += 1;
        }

        result
    }

    fn emit_global_init(&self, global_mappings: &HashMap<String, (u32, u32)>, debug: bool) -> String {
        let mut ret_str = String::from("");

        // needed for case where globals are auto-indexed
        let mut global_count: u32 = 0;

        for global in &self.globals {
            let (offset, _) = match global.id {
                Some(id) => global_mappings.get(id.name()).unwrap(),
                None => global_mappings.get(&format!("g{}", global_count)).unwrap(),
            };
            
            match &global.kind {
                wast::GlobalKind::Inline(expr) => {
                    match &expr.instrs[0] {
                        wast::Instruction::I32Const(val) => {
                            ret_str += &format!("\t{};\n",
                            &emit_write_u32(&format!("(ulong)((global char*)globals_buffer+{})", offset*4),
                                        "(ulong)(globals_buffer)",
                                        &val.to_string(),
                                        "warp_idx"));
                        },
                        _ => panic!("Unknown constant in emit_global_init"),
                    }
                },
                _ => panic!("GlobalKind inlineimport not implemented"), 
            }
            global_count += 1;
        }

        ret_str
    }

    /*
     * There can be multiple elements in a WASM module, so we must enumerate all of them
     * to provide a mapping of table indicies to function names
     */
    fn process_elements(&self, debug: bool) -> HashMap<u32, &wast::Index> {
        let elements_vec = &self.elements;
        let mut table_mapping: HashMap<u32, &wast::Index> = HashMap::new();
        for element in elements_vec {
            match &element.payload {
                wast::ElemPayload::Indices(index_vec) => {
                    let offset: u32 = match &element.kind {
                        wast::ElemKind::Active{table, offset} => {
                            match &offset.instrs[0] {
                                wast::Instruction::I32Const(val) => *val as u32,
                                _ => panic!("Unable to extract offset from WASM module element - unknown offset"),
                            }
                        },
                        _ => panic!("Unable to extract offset from WASM module element!"),
                    };
                    // we now have the element offset, and the index_vec, so we can init the table_mapping
                    let mut starting_offset = offset;
                    for item in index_vec {
                        table_mapping.insert(starting_offset, item);
                        starting_offset += 1;
                    }
                },
                _ => panic!(""),
            }
        }

        table_mapping
    }

    fn generate_function_prelude(&self,
                                 fn_name: &str,
                                 interleave: u32,
                                 stack_size_bytes: u32,
                                 heap_size_bytes: u32,
                                 stack_frames_size_bytes: u32,
                                 call_stack_size_bytes: u32,
                                 predictor_size_bytes: u32,
                                 globals_buffer_size: u32,
                                 stack_frame_ptr_size_bytes: u32,
                                 is_control_fn: bool,
                                 debug: bool) -> String {
        let mut output = String::new();
        /*
         * Generate code for each function in the file first
         */
        if debug {
            // write thread-local private variables before header
            // store branch stack pointers for branch value stack unwinding
            if is_control_fn {
                write!(output, "{} {{\n", format!("void {}(uint   *stack_u32,
                    ulong  *stack_u64,
                    uint   *heap_u32,
                    ulong  *heap_u64,
                    uint   *hypercall_buffer,
                    uint   *globals_buffer,
                    uint   *stack_frames,
                    ulong  *sp,
                    ulong  *sfp,
                    ulong  *call_stack,
                    uint   *call_return_stack,
                    ulong *branch_value_stack_state,
                    ulong *loop_value_stack_state,
                    int    *hypercall_number,
                    uint   *hypercall_continuation,
                    uint   *current_mem_size,
                    uint   *max_mem_size,
                    uchar  *is_calling,
                    ulong  warp_idx,
                    uint   *entry_point,
                    uint   *hcall_ret_val)", fn_name));
            } else {
                write!(output, "{}", format!("void {}(uint   *stack_u32,
                    ulong  *stack_u64,
                    uint   *heap_u32,
                    ulong  *heap_u64,
                    uint   *hypercall_buffer,
                    uint   *globals_buffer,
                    uint   *stack_frames,
                    ulong  *sp,
                    ulong  *sfp,
                    ulong  *call_stack,
                    ulong   *call_return_stack,
                    ulong *branch_value_stack_state,
                    ulong *loop_value_stack_state,
                    int    *hypercall_number,
                    uint   *hypercall_continuation,
                    uint   *current_mem_size,
                    uint   *max_mem_size,
                    uchar  *is_calling,
                    ulong  warp_idx,
                    uint   *entry_point,
                    uint   hcall_ret_val)", fn_name));
            }
        } else if is_control_fn {
            let header = format!("__kernel void {}(__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}) {{\n",
                                    fn_name,
                                    "uint   *stack_u32_global,",
                                    "ulong  *stack_u64_global,",
                                    "uint   *heap_u32_global,",
                                    "ulong  *heap_u64_global,",
                                    "uint   *hypercall_buffer_global,",
                                    "uint   *globals_buffer_global,",
                                    "uint   *stack_frames_global,",
                                    "ulong  *sp_global,",
                                    "ulong  *sfp_global,",
                                    "ulong  *call_stack_global,",
                                    "ulong   *call_return_stack_global,",
                                    "ulong *branch_value_stack_state_global,",
                                    "ulong *loop_value_stack_state_global,",
                                    "int    *hypercall_number_global,",
                                    "uint   *hypercall_continuation_global,",
                                    "uint   *current_mem_size_global,",
                                    "uint   *max_mem_size_global,",
                                    "uchar  *is_calling_global,",
                                    "uint   *entry_point_global,",
                                    "uint   *hcall_ret_val_global");
            // write thread-local private variables before header

            write!(output, "{}", header);
            // TODO: for the openCL launcher, pass the memory stride as a function parameter
            if interleave > 0 {
                write!(output, "\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\n\t{}\n",
                "global uint  *stack_u32    = (global uint*)stack_u32_global;",
                "global ulong *stack_u64    = (global ulong*)stack_u32;",
                "global uint  *heap_u32     = (global uint *)heap_u32_global;",
                "global ulong *heap_u64     = (global ulong *)heap_u32;",
                "global uint  *hypercall_buffer = (global uint *)hypercall_buffer_global;",
                "global uint   *globals_buffer = (global uint *)globals_buffer_global;",
                "global uint  *stack_frames = (global uint*)stack_frames_global;",
                // only an array of N elements, where N=warp size
                "global ulong *sp           = (global ulong *)sp_global+(get_global_id(0));",
                // the stack frame pointer is used for both the stack frame, and call stack as they are
                // essentially the same structure, except they hold different values
                "global ulong *sfp          = (global ulong*)(sfp_global+(get_global_id(0)));",
                // holds the numeric index of the return label for where to jump after a function call
                "global ulong *call_stack   = (global ulong*)call_stack_global;",
                "global ulong *call_return_stack   = (global ulong*)call_return_stack_global;",
                "global ulong *branch_value_stack_state   = (global ulong*)branch_value_stack_state_global;",
                "global ulong *loop_value_stack_state   = (global ulong*)loop_value_stack_state_global;",
                "global int *hypercall_number = (global int *)hypercall_number_global+(get_global_id(0));",
                "global uint *hypercall_continuation = (global uint *)hypercall_continuation_global+(get_global_id(0));",
                "global uint *current_mem_size = (global uint *)current_mem_size_global+(get_global_id(0));",
                "global uint *max_mem_size = (global uint *)max_mem_size_global+(get_global_id(0));",
                "global uchar *is_calling = (global uchar *)is_calling_global+(get_global_id(0));",
                "global uint  *entry_point   = (global uint*)entry_point_global+get_global_id(0);",
                "ulong warp_idx = get_global_id(0);",
                "global uint  *hcall_ret_val = (global uint*)hcall_ret_val_global+get_global_id(0);");
            } else {
                // The pointer math must be calculated in terms of bytes, which is why we cast to (char*) first
                write!(output, "\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\n\t{}\n",
                format!("global uint  *stack_u32    = (global uint*)((global char*)stack_u32_global+(get_global_id(0) * {}));", stack_size_bytes),
                "global ulong *stack_u64    = (global ulong*)stack_u32;",
                format!("global uint  *heap_u32     = (global uint *)((global char*)heap_u32_global+(get_global_id(0) * {}));", heap_size_bytes),
                "global ulong *heap_u64     = (global ulong *)heap_u32;",
                // the hypercall_buffer is hardcoded to always be 16KiB - we can change this later if needed possibly
                "global uint  *hypercall_buffer = (global uint *)((global char*)hypercall_buffer_global+(get_global_id(0) * 1024*16));",
                format!("global uint  *globals_buffer = (global uint*)((global char*)globals_buffer_global+(get_global_id(0) * {}));", globals_buffer_size * 4),
                format!("global uint  *stack_frames = (global uint*)((global char*)stack_frames_global+(get_global_id(0) * {}));", stack_frames_size_bytes),
                // only an array of N elements, where N=warp size
                "global ulong *sp           = (global ulong *)sp_global+(get_global_id(0));",
                // the stack frame pointer is used for both the stack frame, and call stack as they are
                // essentially the same structure, except they hold different values
                format!("global ulong *sfp          = (global ulong*)((global char*)sfp_global+(get_global_id(0) * {}));", stack_frame_ptr_size_bytes),
                // holds the numeric index of the return label for where to jump after a function call
                format!("global ulong *call_stack   = (global ulong*)((global char*)call_stack_global+(get_global_id(0) * {}));", call_stack_size_bytes),
                format!("global ulong *call_return_stack   = (global ulong*)((global char*)call_return_stack_global+(get_global_id(0) * {}));", call_stack_size_bytes),
                format!("global ulong *branch_value_stack_state   = (global ulong*)((global char*)branch_value_stack_state_global+(get_global_id(0) * {}));", predictor_size_bytes),
                format!("global ulong *loop_value_stack_state   = (global ulong*)((global char*)loop_value_stack_state_global+(get_global_id(0) * {}));", predictor_size_bytes),
                "global int *hypercall_number = (global int *)hypercall_number_global+(get_global_id(0));",
                "global uint *hypercall_continuation = (global uint *)hypercall_continuation_global+(get_global_id(0));",
                "global uint *current_mem_size = (global uint *)current_mem_size_global+(get_global_id(0));",
                "global uint *max_mem_size = (global uint *)max_mem_size_global+(get_global_id(0));",
                "global uchar *is_calling = (global uchar *)is_calling_global+(get_global_id(0));",
                "global uint  *entry_point   = (global uint *)entry_point_global+get_global_id(0);",
                "ulong warp_idx = get_global_id(0);",
                "global uint  *hcall_ret_val = (global uint*)hcall_ret_val_global+get_global_id(0);");
            }
        // if we are an OpenCL kernel and we are not the control function, we only need the function header itself
        } else {
            write!(output, "{}", format!("
inline void {}(global uint   *stack_u32,
    global ulong  *stack_u64,
    global uint   *heap_u32,
    global ulong  *heap_u64,
    global uint   *hypercall_buffer,
    global uint   *globals_buffer,
    global uint   *stack_frames,
    global ulong  *sp,
    global ulong  *sfp,
    global ulong  *call_stack,
    global ulong  *call_return_stack,
    global ulong *branch_value_stack_state,
    global ulong *loop_value_stack_state,
    global int    *hypercall_number,
    global uint   *hypercall_continuation,
    global uint   *current_mem_size,
    global uint   *max_mem_size,
    global uchar  *is_calling,
    ulong  warp_idx,
    global uint   *entry_point,
    uint hcall_ret_val)", fn_name));
        }
        output
    }

    // WASM expects all memory (stack, heap) to be zero initialized
    fn zero_init_memory(&self) -> String {
        let mut ret_str = String::from("");

        // zero the stack first
        ret_str += &format!("{}{}{}",
                            "\tfor (uint idx = 0; idx < (VMM_STACK_SIZE_BYTES / 4); idx++) {\n",
                            format!("\t\t{};\n", &emit_write_u32("(ulong)(stack_u32+idx)", "(ulong)(stack_u32)", "0", "warp_idx")),
                            "\t}\n");

        // zero the heap next
        ret_str += &format!("{}{}{}",
                            "\tfor (uint idx = 0; idx < (VMM_HEAP_SIZE_BYTES / 4); idx++) {\n",
                            format!("\t\t{};\n", &emit_write_u32("(ulong)(heap_u32+idx)", "(ulong)(heap_u32)", "0", "warp_idx")),
                            "\t}\n");

        ret_str
    }

    /*
     * This function generates the helper kernel that loads the data sections
     * It is also ressponsible for loading globals into memory id -> (offset, size)
     */
    fn generate_data_section(&self, interleave: u32, heap_size: u32, debug: bool) -> (String, HashMap<String, (u32, u32)>) {
        let mut result = String::from("");
        let mut mapping: HashMap<String, (u32, u32)> = HashMap::new();
        let mut offset: u32 = 0;
        // needed if globals are referred to using numerical indexes
        let mut global_id: u32 = 0;
        for global in &self.globals {
            match global {
                wast::Global{span, id, exports, ty, kind} => {
                    let id = match id {
                        Some(id) => String::from(id.name()),
                        None => format!("g{}", global_id),
                    };

                    let type_size = self.get_size_valtype(&ty.ty);
                    mapping.insert(id, (offset, type_size.clone()));
                    offset += type_size;
                },
                _ => panic!("Uknown global kind found"),
            }
            global_id += 1;
        }

        let (program_start_mem_pages, program_start_max_pages) = match self.memory.get(0) {
            Some(mem) => match mem.kind {
                wast::MemoryKind::Normal(memtype) => {
                    match memtype {
                        wast::MemoryType::B32{limits, shared} => {
                            let max = match limits.max {
                                Some(val) => val,
                                None => heap_size/(1024*64),
                            };
                            (limits.min as u64, max as u64)
                        },
                        wast::MemoryType::B64{limits, shared} => {
                            let max = match limits.max {
                                Some(val) => val,
                                None => heap_size as u64/(1024*64),
                            };
                            (limits.min, max)
                        },
                    }
                },
                _ => (1 as u64, heap_size as u64/(1024*64)),
            },
            None => (1 as u64, heap_size as u64/(1024*64)),
        };

        dbg!(program_start_mem_pages);
        dbg!(program_start_max_pages);

        if debug {
            result += &String::from("\nvoid data_init(uint *stack_u32, uint *heap_u32, uint *globals_buffer, uint *curr_mem, uint *max_mem, uchar *is_calling, ulong *sfp) {\n");
            result += &String::from("\tulong warp_idx = 0;\n");
            // each page = 64KiB
            result += &format!("\tcurr_mem[warp_idx] = {};\n", program_start_mem_pages);
            result += &String::from("\tis_calling[warp_idx] = 1;\n");
            result += &format!("\tmax_mem[warp_idx] = {};\n", program_start_max_pages);
            result += &format!("\t{};\n",
                        "global ulong *sfp = (global ulong *)sfp_global+(get_global_id(0))");

            result += &format!("\t{};\n",
                               "*sfp = 0");
            
            result += &self.zero_init_memory();
            result += &self.emit_memcpy_arr(debug);
            result += &self.emit_global_init(&mapping, debug);

            result += &String::from("}\n\n");
        } else {
            result += &String::from("\n__kernel void data_init(__global uint *stack_u32_global, __global uint *heap_u32_global, __global uint *globals_buffer_global, __global uint *curr_mem_global, __global uint *max_mem_global, __global uchar *is_calling_global, __global ulong *sfp_global) {\n");
            result += &String::from("\tulong warp_idx = get_global_id(0);\n");
            // these structures are not interleaved, so its fine to just read/write them as is
            // they are already implicitly interleaved (like sp for example)
            result += &format!("\tcurr_mem_global[warp_idx] = {};\n", program_start_mem_pages);
            result += &String::from("\tis_calling_global[warp_idx] = 1;\n");
            result += &format!("\tmax_mem_global[warp_idx] = {};\n", program_start_max_pages);
            result += &format!("\t{};\n",
                               "global ulong *sfp = (global ulong *)sfp_global+(get_global_id(0))");

            result += &format!("\t{};\n",
                               "*sfp = 0");

            if interleave == 0 {
                result += &format!("\t{}\n",
                                   format!("global uint *heap_u32 = (global uint *)((global char*)heap_u32_global+(get_global_id(0) * VMM_HEAP_SIZE_BYTES));"));
                result += &format!("\t{}\n",
                                   format!("global uint *stack_u32 = (global uint *)((global char*)stack_u32_global+(get_global_id(0) * VMM_STACK_SIZE_BYTES));"));

                result += &format!("\t{}\n",
                                   format!("global uint *globals_buffer = (global uint *)((global char*)globals_buffer_global+(get_global_id(0) * {}));", offset * 4));
            
                } else {
                result += &format!("\t{}\n",
                                    format!("global uint *heap_u32 = (global uint *)(heap_u32_global);"));
                result += &format!("\t{}\n",
                                    format!("global uint *stack_u32 = (global uint *)(stack_u32_global);"));

                result += &format!("\t{}\n",
                                    format!("global uint *globals_buffer = (global uint *)(globals_buffer_global);"));
            }

            result += &self.zero_init_memory();
            result += &self.emit_memcpy_arr(debug);
            result += &self.emit_global_init(&mapping, debug);

            result += &String::from("}\n\n");
        }
        (result, mapping)
    }

    // This function generates helper functions for performing reads/writes to the stack/heap
    // we call these functions right before returning back to the VMM, and on re-entry
    fn generate_hypercall_helpers(&self, debug: bool) -> String {
        let mut result = String::from("");

        // for each hypercall that we can identify in the import section, generate the read_helper
        // we have to special case proc_exit, since we also call proc_exit if we hit an unreachable statement
        for (_, value) in &self.imports_map {
            match value {
                (_, hypercall_name, _) => result += &emit_hypercall_helpers(self, *hypercall_name, debug),
                _ => (),
            }
        }
        result
    }

    fn emit_wasm_control_fn(&self,
                            interleave: u32,
                            stack_size_bytes: u32,
                            heap_size_bytes: u32,
                            call_stack_size_bytes: u32,
                            stack_frames_size_bytes: u32,
                            stack_frame_ptr_size_bytes: u32, 
                            predictor_size_bytes: u32,
                            debug_print_function_calls: bool,
                            globals_buffer_size: u32,
                            function_idx_label: HashMap<&str, u32>,
                            debug: bool) -> String {
        let mut ret_str = String::from("");
        write!(ret_str, "{}",
                self.generate_function_prelude("wasm_entry",
                                               interleave,
                                               stack_size_bytes,
                                               heap_size_bytes,
                                               stack_frames_size_bytes,
                                               call_stack_size_bytes,
                                               predictor_size_bytes,
                                               globals_buffer_size,
                                               stack_frame_ptr_size_bytes,
                                               true,
                                               debug));
        write!(ret_str, "\tstack_u32 += {};\n", 128);
        if debug_print_function_calls {
            write!(ret_str, "\tprintf(\"stack_u32: %p\\n\", stack_u32);\n");
            write!(ret_str, "\tprintf(\"heap_u32: %p\\n\", heap_u32);\n");
            write!(ret_str, "\tprintf(\"is_calling: %p\\n\", is_calling);\n");
            write!(ret_str, "\tprintf(\"hypercall_buffer: %p\\n\", hypercall_buffer);\n");
            write!(ret_str, "\tprintf(\"call_stack: %p\\n\", call_stack);\n");
            write!(ret_str, "\tprintf(\"stack_frames: %p\\n\", stack_frames);\n");
            write!(ret_str, "\tprintf(\"call_return_stack: %p\\n\", call_return_stack);\n");
            write!(ret_str, "\tprintf(\"branch_value_stack_state: %p\\n\", branch_value_stack_state);\n");
            write!(ret_str, "\tprintf(\"loop_value_stack_state: %p\\n\", loop_value_stack_state);\n");
            write!(ret_str, "\tprintf(\"hypercall_number: %p\\n\", hypercall_number);\n");
            write!(ret_str, "\tprintf(\"hypercall_continuation: %p\\n\", hypercall_continuation);\n");
            write!(ret_str, "\tprintf(\"current_mem_size: %p\\n\", current_mem_size);\n");
            write!(ret_str, "\tprintf(\"max_mem_size: %p\\n\", max_mem_size);\n");
        }

        write!(ret_str, "\t{}\n", "do {");
        write!(ret_str, "\t{}\n", "switch (*entry_point) {");
        for key in function_idx_label.keys() {
            write!(ret_str, "\t\tcase {}:\n", function_idx_label.get(key).unwrap());
            if debug_print_function_calls {
                write!(ret_str, "\t\tprintf(\"{}\\n\");\n", format!("{}{}", "__", key.replace(".", "")));
            }
            // strip illegal chars from function names
            write!(ret_str, "\t\t\t{}({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});\n",
                            format!("{}{}", "__", key.replace(".", "")),
                            "stack_u32",
                            "stack_u64",
                            "heap_u32",
                            "heap_u64",
                            "hypercall_buffer",
                            "globals_buffer",
                            "stack_frames",
                            "sp",
                            "sfp",
                            "call_stack",
                            "call_return_stack",
                            "branch_value_stack_state",
                            "loop_value_stack_state",
                            "hypercall_number",
                            "hypercall_continuation",
                            "current_mem_size",
                            "max_mem_size",
                            "is_calling",
                            "warp_idx",
                            "entry_point",
                            "*hcall_ret_val");
            write!(ret_str, "\t\t\tbreak;\n");
        }
        write!(ret_str, "\t\tdefault:\n");
        if debug_print_function_calls {
            write!(ret_str, "\t\t\tprintf(\"{}\\n\");\n", "taking default case");
        }
        write!(ret_str, "\t\t\treturn;\n");
        write!(ret_str, "\t}}\n");

        // if we reset the hypercall_number, that means we need to exit back to the VMM
        write!(ret_str, "\t{}\n", "} while (*sfp != 0 && *hypercall_number == -2);");

        write!(ret_str, "}}\n");

        ret_str
    }

    pub fn write_opencl_file(&self,
                             interleave: u32,
                             stack_size_bytes: u32,
                             heap_size_bytes: u32,
                             call_stack_size_bytes: u32,
                             stack_frames_size_bytes: u32,
                             stack_frame_ptr_size_bytes: u32, 
                             predictor_size_bytes: u32,
                             debug_print_function_calls: bool,
                             force_inline: bool,
                             is_gpu: bool,
                             debug: bool) -> (String, u32, u32, u32, HashMap<u32, String>) {
        let mut output = String::new();
        let mut header = String::new();
        let mut func_vec = Vec::new();
        let mut kernel_hashmap: HashMap<u32, String> = HashMap::new();
        //let mut output = File::create(filename).unwrap();

        // if we are running in debug C-mode, we must define the openCL types
        if debug {
            write!(output, "{}", format!("#include <stdlib.h>\n"));
            write!(output, "{}", format!("#include \"../includes/wasm_hypercall.h\"\n"));

            write!(output, "{}", format!("#define uchar unsigned char\n"));
            write!(output, "{}", format!("#define ulong unsigned long\n"));
            write!(output, "{}", format!("#define uint unsigned int\n"));
            write!(output, "{}", format!("#define ushort unsigned short\n"));
        }

        // enable the usage of FP64 operations (double precision floats)
        // if we are unable to enable, floating point calculations may be incorrect
        write!(output, "{}",
r#"
#ifdef cl_khr_fp64
#pragma OPENCL EXTENSION cl_khr_fp64 : enable
#elif defined(cl_amd_fp64)
#pragma OPENCL EXTENSION cl_amd_fp64 : enable
#endif
"#);

        // generate the read/write functions
        // we support only either a 1 byte interleave, or no interleave
        // 0 = no interleave
        write!(output, "{}", generate_read_write_calls(&self, interleave, debug));
        write!(header, "{}", generate_read_write_calls(&self, interleave, debug));

        // generate the hypercall helper section
        write!(output, "{}", self.generate_hypercall_helpers(debug));
        write!(header, "{}", self.generate_hypercall_helpers(debug));

        let prelude_header = output.clone();

        // generate the data loading function
        // also return the global mappings: global id -> (global buffer offset, global size)
        let (data_section, global_mappings) = self.generate_data_section(interleave, heap_size_bytes, debug);
        let mut globals_buffer_size = 0;

        for (_key, (_offset, size)) in &global_mappings {
            globals_buffer_size += size;
        }

        let data_program = format!("{}\n{}", prelude_header, data_section.clone());
        kernel_hashmap.insert(99999, data_program);
        write!(output, "{}", data_section);

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

        // hypercall ID tracker count, we use this number to auto-insert
        // the return stub later
        let hypercall_id_count: &mut u32 = &mut 0;

        let call_ret_map: &mut HashMap<&str, u32> = &mut HashMap::new();
        let mut call_ret_idx: u32 = 0;

        // generate the indirect call mapping T0, refer to openclwriter/functions.rs:emit_call_indirect
        // for notes on why we are doing this statically at compile time
        let indirect_call_mapping: &HashMap<u32, &wast::Index> = &self.process_elements(debug);

        for function in funcs.clone() {
            let func = self.emit_function(function,
                                          call_ret_map,
                                          &mut call_ret_idx,
                                          function_idx_label.clone(),
                                          hypercall_id_count,
                                          indirect_call_mapping,
                                          &global_mappings,
                                          force_inline,
                                          debug_print_function_calls,
                                          is_gpu,
                                          debug);

            if debug {
                //write!(output, "{}", func);
            }
            
            write!(output, "{}", func);

            let fname = match (&function.kind, &function.id, &function.ty) {
                (wast::FuncKind::Import(_), _, _) => {
                    panic!("InlineImport functions not yet implemented");
                },
                (wast::FuncKind::Inline{locals, expression}, Some(id), typeuse) => {
                    id.name()
                },
                (_, _, _) => panic!("Inline function must always have a valid identifier in wasm")
            };

            let mut function_idx_label_temp: HashMap<&str, u32> = HashMap::new();
            let fname_idx = function_idx_label.get(fname).unwrap();
            function_idx_label_temp.insert(fname, *fname_idx);
            let control_function = self.emit_wasm_control_fn(interleave,
                                                            stack_size_bytes,
                                                            heap_size_bytes,
                                                            call_stack_size_bytes,
                                                            stack_frames_size_bytes,
                                                            stack_frame_ptr_size_bytes, 
                                                            predictor_size_bytes,
                                                            debug_print_function_calls,
                                                            globals_buffer_size,
                                                            function_idx_label_temp,
                                                            debug);
            let func_full = format!("{}\n{}\n{}\n", prelude_header.clone(), func.clone(), control_function); 

            kernel_hashmap.insert(*fname_idx, func_full);

            // if we are going to try linking a lib
            //write!(output, "{};", self.generate_function_prelude(&format!("{}{}", "__", fname.replace(".", "")), interleave, 0, 0, 0, 0, 0, 0, 0, false, false));
            
            let header_include = format!("#include \"helper.cl\"\n{}", func);
            func_vec.push(header_include);
        }
        

        // generate control function prelude
        write!(output, "{}",
                self.generate_function_prelude("wasm_entry",
                                               interleave,
                                               stack_size_bytes,
                                               heap_size_bytes,
                                               stack_frames_size_bytes,
                                               call_stack_size_bytes,
                                               predictor_size_bytes,
                                               globals_buffer_size,
                                               stack_frame_ptr_size_bytes,
                                               true,
                                               debug));

        /*
         * Set up the stack frame for the first function call (___start)
         * we need to make sure that stack_u32 is set up properly, so that parameters for the first function being called can be
         * accessed. This is because of our indexing for function params (negative offsets)
         * 
         * This is language specific:
         *  wasi-libc takes no args for start, see: https://github.com/WebAssembly/wasi-libc/blob/master/libc-bottom-half/crt/crt1.c
         * 
         *  However, rust does: https://github.com/rust-lang/rust/blob/0d97f7a96877a96015d70ece41ad08bb7af12377/library/std/src/rt.rs#L60
         * 
         */

        // to solve this issue, we just increment the stack_u32 ptr by the size of params for the start function
        // this is inefficient, to search through all the functions until we find start, but not really that slow in the grand scheme of things
        for function in funcs.clone() {
            match function.id {
                Some(name) => {
                    if name.name() == "_start" {
                        // move stack_u32 by the total size of all parameters
                        /*
                        let mut offset = 0;
                        match function.ty.clone().inline {
                            Some(params) => {
                                for parameter in params.params.to_vec() {
                                    match parameter {
                                        (Some(id), _, t) => {
                                            offset += self.get_size_valtype(&t);
                                        },
                                        // if there is no id, we have to name the parameter ourselves!
                                        (None, _, t) => {
                                            offset += self.get_size_valtype(&t);
                                        },
                                        _ => panic!("Unhandled parameter type")
                                    }
                                }
                
                            },
                            None => (),
                        }
                        */
                        write!(output, "\tstack_u32 += {};\n", 128);
                        break;
                    }
                },
                _ => (),
            }
        }

        if debug_print_function_calls {
            write!(output, "\tprintf(\"stack_u32: %p\\n\", stack_u32);\n");
            write!(output, "\tprintf(\"heap_u32: %p\\n\", heap_u32);\n");
            write!(output, "\tprintf(\"is_calling: %p\\n\", is_calling);\n");
            write!(output, "\tprintf(\"hypercall_buffer: %p\\n\", hypercall_buffer);\n");
            write!(output, "\tprintf(\"call_stack: %p\\n\", call_stack);\n");
            write!(output, "\tprintf(\"stack_frames: %p\\n\", stack_frames);\n");
            write!(output, "\tprintf(\"call_return_stack: %p\\n\", call_return_stack);\n");
            write!(output, "\tprintf(\"branch_value_stack_state: %p\\n\", branch_value_stack_state);\n");
            write!(output, "\tprintf(\"loop_value_stack_state: %p\\n\", loop_value_stack_state);\n");
            write!(output, "\tprintf(\"hypercall_number: %p\\n\", hypercall_number);\n");
            write!(output, "\tprintf(\"hypercall_continuation: %p\\n\", hypercall_continuation);\n");
            write!(output, "\tprintf(\"current_mem_size: %p\\n\", current_mem_size);\n");
            write!(output, "\tprintf(\"max_mem_size: %p\\n\", max_mem_size);\n");
            write!(output, "\tprintf(\"globals_buffer: %p\\n\", globals_buffer);\n");
        }

        write!(output, "\t{}\n", "do {");
        write!(output, "\t{}\n", "switch (*entry_point) {");
        for key in function_idx_label.keys() {
            write!(output, "\t\tcase {}:\n", function_idx_label.get(key).unwrap());
            if debug_print_function_calls {
                write!(output, "\t\tprintf(\"{}\\n\");\n", format!("{}{}", "__", key.replace(".", "")));
            }
            // strip illegal chars from function names
            write!(output, "\t\t\t{}({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});\n",
                            format!("{}{}", "__", key.replace(".", "")),
                            "stack_u32",
                            "stack_u64",
                            "heap_u32",
                            "heap_u64",
                            "hypercall_buffer",
                            "globals_buffer",
                            "stack_frames",
                            "sp",
                            "sfp",
                            "call_stack",
                            "call_return_stack",
                            "branch_value_stack_state",
                            "loop_value_stack_state",
                            "hypercall_number",
                            "hypercall_continuation",
                            "current_mem_size",
                            "max_mem_size",
                            "is_calling",
                            "warp_idx",
                            "entry_point",
                            "*hcall_ret_val");
            write!(output, "\t\t\tbreak;\n");
        }
        write!(output, "\t\tdefault:\n");
            write!(output, "\t\t\treturn;\n");
        write!(output, "\t}}\n");

        // if we reset the hypercall_number, that means we need to exit back to the VMM
        write!(output, "\t{}\n", "} while (*sfp != 0 && *hypercall_number == -2);");

        write!(output, "}}\n");

        if debug {
            write!(output, "{}", format!("int main(int argc, char *argv[]) {{\n"));
            
            write!(output, "{}", format!("\t{}\n", "uvwasi_t uvwasi;"));
            write!(output, "{}", format!("\t{}\n", "uvwasi_options_t init_options;"));
            write!(output, "{}", format!("\t{}\n", "uvwasi_options_init(&init_options);"));
            write!(output, "{}", format!("\t{}\n", "uvwasi_init(&uvwasi, &init_options);"));
            write!(output, "{}", format!("\tuint *stack_u32 = calloc(1024*1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tuint *globals = calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tuint *hcall_buf = calloc(1024*16, sizeof(uint));\n"));
            write!(output, "{}", format!("\tulong *stack_u64 = (ulong *)stack_u32;\n"));
            write!(output, "{}", format!("\tuint *heap_u32 = (uint *)calloc(1024*1024*16, sizeof(uint));\n"));
            write!(output, "{}", format!("\tulong *heap_u64 = (ulong *)calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tuint *stack_frames = calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tuint *call_stack = calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tuint *call_return_stack = calloc(1024, sizeof(uint));\n"));

            // the size of this structure is proportional to how many functions there are
            // size = function count * 4096 bytes (64 x 64)
            write!(output, "{}", format!("\tuchar *branch_value_stack_state = calloc({}, sizeof(uchar));\n", funcs.len() * 4096 * 2));
            write!(output, "{}", format!("\tuchar *loop_value_stack_state = calloc({}, sizeof(uchar));\n", funcs.len() * 4096 * 2));

            write!(output, "{}", format!("\tulong sp = 0;\n"));
            write!(output, "{}", format!("\tulong sfp = 0;\n"));
            write!(output, "{}", format!("\tuchar is_calling = 1;\n"));

            write!(output, "{}", format!("\tulong entry_point = {};\n", function_idx_label.get("_start").unwrap()));

            write!(output, "{}", format!("\tint hypercall_number = -2;\n"));
            write!(output, "{}", format!("\tuint hypercall_continuation = 0;\n"));
            write!(output, "{}", format!("\tuint curr_mem[1024];\n"));
            write!(output, "{}", format!("\tuint max_mem[1024];\n"));
            write!(output, "{}", format!("\tstack_frames[sfp] = sp;\n"));
            write!(output, "{}", format!("\tstack_u64[0] = 0x1;\n"));
            write!(output, "{}", format!("\tsp += 2;\n"));
            write!(output, "{}", format!("\tdata_init(heap_u32, globals, curr_mem, max_mem, &is_calling);\n"));

            // now we are entering the main execution loop, continue until *sp == 0
            write!(output, "{}", format!("\twhile(1) {{\n"));

            write!(output, "{}", format!("{}",
                    format!("\t\twasm_entry(stack_u32, stack_u64, heap_u32, heap_u64, hcall_buf, globals, stack_frames, &sp, &sfp, call_stack, call_return_stack, branch_value_stack_state, loop_value_stack_state, &hypercall_number, &hypercall_continuation, curr_mem, max_mem, &is_calling, 0, &entry_point);\n")));

                    // if *sp == 0, break
                    write!(output, "{}", format!("\t\tif (sp == 0) {{\n"));
                    write!(output, "{}", format!("\t\t\tbreak;\n"));

                    // elif hypercall waiting -> dispatch hypercall
                    write!(output, "{}", format!("\t\t}} else {{\n"));
                    // process the hypercall
                    write!(output, "{}", format!("\t\t\t{}\n", "printf(\"hypercall: %d\\n\", hypercall_number);"));
                    // hypercall_number == 1, corresponds to proc_exit
                    write!(output, "{}", format!("\t\t\t{}\n", "switch (hypercall_number) {"));
                    write!(output, "{}", format!("\t\t\t\t{}\n", "case 0:"));
                    write!(output, "{}", format!("\t\t\t\t\t{}\n", "vmm_fd_write(&uvwasi, stack_u32, &sp, heap_u32);"));
                    write!(output, "{}", format!("\t\t\t\t\t{}\n", "break;"));
                    write!(output, "{}", format!("\t\t\t\t{}\n", "case 1:"));
                    write!(output, "{}", format!("\t\t\t\t\t{}\n", "goto exit;"));
                    write!(output, "{}", format!("\t\t\t\t\t{}\n", "break;"));
                    write!(output, "{}", format!("\t\t\t{}\n", "}"));
                    // after processing, reset hypercall_number to -1 to reenter the continuation
                    write!(output, "{}", format!("\t\t\t{}\n", "hypercall_number = -1;"));
                    write!(output, "{}", format!("\t\t}}\n"));

           
            write!(output, "{}", format!("\t}}\n"));

            write!(output, "{}", format!("exit:\n"));

            // now check the result
            write!(output, "{}", format!("\tprintf(\"%d\\n\", stack_u32[sp]);\n"));

            write!(output, "}}\n\n");
        }

        (output, *function_idx_label.get("_start").unwrap(), globals_buffer_size, funcs.len().try_into().unwrap(), kernel_hashmap)
    }
}

impl fmt::Debug for OpenCLCWriter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenCLCWriter")
        .field("types", &self.types)
        .field("imports", &self.imports_map)
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
