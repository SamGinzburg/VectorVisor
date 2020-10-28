use wast::Wat;
use wast::parser::{self, ParseBuffer};
use wast::ModuleKind::{Text, Binary};
use wast::ValType;
use regex::Regex;

use std::fmt;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

use lazy_static::lazy_static;

/*
 * This hashmap contains the WASI calls that have already been implemented!
 */
lazy_static! {
    static ref wasi_snapshot_preview1: HashMap<&'static str, bool> = {
        let mut m = HashMap::new();
        m.insert("fd_write", true);
        m.insert("proc_exit", true);
        m
    };
}


pub struct OpenCLCWriter<'a> {
    types: Vec<wast::Type<'a>>,
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
            types: vec!(),
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

        match module.module.kind {
            Text(t) => {
                for item in t {
                    match item {
                        wast::ModuleField::Type(t) => self.types.push(t),
                        wast::ModuleField::Import(i) => {
                            dbg!(i.clone());
                            match i.clone().item.id {
                                Some(id) => self.imports_map.insert(id.name(), (i.module, i.field, i.item)),
                                None => continue,
                            };
                        },
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

    fn emit_i32_const(&self, val: &i32, debug: bool) -> String {
        format!("\t{}{}, warp_idx);\n\t{}\n",
                "write_u32((ulong)(stack_u32+*sp), (uint)",
                val,
                "*sp += 1;")
    }

    fn emit_i64_const(&self, val: &i64, debug: bool) -> String {
        format!("\t{}{}, warp_idx);\n\t{}\n",
                "write_u64((ulong)(stack_u32+*sp), (ulong)",
                val,
                "*sp += 2;")
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

    fn emit_local_set(&self, id: &str, offsets: &HashMap<&str, u32>, type_info: &HashMap<&str, ValType>, debug: bool) -> String {
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
                        "write_u32((ulong)(stack_u32+*sp), read_u32((ulong)(stack_u32+*sp-1), warp_idx), warp_idx);",
                        "*sp += 1;",
                        format!("{}", self.emit_local_set(id, offsets, type_info, debug)))
            },
            wast::ValType::I64 => {
                format!("\t{}\n{}",
                        format!("{}\n\t{}",
                                "write_u64((ulong)(stack_u32+*sp), read_u64((ulong)(stack_u32+*sp-2), warp_idx), warp_idx);",
                                "*sp += 2;"),
                        format!("{}", self.emit_local_set(id, offsets, type_info, debug)))
            },
            wast::ValType::F32 => {
                format!("\t{}\n\t{}\n{}",
                        "write_u32((ulong)(stack_u32+*sp), read_u32((ulong)(stack_u32+*sp-1), warp_idx), warp_idx);",
                        "*sp += 1;",
                        format!("{}", self.emit_local_set(id, offsets, type_info, debug)))
            },
            wast::ValType::F64 => {
                format!("\t{}\n{}",
                        format!("{}\n\t{}",
                                "write_u64((ulong)(stack_u32+*sp), read_u64((ulong)(stack_u32+*sp-2), warp_idx), warp_idx);",
                                "*sp += 2;"),
                        format!("{}", self.emit_local_set(id, offsets, type_info, debug)))
            },
            _ => panic!("emit_local_tee type not handled")
        }
    }

    // binops have both values popped off the stack
    fn emit_i32_add(&self, debug: bool) -> String {
        format!("\t{}\n\t{}\n",
                "write_u32((ulong)(stack_u32+*sp-2),
                           (int)read_u32((ulong)(stack_u32+*sp-1), warp_idx) + (int)read_u32((ulong)(stack_u32+*sp-2), warp_idx),
                           warp_idx);",
                "*sp -= 1;")
    }

    /*
     * addition is a binop - pops 2 values off the stack and pushes one back on
     */
    fn emit_i64_add(&self, debug: bool) -> String {
        format!("\t{}\n\t{}\n",
                "write_u64((ulong)(stack_u32+*sp-4),
                           (long)read_u64((ulong)(stack_u32+*sp-2), warp_idx) + (long)read_u64((ulong)(stack_u32+*sp-4), warp_idx),
                           warp_idx);",
                "*sp -= 2;")
    }

    /*
     * <, >, = are relops which also pop 2 values and push one back on
     */
    fn emit_i32_lt_s(&self, debug: bool) -> String {
        format!("\t{}\n\t{}\n",
                "write_u32((ulong)(stack_u32+*sp-2),
                           (int)read_u32((ulong)(stack_u32+*sp-1), warp_idx) < (int)read_u32((ulong)(stack_u32+*sp-2), warp_idx),
                           warp_idx);",
                "*sp -= 1;")
    }

    fn emit_i32_eq(&self, debug: bool) -> String {
        format!("\t{}\n",
                "write_u32((ulong)(stack_u32+*sp-1),
                           (int)(read_u32((ulong)(stack_u32+*sp-1), warp_idx)) == (int)(read_u32((ulong)(stack_u32+*sp-2), warp_idx)),
                           warp_idx);")
    }

    fn emit_block(&self, block: &wast::BlockType, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
        let mut result: String = String::from("");
        let label = block.label.unwrap().name();

        // first we have to save the current stack pointer
        // to reset the stack if we jump to this label
        dbg!(label);
        let re = Regex::new(r"\d+").unwrap();
        // we can use the branch index to save to global state
        let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
        dbg!(branch_idx);
        let branch_idx_u32 = branch_idx.parse::<u32>().unwrap();
        if branch_idx_u32 > 1024 {
            panic!("Only up to 1024 branches per function are supported");
        }

        // create a new stack frame for the block, store stack frame pointer in local
        // function private data


        // we have to emulate a 2-D array, since openCL does not support double pts in v1.2
        // the format is (64 x 64 * number of functions),
        // so [..........] 4096 entries per function consecutively
        // lookups are done as: branch_value_stack_state[(*sfp * 64) + idx + (func_id * 4096)]
        // sfp = stack frame ptr, idx = branch ID, func_id = the numerical id of the function

        result += &format!("\t{}\n",
                            // write_
                           format!("branch_value_stack_state[(*sfp * 64) + {} + ({} * 4096)] = *sp;",
                           branch_idx_u32, function_id_map.get(fn_name).unwrap()));
        // we don't emit a label for block statements here, any br's goto the END of the block
        // we don't need to modify the sp here, we will do all stack unwinding in the br instr
        result
    }

    // basically the same as emit_block, except we have to reset the stack pointer
    // at the *top* of the block, since we are doing a backwards jump not a forward jump
    fn emit_loop(&self, block: &wast::BlockType, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
        let mut result: String = String::from("");
        let label = block.label.unwrap().name();

        // first we have to save the current stack pointer
        // to reset the stack if we jump to this label
        dbg!(label);
        let re = Regex::new(r"\d+").unwrap();
        // we can use the branch index to save to global state
        let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
        dbg!(branch_idx);
        let branch_idx_u32 = branch_idx.parse::<u32>().unwrap();
        if branch_idx_u32 > 1024 {
            panic!("Only up to 1024 branches per function are supported");
        }

        // create a new stack frame for the block, store stack frame pointer in local
        // function private data

        // we have to emulate a 2-D array, since openCL does not support double pts in v1.2
        // the format is (64 x 64 * number of functions),
        // so [..........] 4096 entries per function consecutively
        // lookups are done as: branch_value_stack_state[(*sfp * 64) + idx + (func_id * 4096)]
        // sfp = stack frame ptr, idx = branch ID, func_id = the numerical id of the function

        result += &format!("\t{}\n",
                           format!("write_u16((ulong)(loop_value_stack_state+(*sfp*64)+{}+({} *4096)), (ushort)*sp, warp_idx);",
                           branch_idx_u32, function_id_map.get(fn_name).unwrap()));

        // emit a label here for the END instruction to jump back here to restart the loop
        result += &format!("{}:\n", label);
        // the stack pointer should be reset by the BR/BR_IF instruction, so no need to touch it here

        result
    }




    // semantically, the end statement pops from the control stack,
    // in our compiler, this is a no-op
    fn emit_end(&self, id: &Option<wast::Id<'a>>, label: &str, block_type: u32, fn_name: &str, function_id_map: HashMap<&str, u32>, debug: bool) -> String {
        dbg!(id);
        dbg!(label);
        dbg!(block_type);
        println!("emit end!");
        // after a block ends, we need to unwind the stack!
        let re = Regex::new(r"\d+").unwrap();
        // we can use the branch index to save to global state
        let branch_idx: &str = re.captures(label).unwrap().get(0).map_or("", |m| m.as_str());
        dbg!(branch_idx);
        let branch_idx_u32 = branch_idx.parse::<u32>().unwrap();
        if branch_idx_u32 > 1024 {
            panic!("Only up to 1024 branches per function are supported");
        }

        // if the end statement corresponds to a block -> we want to put the label *here* and not at the top
        // of the block, otherwise for loops we jump back to the start of the loop!
        // 0 -> block (label goes here, at the end statement)
        // 1-> loop (label was already inserted at the top, this is a no-op here)
        if block_type == 0 {
            format!("\n{}:\n\t{}\n", label,
                    format!("*sp = read_u16((ulong)(branch_value_stack_state+(*sfp*64)+{}+({}*4096)), warp_idx);",
                            branch_idx_u32, function_id_map.get(fn_name).unwrap()))
        } else {
            let mut result = String::from("");
            result += &format!("\t/* END (loop: {}) */\n", label);
            
            // pop the control flow stack entry (reset the stack to the state it was in before the loop)
            result += &format!("\t*sp = read_u16((ulong)(branch_value_stack_state+(*sfp*64)+{}+({}*4096)), warp_idx);\n",
                                branch_idx_u32, function_id_map.get(fn_name).unwrap());

            result
        }
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
        println!("return size: {}", return_size);
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
    fn function_unwind(&self, fn_name: &str, func_ret_info: &Option<wast::FunctionType>, debug: bool) -> String {
        let mut final_str = String::from("");

        let results: Vec<wast::ValType> = match func_ret_info {
            Some(s) => (*s.results).to_vec(),
            None => vec![]
        };
        
        final_str += &format!("\t{}\n", "/* function unwind */");
        final_str += &format!("{}_return:\n", fn_name);
        // for each value returned by the function, return it on the stack
        // keep track of the change to stack ptr from previous returns
        let mut sp_counter = 0;
        let mut offset = String::from("");
        for value in results {
            match value {
                wast::ValType::I32 => {
                    // compute the offset to read from the bottom of the stack
                    if sp_counter > 0 {
                        offset = format!("write_u32((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                    read_u32((ulong)(stack_u32+*sp-{}-1), warp_idx),
                                                    warp_idx);", sp_counter);
                    } else {
                        offset = format!("write_u32((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                    read_u32((ulong)(stack_u32+*sp-1), warp_idx),
                                                    warp_idx);");
                    }
                    final_str += &format!("\t{}\n", offset);
                    sp_counter += 1;
                },
                wast::ValType::I64 => {
                    // compute the offset to read from the bottom of the stack
                    if sp_counter > 0 {
                        offset = format!("write_u64((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                    read_u64((ulong)(stack_u32+*sp-{}-2), warp_idx),
                                                    warp_idx);", sp_counter);
                    } else {
                        offset = format!("write_u64((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                    read_u64((ulong)(stack_u32+*sp-2), warp_idx),
                                                    warp_idx);");
                    }
                    final_str += &format!("\t{}\n", offset);
                    sp_counter += 2;
                },
                wast::ValType::F32 => {
                    // compute the offset to read from the bottom of the stack
                    if sp_counter > 0 {
                        offset = format!("write_u32((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                    read_u32((ulong)(stack_u32+*sp-{}-1), warp_idx),
                                                    warp_idx);", sp_counter);
                    } else {
                        offset = format!("write_u32((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                    read_u32((ulong)(stack_u32+*sp-1), warp_idx),
                                                    warp_idx);");
                    }
                    final_str += &format!("\t{}\n", offset);
                    sp_counter += 1;
                },
                wast::ValType::F64 => {
                    // compute the offset to read from the bottom of the stack
                    if sp_counter > 0 {
                        offset = format!("write_u64((ulong)(stack_u32+read_u32((ulong)(stack_frames+*sfp), warp_idx)),
                                                    read_u64((ulong)(stack_u32+*sp-{}-2), warp_idx),
                                                    warp_idx);", sp_counter);
                    } else {
                        offset = String::from("*(ulong *)(stack_u32+stack_frames[*sfp]) = *(ulong *)(stack_u32 + *sp - 2);");
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

    // TODO: double check the semantics of this? 
    fn emit_return(&self, fn_name: &str, debug: bool) -> String {
        format!("\tgoto {}_return;\n", fn_name)
    }

    // this function is semantically equivalent to function_unwind
    fn emit_br(&self, idx: wast::Index, fn_name: &str, prev_stack_size: u32, debug: bool) -> String {
        let mut ret_str = String::from("");

        let branch_id = match idx {
            wast::Index::Id(id) => id.name(),
            _ => panic!("Branch specified in terms of numerical index instead of Id"),
        };

        let re = Regex::new(r"\d+").unwrap();
        // we can use the branch index to save to global state
        let branch_idx: &str = re.captures(branch_id).unwrap().get(0).map_or("", |m| m.as_str());

        // debug comment
        ret_str += &format!("\t{}\n", format!("/* br {} */", branch_id));

        // first we want to pop the result value off of the stack, and push it
        dbg!(prev_stack_size);
        /*
        match prev_stack_size {
            1 => {
                // first push the value back
                // next, move the stack pointer
                ret_str += &format!("\t{}\n\t{}\n",
                                    format!("stack_u32[branch_value_stack_state[{}]] = stack_u32[*sp - 1];", branch_idx),
                                    format!("*sp = stack_u32[branch_value_stack_state[{}]];", branch_idx));
            },
            2 => {
                panic!("u64 br l not yet implemented");
                ret_str += &format!("\t{}\n",
                                    "*(ulong*)(stack_u32+*sp-4) = *(ulong*)(stack_u32+*sp-2) + *(ulong*)(stack_u32+*sp-4);");
            },
            _ => panic!("Unable to determine size of the previous item on stack"),
        };
        */

        ret_str += &format!("\t{}\n", format!("goto {};", branch_id));

        ret_str
    }

    fn emit_br_if(&self, idx: wast::Index, fn_name: &str, prev_stack_size: u32, debug: bool) -> String {
        let mut ret_str = String::from("");

        // br_if is just an if statement, if cond is true => br l else continue
        ret_str += &format!("\tif ({} != 0) {{\n", "read_u32((ulong)(stack_u32+*sp-1), warp_idx)");
        ret_str += &self.emit_br(idx, fn_name, prev_stack_size, debug);
        ret_str += &format!("\t}}\n");

        ret_str
    }

    fn emit_hypercall(&self, hypercall_id: u32, hypercall_id_count: &mut u32, debug: bool) -> String {
        let mut ret_str = String::from("");
        // set the hypercall ret flag flag + r
        ret_str += &format!("\t{}\n", format!("*hypercall_number = {};", hypercall_id));
        // insert return (we exit back to the VMM)
        ret_str += &format!("\t{}\n\t{}\n",
                            format!("*hypercall_continuation = {};", hypercall_id_count),
                            "return;");
        // insert return label, the VMM will return to right after the return
        ret_str += &format!("hypercall_return_stub_{}:\n", hypercall_id_count);
        // increment hypercall_id_count
        *hypercall_id_count += 1;
        ret_str
    }

    fn emit_instructions(&self,
                         instr: &wast::Instruction,
                         offsets: &HashMap<&str, u32>,
                         type_info: &HashMap<&str, ValType>,
                         call_ret_map: &mut HashMap<&str, u32>,
                         call_ret_idx: &mut u32,
                         fn_name: &str,
                         previous_stack_size: &mut u32,
                         control_stack: &mut Vec<(String, u32)>,
                         function_id_map: HashMap<&str, u32>,
                         hypercall_id_count: &mut u32,
                         debug: bool) -> String {

        match instr {
            wast::Instruction::I32Const(val) => {
                *previous_stack_size = 1;
                self.emit_i32_const(val, debug)
            },
            wast::Instruction::I64Const(val) => {
                *previous_stack_size = 2;
                self.emit_i64_const(val, debug)
            },
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
                *previous_stack_size = 1;
                self.emit_i32_add(debug)
            },
            wast::Instruction::I64Add => {
                *previous_stack_size = 2;
                self.emit_i64_add(debug)
            },
            wast::Instruction::Call(idx) => {
                let id = match idx {
                    wast::Index::Id(id) => id.name(),
                    _ => panic!("Unable to get Id for function call!"),
                };

                // check function to see if it is imported
                // if the function is imported - AND it is a WASI function,
                // emit the special call stub
                if self.imports_map.contains_key(id) {
                    match self.imports_map.get(id) {
                        Some((wasi_api, Some(wasi_fn_name), item)) => {
                            dbg!(item);
                            dbg!(wasi_fn_name);
                            dbg!(wasi_api);
                            // okay, now we check to see if the WASI call is supported by the compiler
                            // if not -> panic, else, emit the call
                            match (wasi_api, wasi_snapshot_preview1.get(wasi_fn_name)) {
                                // ignore WASI API scoping for now
                                (_, Some(true)) => {
                                    dbg!(wasi_fn_name);
                                    match wasi_fn_name {
                                        &"fd_write" => self.emit_hypercall(0, hypercall_id_count, debug),
                                        &"proc_exit" => self.emit_hypercall(1, hypercall_id_count, debug),
                                        _ => panic!("Unidentified WASI fn name: {:?}", wasi_fn_name),
                                    }
                                },
                                _ => panic!("WASI import not found, this probably means the system call is not yet implemented: {:?}", wasi_fn_name)
                            }
                        },
                        _ => panic!("Unsupported system call found {:?}", self.imports_map.get(id))
                    }
                } else {
                    // else, this is a normal function call
                    dbg!(id);
                    dbg!(idx);
                    dbg!(self.func_map.get(id));
                    // if self.func_map.get(id) is none, we have an import
                    // right now we only support WASI imports
                    match self.func_map.get(id) {
                        Some(_) => {
                            let func_type_signature = &self.func_map.get(id).unwrap().ty;
                            let fn_result_type = &(*func_type_signature.clone().inline.unwrap().results)[0];
                            *previous_stack_size = self.get_size_valtype(fn_result_type);
                            self.emit_fn_call(*idx, call_ret_map, call_ret_idx, debug)
                        },
                        // we have an import that isn't a system call...
                        None => String::from("")
                    }
                }
            },
            wast::Instruction::I32LtS => {
                *previous_stack_size = 1;
                self.emit_i32_lt_s(debug)
            }
            wast::Instruction::I32Eq => {
                *previous_stack_size = 1;
                self.emit_i32_eq(debug)
            },
            // control flow instructions
            wast::Instruction::Block(b) => {
                let label = b.label.unwrap().name().clone();
                control_stack.push((label.to_string(), 0));
                self.emit_block(b, fn_name, function_id_map, debug)
            },
            wast::Instruction::Loop(b) => {
                let label = b.label.unwrap().name().clone();
                control_stack.push((label.to_string(), 1));
                self.emit_loop(b, fn_name, function_id_map, debug)
            }
            // if control_stack.pop() panics, that means we were parsing an incorrectly defined
            // wasm file, each block/loop must have a matching end!
            wast::Instruction::End(id) => {
                let (label, t) = control_stack.pop().unwrap();
                self.emit_end(id, &label, t, fn_name, function_id_map, debug)
            },
            wast::Instruction::Return => self.emit_return(fn_name, debug),
            wast::Instruction::Br(idx) => self.emit_br(*idx, fn_name, *previous_stack_size, debug),
            wast::Instruction::BrIf(idx) => self.emit_br_if(*idx, fn_name, *previous_stack_size, debug),
            _ => panic!("Instruction {:?} not yet implemented", instr)
        }
    }

    fn emit_function(&self, func: &wast::Func, call_ret_map: &mut HashMap<&str, u32>,
                     call_ret_idx: &mut u32, function_id_map: HashMap<&str, u32>,
                     hypercall_id_count: &mut u32, debug: bool) -> String {
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
                dbg!("{:?}", locals);
                dbg!("{:?}", expression);

                /*
                 * __wasm_call_ctors is added by the wasm-linker:
                 * see: https://github.com/emscripten-core/emscripten/issues/10742#issuecomment-602068989
                 * and also see: https://iandouglasscott.com/2019/07/18/experimenting-with-webassembly-dynamic-linking-with-clang/
                 * This function is called externally from JS to perform dynamic linking of WASM modules
                 * It internally calls "__wasm_apply_relocs", which relocates imports specified within the module so
                 * that they can be called.
                 * 
                 * 
                 *  We have a more complex situation, since we can't actually modify the GPU kernel after it is created,
                 *  So instead we have to statically link the functions.
                 */
                if id.name() == "__wasm_call_ctors" {
                    return "".to_string()
                }

                let mut offset = 0;
                // get offsets for parameters, we record offsets from the start of the stack frame
                match typeuse.clone().inline {
                    Some(params) => {
                        for parameter in params.params.to_vec() {
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
        
                    },
                    None => (),
                }

                //stack_frames[*sfp - 1]; points to the head of the stack frame

                // get offsets for locals
                for local in locals {
                    local_parameter_stack_offset.insert(local.id.unwrap().name(), offset);
                    local_type_info.insert(local.id.unwrap().name(), local.ty.clone());
                    offset += self.get_size_valtype(&local.ty);
                }

                // function entry point
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

                // keep a stack of control-flow labels
                // for blocks we need to put the label at the "end" statement, while loops always jump back
                let mut control_stack: Vec<(String, u32)> = vec![];

                // we are now ready to execute instructions!
                let mut previous_stack_size: &mut u32 = &mut 0;
                
                // get the list of instructions first, to solve a lifetime mismatch error
                // (we can't just iterate because the control stack would have a different lifetime)
                for instruction in expression.instrs.iter() {
                    final_string += &self.emit_instructions(instruction,
                                                            &local_parameter_stack_offset,
                                                            &local_type_info,
                                                            call_ret_map,
                                                            call_ret_idx,
                                                            id.name(),
                                                            previous_stack_size,
                                                            &mut control_stack,
                                                            function_id_map.clone(),
                                                            hypercall_id_count,
                                                            debug);
                }

                // to unwind from the function we unwind the call stack by moving the stack pointer
                // and returning the last value on the stack 
                final_string += &self.function_unwind(id.name(), &typeuse.inline, debug);
            },
            (_, _, _) => panic!("Inline function must always have a valid identifier in wasm")
        };
        final_string
    }

    fn emit_memcpy_arr(&self) -> String {
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
                       format!("write_u8((ulong)((char*)heap_u32 + {} + idx), data_segment_data_{}[idx], warp_idx);",
                               offset_val,
                               counter));
            result += &String::from("\t}\n");

            arr_len = 0;
            counter += 1;
        }

        result
    }

    // This function generates the helper kernel that loads the data sections
    // of a program into memory, it is basically just a memcpy
    fn generate_data_section(&self, debug: bool) -> String {
        let mut result = String::from("");

        if debug {
            result += &String::from("\nvoid data_init(uint *heap_u32) {\n");
            result += &String::from("\tulong warp_idx = 0;\n");
            result += &self.emit_memcpy_arr();

            result += &String::from("}\n\n");
        } else {
            result += &String::from("\n__kernel void data_init(__global uint *heap_u32) {\n");

            result += &self.emit_memcpy_arr();

            result += &String::from("}\n\n");
        }
        result
    }

    /*
     * All reads and writes are abstracted through these calls
     * We want to support no interleave, as well as 1 byte, 4 byte, and 8 byte interleaves
     * 
     * Addressing model:
     *  For no interleave, the linear memory is divided into N regions, where N=NUM_THREADS
     * 
     *  Virtual Address = address, this calc is easy because we simply set the heap/stack pointers
     *  at the start of the kernel call, so no pointer math has to be done!
     * 
     *  For a 1 byte interleave, the linear memory is interleaved, with corresponding offsets
     *  mapped to adjacent bytes. For example: if you had 4 threads, that each write 0x1 to
     *  to an address of 0, the corresponding memory would look like:
     *
     *  [0x1, 0x1, 0x1, 0x1], with each of the writes sharing an offset of 0
     *  [T0 (byte 0), T1 (byte 0), T2 (byte 0), T3 (byte 0), T0 (byte 1), ...]
     * 
     * 
     *  The offset calc is:
     * 
     *  Virtual Address = address + (warp_idx * NUM_THREADS)
     *  
     *  ex: if you are in thread 0, and you write to 0, and then 1
     *  the physical addresses are first 0, and then address+NUM_THREADS, with each
     *  subsequent byte being 1 stride of NUM_THREADS away
     * 
     *  We expect NUM_THREADS to be defined at compile time with the macro NUM_THREADS
     * 
     *  We also have to split multi-byte reads into multiple calls, in little-endian format
     * 
     */
    fn generate_read_write_calls(&self, interleave: u32, debug: bool) -> String {
        let mut result = String::from("");

        // we need the warp id to generate the interleave
        // the write functions
        result += &format!("\n{}\n",
                           "void write_u8(ulong addr, uchar value, uint warp_id) {");

        match interleave {
            0 => {
                result += &format!("\t{}",
                                   "*((uchar*)addr) = value;");
            },
            1 => {
                result += &format!("\t{}",
                                   "*((uchar*)addr + (warp_id * NUM_THREADS)) = value;")
            }
            _ => panic!("Unsupported read/write interleave"),
        }

        result += &format!("\n{}\n",
                           "}");
        result += &format!("\n{}\n",
                           "void write_u16(ulong addr, ushort value, uint warp_id) {");
        match interleave {
            0 => {
                result += &format!("\t{}",
                                   "*((ushort*)addr) = value;");
            },
            1 => {
                // write the lower byte first
                result += &format!("\t{}\n",
                                   "write_u8(addr, value & 0xFF, warp_id);");
                // now write the upper byte
                result += &format!("\t{}",
                                   "write_u8((ulong)(((char*)addr)+1), (value >> 8) & 0xFF, warp_id);");
            }
            _ => panic!("Unsupported read/write interleave"),
        }
        result += &format!("\n{}\n",
                           "}");
        result += &format!("\n{}\n",
                           "void write_u32(ulong addr, uint value, uint warp_id) {");
        match interleave {
            0 => {
                result += &format!("\t{}",
                                   "*((uint*)addr) = value;");
            },
            1 => {
                // write the lower byte first
                result += &format!("\t{}\n",
                                   "write_u16(addr, value & 0xFFFF, warp_id);");
                // now write the upper byte
                result += &format!("\t{}",
                                   "write_u16((ulong)(((char*)addr)+2), (value >> 16) & 0xFFFF, warp_id);");
            }
            _ => panic!("Unsupported read/write interleave"),
        }
        result += &format!("\n{}\n",
                           "}");

        result += &format!("\n{}\n",
                           "void write_u64(ulong addr, ulong value, uint warp_id) {");
        match interleave {
            0 => {
                result += &format!("\t{}",
                                   "*((ulong*)addr) = value;");
            },
            1 => {
                // write the lower byte first
                result += &format!("\t{}\n",
                                    "write_u32(addr, value & 0xFFFFFFFF, warp_id);");
                // now write the upper byte
                result += &format!("\t{}",
                                    "write_u32((ulong)(((char*)addr)+4), (value >> 32) & 0xFFFFFFFF, warp_id);");
            }
            _ => panic!("Unsupported read/write interleave"),
        }
        result += &format!("\n{}\n",
                           "}");

        // the read functions
        result += &format!("\n{}\n",
                           "uchar read_u8(ulong addr, uint warp_id) {");
        match interleave {
            0 => {
                result += &format!("\t{}",
                                   "return *((uchar*)addr);");
            },
            1 => {
                result += &format!("\t{}",
                                   "return *(((uchar*)addr)+ (warp_id * NUM_THREADS));");
            }
            _ => panic!("Unsupported read/write interleave"),
        }
        result += &format!("\n{}\n",
                           "}");

        result += &format!("\n{}\n",
                           "ushort read_u16(ulong addr, uint warp_id) {");
        match interleave {
            0 => {
                result += &format!("\t{}",
                                   "return *((ushort*)addr);");
            },
            1 => {
                // use a local variable to store the result as we perform the reads
                // we have to read in the reverse order!!! (high bits then low bits)
                result += &format!("\t{}\n",
                                   "ushort temp = 0;");
                result += &format!("\t{}\n",
                                   "temp += read_u8((ulong)(((char*)addr)+1), warp_id);");
                // bitshift over to make room for the next byte
                result += &format!("\t{}\n",
                                   "temp = temp << 8;");
                result += &format!("\t{}\n",
                                   "temp += read_u8(addr, warp_id);");
                result += &format!("\t{}",
                                   "return temp;");
            }
            _ => panic!("Unsupported read/write interleave"),
        }
        result += &format!("\n{}",
                           "}");

        result += &format!("\n{}\n",
                           "uint read_u32(ulong addr, uint warp_id) {");
        match interleave {
            0 => {
                result += &format!("\t{}",
                                   "return *((uint*)addr);");
            },
            1 => {
                // use a local variable to store the result as we perform the reads
                result += &format!("\t{}\n",
                                    "uint temp = 0;");
                result += &format!("\t{}\n",
                                    "temp += read_u16((ulong)(((char*)addr)+2), warp_id);");
                // bitshift over to make room for the next byte
                result += &format!("\t{}\n",
                                    "temp = temp << 16;");
                result += &format!("\t{}\n",
                                    "temp += read_u16(addr, warp_id);");
                result += &format!("\t{}",
                                    "return temp;");
            }
            _ => panic!("Unsupported read/write interleave"),
        }
        result += &format!("\n{}",
                           "}");

        result += &format!("\n{}\n",
                           "ulong read_u64(ulong addr, uint warp_id) {");
        match interleave {
            0 => {
                result += &format!("\t{}",
                                   "return *((ulong*)addr);");
            },
            1 => {
                // use a local variable to store the result as we perform the reads
                result += &format!("\t{}\n",
                                    "ulong temp = 0;");
                result += &format!("\t{}\n",
                                    "temp += read_u32((ulong)(((char*)addr)+4), warp_id);");
                // bitshift over to make room for the next byte
                result += &format!("\t{}\n",
                                    "temp = temp << 32;");
                result += &format!("\t{}\n",
                                    "temp += read_u32(addr, warp_id);");
                result += &format!("\t{}",
                                    "return temp;");
            }
            _ => panic!("Unsupported read/write interleave"),
        }
        result += &format!("\n{}\n",
                           "}");
        result
    }


    pub fn write_opencl_file(&self, filename: &str, interleave: u32, debug: bool) -> () {
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
            write!(output, "{}", format!("#include \"../includes/wasm_hypercall.h\"\n"));

            write!(output, "{}", format!("#define uchar unsigned char\n"));
            write!(output, "{}", format!("#define ulong unsigned long\n"));
            write!(output, "{}", format!("#define uint unsigned int\n"));
            write!(output, "{}", format!("#define ushort unsigned short\n"));
        }

        // generate the read/write functions
        // we support 0, 1, 4, 8 byte interleaves
        // 0 = no interleave
        write!(output, "{}", self.generate_read_write_calls(interleave, debug));

        // generate the data loading function
        write!(output, "{}", self.generate_data_section(debug));

        /*
         * Generate code for each function in the file first
         */
        if debug {
            // write thread-local private variables before header
            // store branch stack pointers for branch value stack unwinding
            write!(output, "void wasm_entry(uint *stack_u32,
                                            ulong *stack_u64,
                                            uint *heap_u32,
                                            ulong *heap_u64,
                                            uint *stack_frames,
                                            ulong *sp,
                                            ulong *sfp,
                                            ulong *call_stack,
                                            ushort *branch_value_stack_state,
                                            ushort *loop_value_stack_state,
                                            int *hypercall_number,
                                            uint *hypercall_continuation,
                                            uint entry_point) {{\n");
            // for debugging hardcode the warp_idx to 0
            write!(output, "\tulong warp_idx = 0;\n");
        } else {
            let header = format!("__kernel void wasm_entry(__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}\n\t__global {}) {{\n",
                                    "uint  *stack_u32_global,",
                                    "ulong *stack_u64_global,",
                                    "uint  *heap_u32_global,",
                                    "ulong *heap_u64_global,",
                                    "uint  *stack_frames_global,",
                                    "ulong *sp_global,",
                                    "ulong *sfp_global,",
                                    "ulong *call_stack_global,",
                                    "ushort *branch_value_stack_state_global,",
                                    "ushort *loop_value_stack_state_global,",
                                    "int *hypercall_number_global,",
                                    "uint *hypercall_continuation_global,",
                                    "uint  *entry_point_global");
            // write thread-local private variables before header

            write!(output, "{}", header);
            // TODO: for the openCL launcher, pass the memory stride as a function parameter
            if interleave > 0 {
                write!(output, "\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\n\t{}\n",
                "uint  *stack_u32    = (uint*)stack_u32_global;",
                "ulong *stack_u64    = (ulong*)stack_u32;",
                "uint  *heap_u32     = (uint *)heap_u32_global;",
                "ulong *heap_u64     = (ulong *)heap_u32;",
                "uint  *stack_frames = (uint*)stack_frames_global;",
                // only an array of N elements, where N=warp size
                "ulong *sp           = (ulong *)sp_global+(get_global_id(0));",
                // the stack frame pointer is used for both the stack frame, and call stack as they are
                // essentially the same structure, except they hold different values
                "ulong *sfp          = (ulong*)sfp_global+(get_global_id(0) * 1024 * 16);",
                // holds the numeric index of the return label for where to jump after a function call
                "ulong *call_stack   = (ulong*)call_stack_global+(get_global_id(0) * 1024 * 16);",
                "ushort *branch_value_stack_state   = (ushort*)branch_value_stack_state_global;",
                "ushort *loop_value_stack_state   = (ushort*)loop_value_stack_state_global;",
                "int *hypercall_number = (int *)hypercall_number_global+(get_global_id(0));",
                "uint *hypercall_continuation = (uint *)hypercall_continuation_global+(get_global_id(0));",
                "uint  entry_point   = entry_point_global[get_global_id(0)];",
                "ulong warp_idx = get_global_id(0);");
            } else {
                write!(output, "\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\n\t{}\n",
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
                "ulong *call_stack   = (ulong*)call_stack_global+(get_global_id(0) * 1024 * 16);",
                "ulong *branch_value_stack_state   = (ulong*)branch_value_stack_state_global+(get_global_id(0) * 1024 * 16);",
                "ulong *loop_value_stack_state   = (ulong*)loop_value_stack_state_global+(get_global_id(0) * 1024 * 16);",
                "int *hypercall_number = (int *)hypercall_number_global+(get_global_id(0));",
                "uint *hypercall_continuation = (uint *)hypercall_continuation_global+(get_global_id(0));",
                "uint  entry_point   = entry_point_global[get_global_id(0)];",
                "ulong warp_idx = get_global_id(0);");
            }
        }
        
        // 

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

        // upon entry, first check to see if we are returning from a hypercall
        // hypercall_number is set to -1 after completing the hypercall
        write!(output, "\t{}\n", "if (*hypercall_number == -1) {");
        // if we are returning from the hypercall, goto hypercall_return_table
        write!(output, "{}", format!("\t\t{}\n", "goto hypercall_return_table;"));
        write!(output, "\t}}\n");


        // function entry point

        write!(output, "\t{}\n", "uint caller;");
        write!(output, "\t{}\n", "switch (entry_point) {");
        for key in function_idx_label.keys() {
            write!(output, "\t\tcase {}:\n", function_idx_label.get(key).unwrap());
            write!(output, "\t\t\tgoto {};\n", key);
            write!(output, "\t\t\tbreak;\n");
        }
        write!(output, "\t\tdefault:\n");
            write!(output, "\t\t\treturn;\n");
        write!(output, "\t}}\n");

        // hypercall ID tracker count, we use this number to auto-insert
        // the return stub later
        let hypercall_id_count: &mut u32 = &mut 0;

        let call_ret_map: &mut HashMap<&str, u32> = &mut HashMap::new();
        let call_ret_idx: &mut u32 = &mut 0;
        for function in funcs.clone() {
            let func = self.emit_function(function, call_ret_map, call_ret_idx,
                                          function_idx_label.clone(), hypercall_id_count, debug);
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

        // generate the hypercall return table

        write!(output, "{}\n", "hypercall_return_table:");
        write!(output, "\t{}\n", "switch (*hypercall_continuation) {");
        for count in 0..*hypercall_id_count {
            write!(output, "\t\tcase {}:\n", count);
            write!(output, "\t\t\tgoto hypercall_return_stub_{};\n", count);
            write!(output, "\t\t\tbreak;\n");
        } 
        write!(output, "\t}}\n");

        write!(output, "}}\n");

        if debug {
            write!(output, "{}", format!("int main(int argc, char *argv[]) {{\n"));
            
            write!(output, "{}", format!("\t{}\n", "uvwasi_t uvwasi;"));
            write!(output, "{}", format!("\t{}\n", "uvwasi_options_t init_options;"));
            write!(output, "{}", format!("\t{}\n", "uvwasi_options_init(&init_options);"));
            write!(output, "{}", format!("\t{}\n", "uvwasi_init(&uvwasi, &init_options);"));

            write!(output, "{}", format!("\tuint *stack_u32 = calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tulong *stack_u64 = (ulong *)stack_u32;\n"));
            write!(output, "{}", format!("\tuint *heap_u32 = (uint *)calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tulong *heap_u64 = (ulong *)calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tuint *stack_frames = calloc(1024, sizeof(uint));\n"));
            write!(output, "{}", format!("\tuint *call_stack = calloc(1024, sizeof(uint));\n"));
            // the size of this structure is proportional to how many functions there are
            // size = function count * 4096 bytes (64 x 64)
            write!(output, "{}", format!("\tuchar *branch_value_stack_state = calloc({}, sizeof(uchar));\n", funcs.len() * 4096));
            write!(output, "{}", format!("\tuchar *loop_value_stack_state = calloc({}, sizeof(uchar));\n", funcs.len() * 4096));

            write!(output, "{}", format!("\tulong sp = 0;\n"));
            write!(output, "{}", format!("\tulong sfp = 0;\n"));
            write!(output, "{}", format!("\tint hypercall_number = -2;\n"));
            write!(output, "{}", format!("\tuint hypercall_continuation = 0;\n"));
            write!(output, "{}", format!("\tstack_frames[sfp] = sp;\n"));
            write!(output, "{}", format!("\tstack_u64[0] = 0x1;\n"));
            write!(output, "{}", format!("\tsp += 2;\n"));
            write!(output, "{}", format!("\tdata_init(heap_u32);\n"));
            // TODO when calling the function get the entry_point for main


            // now we are entering the main execution loop, continue until *sp == 0
            write!(output, "{}", format!("\twhile(1) {{\n"));

            write!(output, "{}", format!("{}",
                    format!("\t\twasm_entry(stack_u32, stack_u64, heap_u32, heap_u64, stack_frames,
                                            &sp, &sfp, call_stack, branch_value_stack_state,
                                            loop_value_stack_state, &hypercall_number, &hypercall_continuation, {});\n",
                            function_idx_label.get("_start").unwrap())));

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