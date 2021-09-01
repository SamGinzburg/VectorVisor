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
mod fastcalls;
mod compile_stats;
mod cfg_optimizer;
mod trap;

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
use fastcalls::*;
use compile_stats::*;
use cfg_optimizer::*;
use trap::*;

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
use std::iter::FromIterator;
use std::collections::HashSet;

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
pub enum WasmHypercallId {
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
    imports_map: HashMap<String, (&'a str, Option<&'a str>, wast::ItemSig<'a>)>,
    // map of item.id -> (module, field)
    func_map: HashMap<String, wast::Func<'a>>,
    func_names: Vec<String>,
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
            func_names: vec!(),
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
        let mut func_name_count = 0;
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
                                Some(id) => {
                                    self.imports_map.insert(id.name().to_string(), (i.module, i.field, i.item));
                                },
                                None => {
                                    self.imports_map.insert(format!("func_{}", func_name_count), (i.module, i.field, i.item));
                                    func_name_count += 1;
                                }
                            };
                        },
                        wast::ModuleField::Func(f) => {
                            match f.id {
                                Some(f_id) => {
                                    self.func_map.insert(f_id.name().to_string(), f);
                                    self.func_names.push(f_id.name().to_string());
                                },
                                None => {
                                    self.func_map.insert(format!("func_{}", func_name_count), f);
                                    self.func_names.push(format!("func_{}", func_name_count));
                                    func_name_count += 1;
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

    fn emit_hypercall(&self, hypercall_id: WasmHypercallId, stack_ctx: &mut StackCtx, hypercall_id_count: &mut u32, fn_name: String, is_proc_exit_start: bool, debug: bool) -> String {
        let mut ret_str = String::from("");

        // We need to save the context early, because we are going to need the stack parameters after returning...
        // But only if we are actually going to return to avoid overwriting ret vals
        if !is_proc_exit_start {
            ret_str += &stack_ctx.save_context(false, false);
        }

        // set the hypercall ret flag flag + r
        ret_str += &format!("\t{}\n", format!("*hypercall_number = {};", hypercall_id.clone() as u32));

        // run the hypercall setup code - to marshall data for the VMM to read

        // hypercalls that are omitted from this table are implied to not require any data transfer via the hcall buffer 
        match hypercall_id {
            WasmHypercallId::fd_write => ret_str += &emit_fd_write_call_helper(self, stack_ctx, debug),
            WasmHypercallId::fd_prestat_get => ret_str += &emit_fd_prestat_get_helper(self, stack_ctx, debug),
            WasmHypercallId::fd_prestat_dir_name => ret_str += &emit_fd_prestat_dir_name_helper(self, stack_ctx, debug),
            WasmHypercallId::serverless_invoke => ret_str += &emit_serverless_invoke_pre(self, debug),
            WasmHypercallId::serverless_response => ret_str += &emit_serverless_response_pre(self, stack_ctx, debug),
            WasmHypercallId::random_get => ret_str += &emit_random_get_pre(self, stack_ctx, debug),
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
            // increment hypercall_id_count, IFF we are counting it
            *hypercall_id_count += 1;
        }

        // restore the contex
        if !is_proc_exit_start {
            ret_str += &stack_ctx.restore_context(false, false);
        }

        // after the hypercall, we need to reset values on re-entry, and possible copy data back from the hcall buf
        // skipped hypercall entries here are no-ops
        match hypercall_id {
            WasmHypercallId::fd_write => {
                ret_str += &emit_fd_write_post(&self, stack_ctx, debug);
            },
            WasmHypercallId::environ_sizes_get => {
                ret_str += &emit_environ_sizes_get_post(&self, stack_ctx, debug);
            },
            WasmHypercallId::environ_get => {
                ret_str += &emit_environ_get_post(&self, stack_ctx, debug);
            },
            WasmHypercallId::fd_prestat_get => {
                ret_str += &emit_fd_prestat_get_post(&self, stack_ctx, debug);
            },
            WasmHypercallId::fd_prestat_dir_name => {
                ret_str += &emit_fd_prestat_dir_name_post(&self, stack_ctx, debug);
            },
            WasmHypercallId::serverless_invoke => {
                ret_str += &emit_serverless_invoke_post(&self, stack_ctx, debug);
            },
            WasmHypercallId::serverless_response => {
                ret_str += &emit_serverless_response_post(&self, stack_ctx, debug);
            },
            WasmHypercallId::random_get => {
                ret_str += &emit_random_get_post(&self, stack_ctx, debug);
            },
            _ => (),
        }

        ret_str
    }

    fn emit_instructions(&self,
                         // instruction to emit
                         instr: &wast::Instruction,
                         // sizes of current stack items
                         stack_sizes: &mut Vec<u32>,
                         // the stack context (needed for stack allocs)
                         stack_ctx: &mut StackCtx,
                         // the offset of parameters on the current callstack
                         parameter_offset: i32,
                         // map of local/parameter IDs to offset from stack frame start
                         offsets: &HashMap<String, u32>,
                         type_info: &HashMap<String, ValType>,
                         // keeps track of local vs. parameter
                         is_param: &HashMap<String, bool>,
                         call_ret_map: &mut HashMap<&str, u32>,
                         call_ret_idx: &mut u32,
                         // function name
                         fn_name: &str,
                         // track "_start" entry point
                         start_function_name: String,
                         // stack of control flow operations (blocks, loops)
                         control_stack: &mut Vec<ControlStackEntryType>,
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
                         if_name_count: &mut u32,
                         loop_name_count: &mut u32,
                         call_indirect_count: &mut u32,
                         fastcall_set: &HashSet<String>,
                         // emit an optimized function that does not require a CPS-style transformation
                         is_fastcall: bool,
                         // emit OpenCL C (False) or standard C for debugging on the CPU (True)
                         debug: bool) -> String {
        match instr {
            wast::Instruction::Nop => {
                // No-op
                String::from("")
            },
            wast::Instruction::Drop => {
                // based on the previous stack size, decrement sp
                // we don't need to handle all cases, only the common case provided by LLVM output
                // which is when drop follows a function call
                stack_sizes.pop().unwrap();
                let dropped_type = stack_ctx.vstack_peak_type(0);
                stack_ctx.vstack_pop(dropped_type);
                String::from("")
            }
            wast::Instruction::I32Store(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore_i32(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I32Store8(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore8_i32(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I64Store8(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore8_i64(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I64Store16(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore16_i64(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I32Store16(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore16_i32(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I64Store32(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore32_i64(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I32Load(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_i32(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I32Load8u(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_i32_8u(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I64Load16u(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_memload_i64_16u(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I32Load16u(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_i32_16u(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I32Load16s(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_i32_16s(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I32Load8s(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_i32_8s(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I64Load8u(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_memload_i64_8u(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I64Load32u(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_memload_i64_32u(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I64Load32s(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_memload_i64_32s(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I64Load(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_memload_i64(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::F64Load(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_memload_f64(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::F32Load(memarg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_memload_f32(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::I64Store(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore_i64(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::F64Store(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore_f64(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::F32Store(memarg) => {
                stack_sizes.pop();
                stack_sizes.pop();
                emit_memstore_f32(self, stack_ctx, memarg, debug)
            },
            wast::Instruction::GlobalGet(idx) => {
                match idx {
                    wast::Index::Id(id) => {
                        emit_global_get(self, stack_ctx, id.name(), global_mappings, stack_sizes, debug)
                    },
                    wast::Index::Num(value, _) => {
                        emit_global_get(self, stack_ctx, &format!("g{}", value), global_mappings, stack_sizes, debug)
                    },
                }
            },
            wast::Instruction::GlobalSet(idx) => {
                match idx {
                    wast::Index::Id(id) => {
                        emit_global_set(self, stack_ctx, id.name(), global_mappings, stack_sizes, debug)
                    },
                    wast::Index::Num(value, _) => {
                        emit_global_set(self, stack_ctx, &format!("g{}", value), global_mappings, stack_sizes, debug)
                    },
                }
            },
            wast::Instruction::I32Const(val) => {
                stack_sizes.push(1);
                emit_i32_const(self, stack_ctx, val, debug)
            },
            wast::Instruction::I64Const(val) => {
                stack_sizes.push(2);
                emit_i64_const(self, stack_ctx, val, debug)
            },
            wast::Instruction::F32Const(val) => {
                stack_sizes.push(1);
                emit_f32_const(self, stack_ctx, &val.bits, debug)
            },
            wast::Instruction::F64Const(val) => {
                stack_sizes.push(2);
                emit_f64_const(self, stack_ctx, &val.bits, debug)
            },
            wast::Instruction::LocalGet(idx) => {
                match idx {
                    wast::Index::Id(id) => {
                        emit_local_get(self, stack_ctx, parameter_offset, id.name(), offsets, type_info, stack_sizes, debug)
                    },
                    wast::Index::Num(value, _) => {
                        let id = match is_param.get(&format!("l{}", value)) {
                            Some(false) => {
                                format!("l{}", value)
                            },
                            Some(true) => format!("p{}", value),
                            _ => format!("p{}", value),
                        };
                        emit_local_get(self, stack_ctx, parameter_offset, &id, offsets, type_info, stack_sizes, debug)
                    },
                }
            },
            wast::Instruction::LocalSet(idx) => {
                match idx {
                    wast::Index::Id(id) => {
                        emit_local_set(self, stack_ctx, parameter_offset, id.name(), offsets, type_info, stack_sizes, is_fastcall, debug)
                    },
                    wast::Index::Num(value, _) => {
                        let id = match is_param.get(&format!("l{}", value)) {
                            Some(false) => {
                                format!("l{}", value)
                            },
                            Some(true) => format!("p{}", value),
                            _ => format!("p{}", value),
                        };
                        emit_local_set(self, stack_ctx, parameter_offset, &id, offsets, type_info, stack_sizes, is_fastcall, debug)
                    },
                }
            },
            wast::Instruction::LocalTee(idx) => {
                match idx {
                    wast::Index::Id(id) => {
                        emit_local_tee(self, stack_ctx, parameter_offset, id.name(), offsets, type_info, stack_sizes, is_fastcall, debug)
                    },
                    wast::Index::Num(value, _) => {
                        let id = match is_param.get(&format!("l{}", value)) {
                            Some(false) => {
                                format!("l{}", value)
                            },
                            Some(true) => format!("p{}", value),
                            _ => format!("p{}", value),
                        };
                        emit_local_tee(self, stack_ctx, parameter_offset, &id, offsets, type_info, stack_sizes, is_fastcall, debug)
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
                emit_i32_add(self, stack_ctx, debug)
            },
            wast::Instruction::I32Mul => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_mul(self, stack_ctx, debug)
            },
            wast::Instruction::I64Mul => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_mul(self, stack_ctx, debug)
            },
            wast::Instruction::I32Sub => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_sub(self, stack_ctx, debug)
            },
            wast::Instruction::I64Add => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_add(self, stack_ctx, debug)
            },
            wast::Instruction::F32Trunc => {
                emit_f32_trunc(self, stack_ctx, debug)
            },
            wast::Instruction::F64Trunc => {
                emit_f64_trunc(self, stack_ctx, debug)
            },
            wast::Instruction::F64Add => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_add(self, stack_ctx, debug)
            },
            wast::Instruction::F64Max => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_max(self, stack_ctx, debug)
            },
            wast::Instruction::F64Min => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_min(self, stack_ctx, debug)
            },
            wast::Instruction::F32Max => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_max(self, stack_ctx, debug)
            },
            wast::Instruction::F32Min => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_min(self, stack_ctx, debug)
            },
            wast::Instruction::F64Sub => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_sub(self, stack_ctx, debug)
            },
            wast::Instruction::F32Add => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_add(self, stack_ctx, debug)
            },
            wast::Instruction::F32Sub => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_sub(self, stack_ctx, debug)
            },
            wast::Instruction::F32Mul => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_mul(self, stack_ctx, debug)
            },
            wast::Instruction::F64Div => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_div(self, stack_ctx, debug)
            },
            wast::Instruction::F32Div => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_div(self, stack_ctx, debug)
            },
            wast::Instruction::F64Mul => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_mul(self, stack_ctx, debug)
            },
            wast::Instruction::F64Neg => {
                emit_f64_neg(self, stack_ctx, debug)
            },
            wast::Instruction::F32Neg => {
                emit_f32_neg(self, stack_ctx, debug)
            },
            wast::Instruction::F32Abs => {
                emit_f32_abs(self, stack_ctx, debug)
            },
            wast::Instruction::F64Abs => {
                emit_f64_abs(self, stack_ctx, debug)
            },
            wast::Instruction::F64Ne => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f64_ne(self, stack_ctx, debug)
            },
            wast::Instruction::F32Ne => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_ne(self, stack_ctx, debug)
            },
            wast::Instruction::F64Lt => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f64_lt(self, stack_ctx, debug)
            },
            wast::Instruction::F32Gt => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_gt(self, stack_ctx, debug)
            },
            wast::Instruction::F32Lt => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_lt(self, stack_ctx, debug)
            },
            wast::Instruction::F64Le => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f64_le(self, stack_ctx, debug)
            },
            wast::Instruction::F64Ge => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f64_ge(self, stack_ctx, debug)
            },
            wast::Instruction::F64Gt => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f64_gt(self, stack_ctx, debug)
            },
            wast::Instruction::F32Le => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_le(self, stack_ctx, debug)
            },
            wast::Instruction::F32Ge => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_ge(self, stack_ctx, debug)
            },
            wast::Instruction::I64LtU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_lt_u(self, stack_ctx, debug)
            },
            wast::Instruction::I64Eq => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_eq(self, stack_ctx, debug)
            },
            wast::Instruction::F64Eq => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f64_eq(self, stack_ctx, debug)
            },
            wast::Instruction::F32Eq => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_eq(self, stack_ctx, debug)
            },
            wast::Instruction::I32TruncF64U => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_trunc_f64_u(self, stack_ctx, debug)
            },
            wast::Instruction::I64Ne => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_ne(self, stack_ctx, debug)
            },
            wast::Instruction::I64DivU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_div_u(self, stack_ctx, debug)
            },
            wast::Instruction::I32Eqz => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_eqz(self, stack_ctx, debug)
            },
            wast::Instruction::I64Eqz => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_eqz(self, stack_ctx, debug)
            },
            wast::Instruction::I32And => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_and(self, stack_ctx, debug)
            },
            wast::Instruction::I64And => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_and(self, stack_ctx, debug)
            },
            wast::Instruction::I32Ne => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_ne(self, stack_ctx, debug)
            },
            wast::Instruction::I32LtU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_lt_u(self, stack_ctx, debug)
            },
            wast::Instruction::I32LtS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_lt_s(self, stack_ctx, debug)
            },
            wast::Instruction::I64LtS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_lt_s(self, stack_ctx, debug)
            },
            wast::Instruction::I32GtU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_gt_u(self, stack_ctx, debug)
            },
            wast::Instruction::I64GtU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_gt_u(self, stack_ctx, debug)
            },
            wast::Instruction::I64GtS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_gt_s(self, stack_ctx, debug)
            },
            wast::Instruction::I32GtS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_gt_s(self, stack_ctx, debug)
            },
            wast::Instruction::I32LeU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_le_u(self, stack_ctx, debug)
            },
            wast::Instruction::I32LeS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_le_s(self, stack_ctx, debug)
            },
            wast::Instruction::I64LeU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_le_u(self, stack_ctx, debug)
            },
            wast::Instruction::I64LeS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_le_s(self, stack_ctx, debug)
            },
            wast::Instruction::I32GeU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_ge_u(self, stack_ctx, debug)
            },
            wast::Instruction::I32GeS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_ge_s(self, stack_ctx, debug)
            },
            wast::Instruction::I64GeU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_ge_u(self, stack_ctx, debug)
            },
            wast::Instruction::I64GeS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i64_ge_s(self, stack_ctx, debug)
            },
            wast::Instruction::I32Xor => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_xor(self, stack_ctx, debug)
            },
            wast::Instruction::I32WrapI64 => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_wrap_i64(self, stack_ctx, debug)
            },
            wast::Instruction::I64ExtendI32S => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_extend_i32_s(self, stack_ctx, debug)
            },
            wast::Instruction::I64ExtendI32U => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_extend_i32_u(self, stack_ctx, debug)
            },
            wast::Instruction::Call(idx) => {
                let id = &match idx {
                    wast::Index::Id(id) => id.name().to_string(),
                    wast::Index::Num(val, _) => format!("func_{}", val),
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
                                        &"fd_write"               => self.emit_hypercall(WasmHypercallId::fd_write, stack_ctx, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"proc_exit"              => self.emit_hypercall(WasmHypercallId::proc_exit, stack_ctx, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"environ_sizes_get"      => self.emit_hypercall(WasmHypercallId::environ_sizes_get, stack_ctx, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"environ_get"            => self.emit_hypercall(WasmHypercallId::environ_get, stack_ctx, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"fd_prestat_get"         => self.emit_hypercall(WasmHypercallId::fd_prestat_get, stack_ctx, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"fd_prestat_dir_name"    => self.emit_hypercall(WasmHypercallId::fd_prestat_dir_name, stack_ctx, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"random_get"             => self.emit_hypercall(WasmHypercallId::random_get, stack_ctx, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"serverless_invoke"      => self.emit_hypercall(WasmHypercallId::serverless_invoke, stack_ctx, hypercall_id_count, fn_name.to_string(), false, debug),
                                        &"serverless_response"    => self.emit_hypercall(WasmHypercallId::serverless_response, stack_ctx, hypercall_id_count, fn_name.to_string(), false, debug),
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
                            let _func_type_signature = &self.func_map.get(id).unwrap().ty;

                            // We emit fastcalls either if the function itself is a fastcall, or if we are a CPS-style function making a fastcall
                            let make_fastcall = is_fastcall || fastcall_set.contains(id);
                            emit_fn_call(&self, stack_ctx, fn_name.to_string(), *idx, call_ret_map, call_ret_idx, &function_id_map, stack_sizes, false, make_fastcall, String::from(""), vec![], debug)
                        },
                        // we have an import that isn't a system call...
                        None => String::from("")
                    }
                }
            },
            wast::Instruction::CallIndirect(call_indirect) => {
                // Check for types
                let call_indirect_type_index = match (call_indirect.ty.index.as_ref(), call_indirect.ty.inline.as_ref()) {
                    (Some(index), _) => {
                        // if we have an index, we need to look it up in the global structure
                        let type_index = match index {
                            wast::Index::Num(n, _) => format!("t{}", n),
                            wast::Index::Id(i) => i.name().to_string(),
                        };

                        type_index
                    },
                    (_, Some(_inline)) => panic!("Inline types for call_indirect not implemented yet (main pass opencl_writer.rs)"),
                    _ => panic!("Unable to find types for call_indirect (main pass opencl_writer.rs)"),
                };
                emit_call_indirect(&self, stack_ctx, call_indirect, fn_name.to_string(), fastcall_set, indirect_call_mapping, call_ret_map, call_ret_idx, call_indirect_count, function_id_map, stack_sizes, call_indirect_type_index, debug)
            },
            wast::Instruction::I32Eq => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_eq(self, stack_ctx, debug)
            },
            wast::Instruction::I32Or => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_or(self, stack_ctx, debug)
            },
            wast::Instruction::I32ShrU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_shr_u(self, stack_ctx, debug)
            },
            wast::Instruction::I64ShrU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_shr_u(self, stack_ctx, debug)
            },
            wast::Instruction::I32ShrS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_shr_s(self, stack_ctx, debug)
            },
            wast::Instruction::I32Shl => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_shl(self, stack_ctx, debug)
            },
            wast::Instruction::I64Shl => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_shl(self, stack_ctx, debug)
            },
            wast::Instruction::I32DivU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_div_u(self, stack_ctx, debug)
            },
            wast::Instruction::I32DivS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_div_s(self, stack_ctx, debug)
            },
            wast::Instruction::I64DivS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_div_s(self, stack_ctx, debug)
            },
            wast::Instruction::I32RemU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_rem_u(self, stack_ctx, debug)
            },
            wast::Instruction::I64RemU => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_rem_u(self, stack_ctx, debug)
            },
            wast::Instruction::I32RemS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_rem_s(self, stack_ctx, debug)
            },
            wast::Instruction::I64RemS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_rem_s(self, stack_ctx, debug)
            },
            wast::Instruction::I64ShrS => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_shr_s(self, stack_ctx, debug)
            },
            wast::Instruction::I64Xor => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_xor(self, stack_ctx, debug)
            },
            wast::Instruction::I64Or => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_or(self, stack_ctx, debug)
            },
            wast::Instruction::I32Rotl => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_rotl(self, stack_ctx, debug)
            },
            wast::Instruction::I64Rotl => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_rotl(self, stack_ctx, debug)
            },
            wast::Instruction::I64Sub => {
                stack_sizes.pop();
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_sub(self, stack_ctx, debug)
            },
            wast::Instruction::I64ReinterpretF64 => {
                emit_i64_reinterpret_f64(self, stack_ctx, debug)
            },
            wast::Instruction::F64ReinterpretI64 => {
                emit_f64_reinterpret_i64(self, stack_ctx, debug)
            },
            wast::Instruction::F32ReinterpretI32 => {
                emit_f32_reinterpret_i32(self, stack_ctx, debug)
            },
            wast::Instruction::I32ReinterpretF32 => {
                emit_i32_reinterpret_f32(self, stack_ctx, debug)
            },
            wast::Instruction::F64Ceil => {
                emit_f64_ceil(self, stack_ctx, debug)
            },
            wast::Instruction::F32Ceil => {
                emit_f32_ceil(self, stack_ctx, debug)
            },
            wast::Instruction::F64Floor => {
                emit_f64_floor(self, stack_ctx, debug)
            },
            wast::Instruction::F32Floor => {
                emit_f32_floor(self, stack_ctx, debug)
            },
            wast::Instruction::F64PromoteF32 => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_promote_f32(self, stack_ctx, debug)
            },
            wast::Instruction::F32DemoteF64 => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_demote_f64(self, stack_ctx, debug)
            },
            wast::Instruction::F64ConvertI32S => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_convert_i32(self, stack_ctx, debug)
            },
            wast::Instruction::F64ConvertI32U => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_convert_i32u(self, stack_ctx, debug)
            },
            wast::Instruction::F32ConvertI32U => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_convert_i32u(self, stack_ctx, debug)
            },
            wast::Instruction::F32ConvertI32S => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_f32_convert_i32s(self, stack_ctx, debug)
            },
            wast::Instruction::F64ConvertI64U => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_convert_i64u(self, stack_ctx, debug)
            },
            wast::Instruction::F64ConvertI64S => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_f64_convert_i64s(self, stack_ctx, debug)
            },
            wast::Instruction::I32TruncF32U => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_trunc_f32_u(self, stack_ctx, debug)
            },
            wast::Instruction::I64TruncF32U => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_trunc_f32_u(self, stack_ctx, debug)
            },
            wast::Instruction::I64TruncF32S => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_trunc_f32_s(self, stack_ctx, debug)
            },
            wast::Instruction::I32TruncF32S => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_trunc_f32_s(self, stack_ctx, debug)
            },
            wast::Instruction::I32Clz => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_clz(self, stack_ctx, debug)
            },
            wast::Instruction::I32Popcnt => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_popcnt(self, stack_ctx, debug)
            },
            wast::Instruction::I64Clz => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_clz(self, stack_ctx, debug)
            },
            wast::Instruction::I32Ctz => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_i32_ctz(self, stack_ctx, debug)
            },
            wast::Instruction::I64Ctz => {
                stack_sizes.pop();
                stack_sizes.push(2);
                emit_i64_ctz(self, stack_ctx, debug)
            },
            // control flow instructions
            wast::Instruction::If(b) => {
                let label: String = match b.label {
                    Some(id) => id.name().to_string().clone(),
                    _ => format!("if{}", if_name_count),
                };
    
                emit_if(&self, label, fn_name.to_string(), b, control_stack, if_name_count, stack_ctx)
            },
            wast::Instruction::Else(_) => {
                emit_else(&self, fn_name.to_string(), control_stack, stack_ctx)
            },
            wast::Instruction::Block(b) => {
                // if a block doesn't have a label, we have to make one up
                let label: String = match b.label {
                    Some(id) => id.name().to_string().clone(),
                    _ => format!("b{}", block_name_count),
                };

                // Get the type of the block
                let block_type = get_func_result(&self, &b.ty);
                // Allocate a register to store the result in after the block exits, if we have one
                // We pop this value back during the corresponding `end` instruction, since WASM does not allow hanging values
                let result_register = match block_type {
                    Some(StackType::i32) => {
                        Some(stack_ctx.vstack_alloc(StackType::i32))
                    },
                    Some(StackType::i64) => {
                        Some(stack_ctx.vstack_alloc(StackType::i64))
                    },
                    Some(StackType::f32) => {
                        Some(stack_ctx.vstack_alloc(StackType::f32))
                    },
                    Some(StackType::f64) => {
                        Some(stack_ctx.vstack_alloc(StackType::f64))
                    },
                    None => None,
                };

                // for the control stack, we don't use the third parameter for blocks
                control_stack.push((label.to_string(), 0, -1, *block_name_count, block_type, result_register));
                *block_name_count += 1;
                emit_block(&self, stack_ctx, b, label, *block_name_count-1, fn_name, function_id_map, is_fastcall, debug)
            },
            wast::Instruction::Loop(b) => {
                // Check the loop index to see if we can optimize the loop
                let is_tainted = stack_ctx.is_loop_tainted((*loop_name_count).try_into().unwrap());

                let label: String = match b.label {
                    Some(id) => id.name().to_string().clone(),
                    _ => format!("l{}", loop_name_count),
                };

                emit_loop(&self, stack_ctx, control_stack, b, label, loop_name_count, fn_name, function_id_map, call_ret_idx, is_fastcall, is_tainted, debug)
            }
            // if control_stack.pop() panics, that means we were parsing an incorrectly defined
            // wasm file, each block/loop must have a matching end!
            wast::Instruction::End(id) => {
                let (label, t, _, loop_idx, result_type, result_register) = control_stack.pop().unwrap();

                emit_end(&self, stack_ctx, id, &label, t, fn_name, result_type, result_register, is_fastcall, loop_idx, debug)
            },
            wast::Instruction::Select(_) => {
                emit_select(self, stack_ctx, stack_sizes, fn_name, debug)
            },
            wast::Instruction::MemoryGrow(arg) => {
                stack_sizes.pop();
                stack_sizes.push(1);
                emit_mem_grow(self, stack_ctx, arg, debug)
            },
            wast::Instruction::MemorySize(arg) => {
                stack_sizes.push(1);
                emit_mem_size(self, stack_ctx, arg, debug)
            },
            wast::Instruction::Return => emit_return(self, stack_ctx, fn_name, start_function_name, hypercall_id_count, is_fastcall, debug),
            wast::Instruction::Br(idx) => emit_br(self, stack_ctx, *idx, fn_name, control_stack, function_id_map, is_fastcall, false, debug),
            wast::Instruction::BrIf(idx) => emit_br_if(self, stack_ctx, *idx, fn_name, stack_sizes, control_stack, function_id_map, is_fastcall, debug),
            wast::Instruction::BrTable(table_idxs) => emit_br_table(self, stack_ctx, table_idxs, fn_name, stack_sizes, control_stack, function_id_map, is_fastcall, debug),
            wast::Instruction::Unreachable => {
                if is_fastcall {
                    // if we are in a fastcall, just dereference some invalid memory address
                    emit_trap(TrapCode::TrapUnreachable, true)
                } else {
                    let skip_label = if debug {
                        false
                    } else {
                        true
                    };
                    self.emit_hypercall(WasmHypercallId::proc_exit, stack_ctx, hypercall_id_count, fn_name.to_string(), skip_label, debug)
                }
            },
            _ => panic!("Instruction {:?} not yet implemented, in func: {:?}", instr, func.id)
        }
    }

    fn emit_function(&self,
                     func: &wast::Func,
                     fn_name: String,
                     call_ret_map: &mut HashMap<&str, u32>,
                     call_ret_idx: &mut u32,
                     function_id_map: HashMap<&str, u32>,
                     hypercall_id_count: &mut u32,
                     local_work_group: usize,
                     indirect_call_mapping: &HashMap<u32, &wast::Index>, 
                     global_mappings: &HashMap<String, (u32, u32)>,
                     fastcall_set: HashSet<String>,
                     force_inline: bool,
                     debug_call_print: bool,
                     start_function: String,
                     is_gpu: bool,
                     reduction_size: &mut u32,
                     is_fastcall: bool,
                     debug: bool) -> (String, u32, HashSet<String>) {
        let mut final_string = String::from("");
        let func_intermediate_size;
        let fastfunc_calls;

        *call_ret_idx = 0;
        *hypercall_id_count = 0;

        // store the stack offset for all parameters and locals
        let mut local_parameter_stack_offset: HashMap<String, u32> = HashMap::new();
        let mut local_type_info: HashMap<String, ValType> = HashMap::new();
        let mut is_param: HashMap<String, bool> = HashMap::new();

        // Function header
        match (&func.kind, &func.id, &func.ty) {
            (wast::FuncKind::Import(_), _, _) => {
                // In this case, we have an InlineImport of the form:
                // (func (type 3) (import "foo" "bar"))
                panic!("InlineImport functions not yet implemented");
            },
            (wast::FuncKind::Inline{locals, expression}, _, typeuse) => {
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
                                    is_param.insert(id.name().to_string(), true);
                                    offset += self.get_size_valtype(&t);
                                },
                                // if there is no id, we have to name the parameter ourselves!
                                (None, _, t) => {
                                    local_parameter_stack_offset.insert(format!("p{}", param_idx), offset);
                                    local_type_info.insert(format!("p{}", param_idx), t.clone());
                                    is_param.insert(format!("p{}", param_idx), true);
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
                        None => format!("l{}", param_idx),
                    };
                    local_parameter_stack_offset.insert(local_id.clone(), offset);
                    local_type_info.insert(local_id.clone(), local.ty.clone());
                    is_param.insert(local_id.clone(), false);

                    offset += self.get_size_valtype(&local.ty);
                    param_idx += 1;
                }

                // Now that we have the type info for the parameters and locals, we can generate the stack context
                // First, generate the stack context for the function
                let mut stack_ctx = StackCtx::initialize_context(&self, &expression.instrs, &local_type_info, &local_parameter_stack_offset, &is_param, fastcall_set.clone(), param_offset, indirect_call_mapping, fn_name.clone(), reduction_size, local_work_group, is_gpu);
                fastfunc_calls = stack_ctx.called_fastcalls();

                // function entry point
                // strip illegal chars from function name
                let inline = if force_inline {
                    format!("inline")
                } else {
                    format!("")
                };

                if !is_fastcall {
                    final_string += &format!("{}{} {{\n", inline,
                    self.generate_function_prelude(&format!("{}{}", "__", fn_name.replace(".", "")),
                                                    0,
                                                    0,
                                                    0,
                                                    0,
                                                    0,
                                                    0,
                                                    0,
                                                    0,
                                                    0,
                                                    local_work_group,
                                                    false,
                                                    debug));
                } else {
                    // Generate the fastcall header
                    let ret_type = match typeuse.clone().inline {
                        Some(ret) => {
                            if ret.results.len() > 0 {
                                Some(ret.results[0])
                            } else {
                                None
                            }
                        },
                        _ => None,
                    };

                    let ret_signature = match ret_type {
                        Some(wast::ValType::I32) => String::from("uint"),
                        Some(wast::ValType::I64) => String::from("ulong"),
                        Some(wast::ValType::F32) => String::from("float"),
                        Some(wast::ValType::F64) => String::from("double"),
                        _ => String::from("void"),
                    };

                    let func_name_demangle = format!("{}{}", "__", fn_name.replace(".", ""));
                    final_string += &format!("{} {}_fastcall{}", ret_signature, func_name_demangle, stack_ctx.emit_fastcall_header());
                }

                // emit the local/parameter cacheing array, this is used to elide writes during ctx saves/restores
                let (cache_arr, local_cache_size) = stack_ctx.emit_cache_array(is_fastcall);
                final_string += &cache_arr;

                // emit the necessary intermediate values
                final_string += &stack_ctx.emit_intermediates(is_fastcall, local_work_group);

                if debug_call_print && !is_fastcall {
                    write!(final_string, "\t\tprintf(\"*sfp = %d\\n\", *sfp);\n").unwrap();
                    write!(final_string, "\t\tprintf(\"*sp = %d\\n\", *sp);\n").unwrap();
                    write!(final_string, "\t\tprintf(\"*hypercall_number = %d\\n\", *hypercall_number);\n").unwrap();
                    write!(final_string, "\t\tprintf(\"*hypercall_continuation = %d\\n\", *hypercall_continuation);\n").unwrap();
                    write!(final_string, "\t\tprintf(\"read_u32(stack_frames+*sfp) = %d\\n\", read_u32((ulong)(stack_frames+*sfp), (ulong)stack_frames, warp_idx));\n").unwrap();
                    write!(final_string, "\t\tprintf(\"read_u64(call_stack+*sfp) = %d\\n\", read_u64((ulong)(call_stack+*sfp), (ulong)(call_stack), warp_idx));\n").unwrap();
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
                let (num_function_calls, num_hypercalls) = stack_ctx.get_reentry_stub_lengths();


                // upon entry, first check to see if we are returning from a hypercall
                // hypercall_number is set to -1 after completing the hypercall
                if !is_fastcall {
                    write!(final_string, "\t{}\n", "if (*hypercall_number == -1) {").unwrap();
                    write!(final_string, "\t\t{}\n", "*hypercall_number = -2;").unwrap();
                    if num_hypercalls > 0 {
                        write!(final_string, "\t\t{}\n", "switch (*hypercall_continuation) {").unwrap();
                        for count in 0..num_hypercalls {
                            write!(final_string, "\t\t\tcase {}:\n", count).unwrap();
                            if debug_call_print {
                                write!(final_string, "\t\t\t\tprintf(\"goto: {}_hypercall_return_stub_{}\\n\");\n", format!("{}{}", "__", fn_name.replace(".", "")), count).unwrap();
                            }
                            write!(final_string, "\t\t\t\tgoto {}_hypercall_return_stub_{};\n", format!("{}{}", "__", fn_name.replace(".", "")), count).unwrap();
                            write!(final_string, "\t\t\t\tbreak;\n").unwrap();
                        }
                        write!(final_string, "\t\t}}\n").unwrap();
                    }
    
                    write!(final_string, "\t}}\n").unwrap();
    
                    // after checking for hypercalls, check if we are unwinding the call stack
                    // (returning from another function)
    
                    write!(final_string, "\t{}\n", "if (!*is_calling) {").unwrap();
                    if num_function_calls > 0 {
                        write!(final_string, "\t\t{}\n",
                            format!("switch ({}) {{", emit_read_u64("(ulong)(call_stack+*sfp)", "(ulong)(call_stack)", "warp_idx"))).unwrap();
                        for count in 0..num_function_calls {
                            write!(final_string, "\t\t\tcase {}:\n", count).unwrap();
                            write!(final_string, "\t\t\t\t*sfp -= 1;\n").unwrap();
                            if debug_call_print {
                                write!(final_string, "\t\t\t\tprintf(\"goto: {}_call_return_stub_{}\\n\");\n", format!("{}{}", "__", fn_name.replace(".", "")), count).unwrap();
                            }
                            write!(final_string, "\t\t\t\tgoto {}_call_return_stub_{};\n", format!("{}{}", "__", fn_name.replace(".", "")), count).unwrap();
                            //write!(final_string, "\t\t\t\tbreak;\n");
                        }
                        write!(final_string, "\t\t}}\n").unwrap();
                    }
    
                    write!(final_string, "\t}} else {{\n").unwrap();
                    // If we are running the func for the first time, init param intermediates
                    final_string += &stack_ctx.emit_load_params(debug_call_print);
                    write!(final_string, "\t}}\n").unwrap();
                }

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

                if !is_fastcall {
                    for local in locals {
                        final_string += &emit_local(&self, local.clone(), debug);
                    }
                }

                if !is_fastcall {
                    // Allocate space on the stack for saving the intermediate context
                    final_string += &format!("\t*sp += {};\n", stack_ctx.max_stack_frame_size());
                    // Allocate space on the stack for storing the local_cache
                    final_string += &format!("\t*sp += {};\n", local_cache_size);
                }

                // keep a stack of control-flow labels
                // for blocks we need to put the label at the "end" statement, while loops always jump back
                let mut control_stack: Vec<ControlStackEntryType> = vec![];

                // keep a stack of the size of previous stack operations
                // this is needed to implement drop/select
                let stack_sizes: &mut Vec<u32> = &mut vec![];

                // used for generating names for anonymous blocks
                let block_name_count: &mut u32  = &mut 0;
                let if_name_count: &mut u32  = &mut 0;
                let loop_name_count: &mut u32  = &mut 0;

                // used for tracking fastcall optimizations for call_indirect
                let call_indirect_count: &mut u32  = &mut 0;

                // get the list of instructions first, to solve a lifetime mismatch error
                // (we can't just iterate because the control stack would have a different lifetime)

                for instruction in expression.instrs.iter() {
                    final_string += &self.emit_instructions(instruction,
                                                            stack_sizes,
                                                            &mut stack_ctx,
                                                            param_offset,
                                                            &local_parameter_stack_offset,
                                                            &local_type_info,
                                                            &is_param,
                                                            call_ret_map,
                                                            call_ret_idx,
                                                            &fn_name,
                                                            start_function.clone(),
                                                            &mut control_stack,
                                                            function_id_map.clone(),
                                                            hypercall_id_count,
                                                            indirect_call_mapping,
                                                            global_mappings,
                                                            func,
                                                            block_name_count,
                                                            if_name_count,
                                                            loop_name_count,
                                                            call_indirect_count,
                                                            &fastcall_set,
                                                            is_fastcall,
                                                            // if we are compiling a CPU kernel
                                                            // we have to force this to true, even if we aren't
                                                            // actually emitting "debug" code
                                                            !is_gpu || debug);
                }

                // The size of all the locals plus the intermediates
                func_intermediate_size = (offset + stack_ctx.get_max_emitted_context() as u32) * 4; // *4 to convert to bytes

                // If we are emitting the start function, just emit a proc_exit here
                if fn_name == start_function {
                    // emit modified func unwind for _start
                    final_string += &function_unwind(&self, &mut stack_ctx, &fn_name, &typeuse.inline, true, is_fastcall, debug);
                    final_string += &self.emit_hypercall(WasmHypercallId::proc_exit, &mut stack_ctx, hypercall_id_count, fn_name, true, debug);
                } else {
                    // to unwind from the function we unwind the call stack by moving the stack pointer
                    // and returning the last value on the stack 
                    final_string += &function_unwind(&self, &mut stack_ctx, &fn_name, &typeuse.inline, false, is_fastcall, debug);
                }
            },
            (_, _, _) => panic!("Inline function must always have a valid identifier in wasm")
        };

        // end function
        final_string += &format!("}}\n");

        (final_string, func_intermediate_size, fastfunc_calls)
    }

    fn emit_memcpy_arr(&self, _debug: bool) -> String {
        let mut result = String::from("");
        let mut counter = 0;
        let mut offset_val;
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

    fn emit_global_init(&self, global_mappings: &HashMap<String, (u32, u32)>, _debug: bool) -> String {
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
    fn process_elements(&self, _debug: bool) -> HashMap<u32, &wast::Index> {
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
                                 _hcall_size: u32,
                                 interleave: u32,
                                 stack_size_bytes: u32,
                                 heap_size_bytes: u32,
                                 stack_frames_size_bytes: u32,
                                 call_stack_size_bytes: u32,
                                 _predictor_size_bytes: u32,
                                 globals_buffer_size: u32,
                                 stack_frame_ptr_size_bytes: u32,
                                 local_work_group: usize,
                                 is_control_fn: bool,
                                 debug: bool) -> String {
        let mut output = String::new();
        /*
         * Generate code for each function in the file first
         */
        if debug {
            // write thread-local private variables before header
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
                    int    *hypercall_number,
                    uint   *hypercall_continuation,
                    uint   *current_mem_size,
                    uint   *max_mem_size,
                    uchar  *is_calling,
                    ulong  warp_idx,
                    ulong  thread_idx,
                    uint   hcall_size,
                    uint   *entry_point,
                    uint   *hcall_ret_val)", fn_name)).unwrap();
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
                    int    *hypercall_number,
                    uint   *hypercall_continuation,
                    uint   *current_mem_size,
                    uint   *max_mem_size,
                    uchar  *is_calling,
                    ulong  warp_idx,
                    ulong  thread_idx,
                    uint   hcall_size,
                    uint   *entry_point,
                    uint   hcall_ret_val)", fn_name)).unwrap();
            }
        } else if is_control_fn {
            let work_group = if local_work_group == 999999 {
                String::from("")
            } else {
                //format!("__attribute__((reqd_work_group_size({}, 1, 1)))", local_work_group)
                String::from("")
            };
            let header = format!("__kernel {} void {}(__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}) {{\n",
                                    work_group,
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
                                    "int    *hypercall_number_global,",
                                    "uint   *hypercall_continuation_global,",
                                    "uint   *current_mem_size_global,",
                                    "uint   *max_mem_size_global,",
                                    "uchar  *is_calling_global,",
                                    "uint   *entry_point_global,",
                                    "uint   *hcall_ret_val_global,",
                                    "uint   *hcall_size_global");
            // write thread-local private variables before header

            write!(output, "{}", header).unwrap();
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
                "global int *hypercall_number = (global int *)hypercall_number_global+(get_global_id(0));",
                "global uint *hypercall_continuation = (global uint *)hypercall_continuation_global+(get_global_id(0));",
                "global uint *current_mem_size = (global uint *)current_mem_size_global+(get_global_id(0));",
                "global uint *max_mem_size = (global uint *)max_mem_size_global+(get_global_id(0));",
                "global uchar *is_calling = (global uchar *)is_calling_global+(get_global_id(0));",
                "global uint  *entry_point   = (global uint*)entry_point_global+get_global_id(0);",
                "ulong warp_idx = get_global_id(0);",
                "ulong thread_idx = get_local_id(0);",
                "global uint  *hcall_ret_val = (global uint*)hcall_ret_val_global+get_global_id(0);",
                "global uint  *hcall_size = (global uint*)hcall_size_global+get_global_id(0);").unwrap();
            } else {
                // The pointer math must be calculated in terms of bytes, which is why we cast to (char*) first
                write!(output, "\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\n\t{}\n",
                format!("global uint  *stack_u32    = (global uint*)((global char*)stack_u32_global+(get_global_id(0) * {}));", stack_size_bytes),
                "global ulong *stack_u64    = (global ulong*)stack_u32;",
                format!("global uint  *heap_u32     = (global uint *)((global char*)heap_u32_global+(get_global_id(0) * {}));", heap_size_bytes),
                "global ulong *heap_u64     = (global ulong *)heap_u32;",
                "global uint  *hcall_size = (global uint*)hcall_size_global+get_global_id(0);",
                format!("global uint  *hypercall_buffer = (global uint*)((global char*)hypercall_buffer_global+(get_global_id(0) * (uint)*hcall_size));"),
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
                "global int *hypercall_number = (global int *)hypercall_number_global+(get_global_id(0));",
                "global uint *hypercall_continuation = (global uint *)hypercall_continuation_global+(get_global_id(0));",
                "global uint *current_mem_size = (global uint *)current_mem_size_global+(get_global_id(0));",
                "global uint *max_mem_size = (global uint *)max_mem_size_global+(get_global_id(0));",
                "global uchar *is_calling = (global uchar *)is_calling_global+(get_global_id(0));",
                "global uint  *entry_point   = (global uint *)entry_point_global+get_global_id(0);",
                "ulong warp_idx = get_global_id(0);",
                "ulong thread_idx = get_local_id(0);",
                "global uint  *hcall_ret_val = (global uint*)hcall_ret_val_global+get_global_id(0);").unwrap();
            }
        // if we are an OpenCL kernel and we are not the control function, we only need the function header itself
        } else {
            write!(output, "{}", format!("
__attribute__((always_inline)) void {}(global uint   *stack_u32,
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
    global int    *hypercall_number,
    global uint   *hypercall_continuation,
    global uint   *current_mem_size,
    global uint   *max_mem_size,
    global uchar  *is_calling,
    ulong  warp_idx,
    ulong  thread_idx,
    uint   hcall_size,
    global uint   *entry_point,
    uint hcall_ret_val)", fn_name)).unwrap();
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
                        wast::MemoryType::B32{limits, ..} => {
                            let max = match limits.max {
                                Some(val) => val,
                                None => heap_size/(1024*64),
                            };
                            (limits.min as u64, max as u64)
                        },
                        wast::MemoryType::B64{limits, ..} => {
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
                            hcall_buf_size: u32,
                            interleave: u32,
                            stack_size_bytes: u32,
                            heap_size_bytes: u32,
                            call_stack_size_bytes: u32,
                            stack_frames_size_bytes: u32,
                            stack_frame_ptr_size_bytes: u32, 
                            predictor_size_bytes: u32,
                            local_work_group: usize,
                            debug_print_function_calls: bool,
                            globals_buffer_size: u32,
                            function_idx_label: HashMap<String, u32>,
                            debug: bool) -> String {
        let mut ret_str = String::from("");
        write!(ret_str, "{}",
                self.generate_function_prelude("wasm_entry",
                                               hcall_buf_size,
                                               interleave,
                                               stack_size_bytes,
                                               heap_size_bytes,
                                               stack_frames_size_bytes,
                                               call_stack_size_bytes,
                                               predictor_size_bytes,
                                               globals_buffer_size,
                                               stack_frame_ptr_size_bytes,
                                               local_work_group,
                                               true,
                                               debug)).unwrap();
        write!(ret_str, "\tstack_u32 += {};\n", 128).unwrap();
        if debug_print_function_calls {
            write!(ret_str, "\tprintf(\"stack_u32: %p\\n\", stack_u32);\n").unwrap();
            write!(ret_str, "\tprintf(\"heap_u32: %p\\n\", heap_u32);\n").unwrap();
            write!(ret_str, "\tprintf(\"is_calling: %p\\n\", is_calling);\n").unwrap();
            write!(ret_str, "\tprintf(\"hypercall_buffer: %p\\n\", hypercall_buffer);\n").unwrap();
            write!(ret_str, "\tprintf(\"call_stack: %p\\n\", call_stack);\n").unwrap();
            write!(ret_str, "\tprintf(\"stack_frames: %p\\n\", stack_frames);\n").unwrap();
            write!(ret_str, "\tprintf(\"call_return_stack: %p\\n\", call_return_stack);\n").unwrap();
            write!(ret_str, "\tprintf(\"hypercall_number: %p\\n\", hypercall_number);\n").unwrap();
            write!(ret_str, "\tprintf(\"hypercall_continuation: %p\\n\", hypercall_continuation);\n").unwrap();
            write!(ret_str, "\tprintf(\"current_mem_size: %p\\n\", current_mem_size);\n").unwrap();
            write!(ret_str, "\tprintf(\"max_mem_size: %p\\n\", max_mem_size);\n").unwrap();
        }

        write!(ret_str, "\t{}\n", "do {").unwrap();
        write!(ret_str, "\t{}\n", "switch (*entry_point) {").unwrap();
        for key in function_idx_label.keys() {
            write!(ret_str, "\t\tcase {}:\n", function_idx_label.get(key).unwrap()).unwrap();
            if debug_print_function_calls {
                write!(ret_str, "\t\tprintf(\"{}\\n\");\n", format!("{}{}", "__", key.replace(".", ""))).unwrap();
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
                            "hypercall_number",
                            "hypercall_continuation",
                            "current_mem_size",
                            "max_mem_size",
                            "is_calling",
                            "warp_idx",
                            "thread_idx",
                            "*hcall_size",
                            "entry_point",
                            "*hcall_ret_val").unwrap();
            write!(ret_str, "\t\t\tbreak;\n").unwrap();
        }
        write!(ret_str, "\t\tdefault:\n").unwrap();
        if debug_print_function_calls {
            write!(ret_str, "\t\t\tprintf(\"{}\\n\");\n", "taking default case").unwrap();
        }
        write!(ret_str, "\t\t\treturn;\n").unwrap();
        write!(ret_str, "\t}}\n").unwrap();

        // if we reset the hypercall_number, that means we need to exit back to the VMM
        write!(ret_str, "\t{}\n", "} while (*sfp != 0 && *hypercall_number == -2);").unwrap();

        write!(ret_str, "}}\n").unwrap();

        ret_str
    }

    pub fn write_opencl_file(&self,
                             hcall_buf_size: u32,
                             interleave: u32,
                             stack_size_bytes: u32,
                             heap_size_bytes: u32,
                             call_stack_size_bytes: u32,
                             stack_frames_size_bytes: u32,
                             stack_frame_ptr_size_bytes: u32, 
                             predictor_size_bytes: u32,
                             max_partitions: u32,
                             max_loc_in_partition: u32,
                             max_duplicate_funcs: u32,
                             local_work_group: usize,
                             disable_fastcalls: bool,
                             debug_print_function_calls: bool,
                             force_inline: bool,
                             is_gpu: bool,
                             debug: bool) -> (String, String, u32, u32, u32, HashMap<u32, String>, HashMap<u32, (u32, u32, u32, u32, u32, u32)>, HashMap<u32, u32>) {
        let mut output = String::new();
        let mut header = String::new();
        let mut kernel_hashmap: HashMap<u32, String> = HashMap::new();
        let mut kernel_compile_stats: HashMap<u32, (u32, u32, u32, u32, u32, u32)> = HashMap::new();
        let mut kernel_partition_mappings: HashMap<u32, u32> = HashMap::new();

        // Find the "_start" function
        let mut start_fn_name_tmp = None;
        for export in &self.exports {
            if export.name == "_start" {
                start_fn_name_tmp = Some(&export.kind);
            }
        }
        let start_func = match start_fn_name_tmp {
            Some(wast::ExportKind::Func(wast::Index::Id(id))) => id.name().to_string(),
            Some(wast::ExportKind::Func(wast::Index::Num(val, _))) => format!("func_{:?}", val),
            Some(_) => panic!("Unable to find \"_start\" function"),
            None => panic!("Unable to find \"_start\" function"),
        };

        // enable the usage of FP64 operations (double precision floats)
        // if we are unable to enable, floating point calculations may be incorrect
        write!(output, "{}",
r#"
#ifdef cl_khr_fp64
#pragma OPENCL EXTENSION cl_khr_fp64 : enable
#elif defined(cl_amd_fp64)
#pragma OPENCL EXTENSION cl_amd_fp64 : enable
#endif
// we always want this
#pragma OPENCL EXTENSION cl_khr_byte_addressable_store : enable

// Emit some utility macros

#define FMIN(x, y)                                \
   (((x) != (x)) ? NAN                            \
  : ((y) != (y)) ? NAN                            \
  : ((x) == 0 && (y) == 0) ? (signbit(x) ? x : y) \
  : (x < y) ? x : y)

#define FMAX(x, y)                                \
   (((x) != (x)) ? NAN                            \
  : ((y) != (y)) ? NAN                            \
  : ((x) == 0 && (y) == 0) ? (signbit(x) ? y : x) \
  : (x > y) ? x : y)
"#).unwrap();

        // generate the read/write functions
        // we support only either a 1 byte interleave, or no interleave
        // 0 = no interleave
        write!(output, "{}", generate_read_write_calls(&self, interleave, debug)).unwrap();
        write!(header, "{}", generate_read_write_calls(&self, interleave, debug)).unwrap();

        // generate the hypercall helper section
        write!(output, "{}", self.generate_hypercall_helpers(debug)).unwrap();
        write!(header, "{}", self.generate_hypercall_helpers(debug)).unwrap();

        write!(output, "#include \"fastcalls.cl\"").unwrap();

        let prelude_header = output.clone();

        let mut fastcall_header = String::from("");

        // generate the data loading function
        // also return the global mappings: global id -> (global buffer offset, global size)
        let (data_section, global_mappings) = self.generate_data_section(interleave, heap_size_bytes, debug);
        let mut globals_buffer_size = 0;

        for (_key, (_offset, size)) in &global_mappings {
            globals_buffer_size += size;
        }

        let data_program = format!("{}\n{}", prelude_header, data_section.clone());
        kernel_hashmap.insert(99999, data_program);
        kernel_partition_mappings.insert(99999, 99999);

        write!(output, "{}", data_section).unwrap();

        // for each function, assign an ID -> index mapping
        let mut function_idx_label: HashMap<&str, u32> = HashMap::new();
        let mut count = 0;
        let funcs_keys = self.func_map.keys().clone();
        let funcs = self.func_map.values().clone();
        for f_name in funcs_keys {
            function_idx_label.insert(f_name, count);
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
        
        let mut func_mapping: HashMap<String, &wast::Func> = HashMap::new();

        for (name, func) in &self.func_map {
            func_mapping.insert(name.to_string(), func);
        }

        // generate the set of indirect calls
        let mut indirect_call_set = HashSet::new();
        for (_, indirect_call_name) in indirect_call_mapping.iter() {
            let name = match indirect_call_name {
                wast::Index::Id(id)  => id.name().to_string(),
                wast::Index::Num(val, _) => format!("func_{}", val),
            };
            indirect_call_set.insert(name);
        }

        let fast_function_set = if !disable_fastcalls {
            compute_fastcall_set(self, &self.func_map, &mut indirect_call_set, start_func.clone())
        } else {
            let allowed_fastcalls = vec!["__lctrans",
                                         "dlfree",
                                         "__rust_start_panic",
                                         "__stpcpy",
                                         "__rdl_dealloc",
                                         "__rdl_realloc",
                                         "__rdl_alloc_zeroed",
                                         "strdup",
                                         "__rust_alloc_zeroed",
                                         "__stpcpy",
                                         "__rdl_dealloc",
                                         "__rdl_realloc",
                                         "__rdl_alloc_zeroed",
                                         "strdup",
                                         "__rust_alloc_zeroed",
                                         "__rust_alloc",
                                         "dummy",
                                         "strlen",
                                         "dummy_1",
                                         "calloc",
                                         "strncmp",
                                         "memcpy",
                                         "strerror_r",
                                         "__rust_dealloc",
                                         "free",
                                         "strerror",
                                         "realloc",
                                         "__rust_realloc",
                                         "getcwd",
                                         "aligned_alloc",
                                         "abort",
                                         "__strchrnul",
                                         "malloc",
                                         "__rdl_alloc",
                                         "memcmp",
                                         "memset",
                                         "dispose_chunk",
                                         "strcpy",
                                         "__wasm_call_dtors"];
            let mut final_hset = HashSet::new();
            let mut hset = HashSet::new();
            for f in allowed_fastcalls {
                hset.insert(f.to_string());
            }
            let mut fastcall_set = compute_fastcall_set(self, &self.func_map, &mut indirect_call_set, start_func.clone());

            // now filter to only allow the allowed list and functions they call
            let hset_clone = hset.clone();
            let intersection = fastcall_set.intersection(&hset_clone);
            for func in intersection {
                let (called_loop, called) = get_called_funcs(self,
                                                             indirect_call_mapping,
                                                             self.func_map.get(&func.to_string()).unwrap(),
                                                             &HashSet::new(),
                                                             &func_mapping,
                                                             &self.imports_map,
                                                             &mut HashSet::new());
                for call in called_loop {
                    final_hset.insert(call);
                }
                for call in called {
                    final_hset.insert(call);
                }
            }

            final_hset
        };

        // Generate the fastcall header

        // first generate the function declarations
        for fastfunc in fast_function_set.iter() {
            let func = self.func_map.get(&fastfunc.to_string()).unwrap();
            // Get parameters & return type of the function
            let params = get_func_params(self, &func.ty);
            let mut parameter_list = String::from("");
            for parameter in params {
                let fn_ty_name = match parameter {
                    StackType::i32 => format!("uint"),
                    StackType::i64 => format!("ulong"),
                    StackType::f32 => format!("float"),
                    StackType::f64 => format!("double")
                };
                parameter_list += &format!("{}, ", fn_ty_name);
            }

            let ret_val = get_func_result(self, &func.ty);
            let func_ret_val = match ret_val {
                Some(ty) => {
                    match ty {
                        StackType::i32 => format!("uint"),
                        StackType::i64 => format!("ulong"),
                        StackType::f32 => format!("float"),
                        StackType::f64 => format!("double")
                    }
                },
                None => {
                    format!("void")
                }
            };
            let calling_func_name = format!("{}{}", "__", fastfunc.to_string().replace(".", ""));
            let func_declaration = format!("{} {}_fastcall({}global uint *, global uint *, global uint *, global uint *, ulong);\n", func_ret_val, calling_func_name, parameter_list);
            write!(fastcall_header, "{}", func_declaration).unwrap();
        }

        // emit functions
        // Each fastcall has a stack frame size (size of intermediate values) + list of funcs it calls
        // We track these to resolve register spilling to shared memory
        let mut fast_func_size: HashMap<String, u32> = HashMap::new();
        let mut fast_func_called: HashMap<String, HashSet<String>> = HashMap::new();
        let mut fastcall_called_func_sizes: HashMap<String, Vec<u32>> = HashMap::new();

        for fastfunc in fast_function_set.iter() {
            let (func, func_size, func_called) = self.emit_function(self.func_map.get(&fastfunc.to_string()).unwrap(),
                                            fastfunc.to_string(),
                                            call_ret_map,
                                            &mut call_ret_idx,
                                            function_idx_label.clone(),
                                            hypercall_id_count,
                                            local_work_group,
                                            indirect_call_mapping,
                                            &global_mappings,
                                            fast_function_set.clone(),
                                            force_inline,
                                            debug_print_function_calls,
                                            start_func.clone(),
                                            is_gpu,
                                            &mut 0,
                                            true,
                                            debug);
            fast_func_size.insert(fastfunc.to_string(), func_size);
            fast_func_called.insert(fastfunc.to_string(), func_called);

            write!(fastcall_header, "{}", func).unwrap();
        }

        // Compute how many instructions each fastcall contains (necessary for packing functions)
        // This is needed for the function packing

        loop {
            // Go through each fastcall, tracking which ones we have known maximum possible sizes for 
            let mut counter = 0;
            for fastfunc in fast_function_set.iter() {
                let funcs_called = fast_func_called.get(&fastfunc as &str).unwrap();
                // we know the max possible size for this function already
                if funcs_called.len() == 0 {
                    counter += 1;
                } else {
                    // Try to go through the called funcs and see if we know the sizes for any called functions
                    // If so - we remove the func from the set
                    let mut new_called_set: HashSet<String> = HashSet::new();
                    for called_func in funcs_called {
                        // Get all the funcs that this func calls
                        let funcs_called = fast_func_called.get(&called_func as &str).unwrap();
                        // If len==0, then the size is known, else if len>0, then we need to keep it in the set
                        if funcs_called.len() != 0 {
                            new_called_set.insert(called_func.to_string());
                        } else {
                            // we are removing the func from the set, so we track the size
                            let size = fast_func_size.get(&called_func as &str).unwrap();
                            fastcall_called_func_sizes.entry(fastfunc.to_string()).or_insert(vec![]).push(*size);
                        }
                    }
                    // replace the hashset
                    fast_func_called.insert(fastfunc.clone(), new_called_set);
                }
            }

            // if all sizes are known, then that means we know all the sizes
            if counter == fast_function_set.len() {
                break;
            }
        }

        // Now update fast_func_size with max possible sizes
        for fastfunc in fast_function_set.iter() {
            let tmp = vec![0];
            let max = fastcall_called_func_sizes.get(fastfunc).unwrap_or(&tmp).iter().max().unwrap();
            *fast_func_size.entry(fastfunc.to_string()).or_insert(0) += max;
        }

        // Compute the function groups, we will then enumerate the groups to emit the functions
        // kernel_partition_mapping get the partition ID from a function idx
        let partitions = form_partitions(&self, max_partitions, max_loc_in_partition, max_duplicate_funcs, self.func_names.clone(), &fast_function_set, &func_mapping, &self.imports_map, &mut kernel_compile_stats, indirect_call_mapping);

        for (partition_idx, partition) in partitions.clone() {
            let mut function_idx_label_temp: HashMap<String, u32> = HashMap::new();
            let mut partition_func_str = String::from("");

            // for each function in a partition, perform codegen
            let mut partition_intermediate_size = vec![];
            let mut temp_partition = String::from("");
            for function in partition.clone() {
                let fname = function.clone();
                let func_to_emit = self.func_map.get(&function).unwrap();

                let (func, intermediate_size, called_fastcalls) = self.emit_function(func_to_emit,
                                              function,
                                              call_ret_map,
                                              &mut call_ret_idx,
                                              function_idx_label.clone(),
                                              hypercall_id_count,
                                              local_work_group,
                                              indirect_call_mapping,
                                              &global_mappings,
                                              fast_function_set.clone(),
                                              force_inline,
                                              debug_print_function_calls,
                                              start_func.clone(),
                                              is_gpu,
                                              &mut 0,
                                              false,
                                              debug);
                // perform conservative estimate of required register space
                let mut fastcall_int_sizes = vec![0];
                for fastcall in called_fastcalls.iter() {
                    let fsize = fast_func_size.get(fastcall).unwrap();
                    fastcall_int_sizes.push(*fsize);
                }
                partition_intermediate_size.push(intermediate_size + fastcall_int_sizes.iter().max().unwrap());
                temp_partition += &format!("{}", func);
                let fname_idx = function_idx_label.get(&fname as &str).unwrap();
                function_idx_label_temp.insert(fname, *fname_idx);
                kernel_partition_mappings.insert(*fname_idx, partition_idx);
            }

            // Check the partition size to see if we need to regenerate registers with constraints
            let max_partition_reg_usage = partition_intermediate_size.iter().max().unwrap();
            let sum_partition_reg_usage = partition_intermediate_size.iter().sum::<u32>();
            println!("max size: {}, sum size: {}, part_idx: {}", &max_partition_reg_usage, &sum_partition_reg_usage, partition_idx);

            // If constraint exceeded, regenerate the partition with constraints
            if sum_partition_reg_usage > 5000 {
                /*
                 * Compute size to reduce kernel by:
                 * 
                 * For now:
                 * max_partition_reg_usage > 10000 => Move 128, 4 byte sized locals or 64 8 byte sized locals
                 *
                 * max_partition_reg_usage > 5000 => Move 64, 4 byte sized locals or 32 8 byte sized locals
                 *
                 */
                let reduction_size: &mut u32 = &mut if sum_partition_reg_usage > 10000 {
                    512
                } else {
                    256
                };

                temp_partition = String::from("");
                for function in partition.clone() {
                    let fname = function.clone();
                    let func_to_emit = self.func_map.get(&function).unwrap();
    
                    let (func, intermediate_size, called_fastcalls) = self.emit_function(func_to_emit,
                                                  function,
                                                  call_ret_map,
                                                  &mut call_ret_idx,
                                                  function_idx_label.clone(),
                                                  hypercall_id_count,
                                                  local_work_group,
                                                  indirect_call_mapping,
                                                  &global_mappings,
                                                  fast_function_set.clone(),
                                                  force_inline,
                                                  debug_print_function_calls,
                                                  start_func.clone(),
                                                  is_gpu,
                                                  reduction_size,
                                                  false,
                                                  debug);
                    // perform conservative estimate of required register space
                    let mut fastcall_int_sizes = vec![0];
                    for fastcall in called_fastcalls.iter() {
                        let fsize = fast_func_size.get(fastcall).unwrap();
                        fastcall_int_sizes.push(*fsize);
                    }
                    partition_intermediate_size.push(intermediate_size + fastcall_int_sizes.iter().max().unwrap());
                    temp_partition += &format!("{}", func);
                    let fname_idx = function_idx_label.get(&fname as &str).unwrap();
                    function_idx_label_temp.insert(fname, *fname_idx);
                    kernel_partition_mappings.insert(*fname_idx, partition_idx);
                }
            }

            write!(output, "{}", temp_partition).unwrap();
            write!(partition_func_str, "{}\n", temp_partition).unwrap();

            let control_function = self.emit_wasm_control_fn(hcall_buf_size,
                                                            interleave,
                                                            stack_size_bytes,
                                                            heap_size_bytes,
                                                            call_stack_size_bytes,
                                                            stack_frames_size_bytes,
                                                            stack_frame_ptr_size_bytes, 
                                                            predictor_size_bytes,
                                                            local_work_group,
                                                            debug_print_function_calls,
                                                            globals_buffer_size,
                                                            function_idx_label_temp,
                                                            debug);

            let func_full = format!("{}\n{}\n{}\n", prelude_header.clone(), partition_func_str.clone(), control_function); 

            kernel_hashmap.insert(partition_idx, func_full);
        }
        

        // generate control function prelude
        write!(output, "{}",
                self.generate_function_prelude("wasm_entry",
                                               hcall_buf_size,
                                               interleave,
                                               stack_size_bytes,
                                               heap_size_bytes,
                                               stack_frames_size_bytes,
                                               call_stack_size_bytes,
                                               predictor_size_bytes,
                                               globals_buffer_size,
                                               stack_frame_ptr_size_bytes,
                                               local_work_group,
                                               true,
                                               debug)).unwrap();

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
                    if name.name() == start_func {
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
                        write!(output, "\tstack_u32 += {};\n", 128).unwrap();
                        break;
                    }
                },
                _ => (),
            }
        }

        if debug_print_function_calls {
            write!(output, "\tprintf(\"stack_u32: %p\\n\", stack_u32);\n").unwrap();
            write!(output, "\tprintf(\"heap_u32: %p\\n\", heap_u32);\n").unwrap();
            write!(output, "\tprintf(\"is_calling: %p\\n\", is_calling);\n").unwrap();
            write!(output, "\tprintf(\"hypercall_buffer: %p\\n\", hypercall_buffer);\n").unwrap();
            write!(output, "\tprintf(\"call_stack: %p\\n\", call_stack);\n").unwrap();
            write!(output, "\tprintf(\"stack_frames: %p\\n\", stack_frames);\n").unwrap();
            write!(output, "\tprintf(\"call_return_stack: %p\\n\", call_return_stack);\n").unwrap();
            write!(output, "\tprintf(\"hypercall_number: %p\\n\", hypercall_number);\n").unwrap();
            write!(output, "\tprintf(\"hypercall_continuation: %p\\n\", hypercall_continuation);\n").unwrap();
            write!(output, "\tprintf(\"current_mem_size: %p\\n\", current_mem_size);\n").unwrap();
            write!(output, "\tprintf(\"max_mem_size: %p\\n\", max_mem_size);\n").unwrap();
            write!(output, "\tprintf(\"globals_buffer: %p\\n\", globals_buffer);\n").unwrap();
        }

        write!(output, "\t{}\n", "do {").unwrap();
        write!(output, "\t{}\n", "switch (*entry_point) {").unwrap();
        for key in function_idx_label.keys() {
            write!(output, "\t\tcase {}:\n", function_idx_label.get(key).unwrap()).unwrap();
            if debug_print_function_calls {
                write!(output, "\t\tprintf(\"{}\\n\");\n", format!("{}{}", "__", key.replace(".", ""))).unwrap();
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
                            "hypercall_number",
                            "hypercall_continuation",
                            "current_mem_size",
                            "max_mem_size",
                            "is_calling",
                            "warp_idx",
                            "thread_idx",
                            "*hcall_size",
                            "entry_point",
                            "*hcall_ret_val").unwrap();
            write!(output, "\t\t\tbreak;\n").unwrap();
        }
        write!(output, "\t\tdefault:\n").unwrap();
            write!(output, "\t\t\treturn;\n").unwrap();
        write!(output, "\t}}\n").unwrap();

        // if we reset the hypercall_number, that means we need to exit back to the VMM
        write!(output, "\t{}\n", "} while (*sfp != 0 && *hypercall_number == -2);").unwrap();

        write!(output, "}}\n").unwrap();

        (output, fastcall_header, *function_idx_label.get(&start_func as &str).unwrap(), globals_buffer_size, funcs.len().try_into().unwrap(), kernel_hashmap, kernel_compile_stats, kernel_partition_mappings)
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
