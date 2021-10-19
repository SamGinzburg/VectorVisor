/*
 * In order to gain optimal performance, we have to perform binary patching on input programs,
 * mostly to optimize failure paths that perform indirect function calls. (e.g.
 * handle_alloc_error).
 */

use crate::opencl_writer::OpenCLCWriter;
use wast::Wat;
use wast::parser::{self, ParseBuffer};
use wast::ModuleKind::{Text, Binary};
use std::sync::Arc;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::iter::FromIterator;
use regex::Regex;
use crate::opencl_writer::format_fn_name;
use crate::opencl_writer::get_func_params;
use crate::opencl_writer::get_func_result;

pub const PATCH_FILE: &'static str = include_str!("patch/do_reserve_and_handle.wat");

fn get_function_hash(f: wast::Func) -> u64 {
    let mut hash = DefaultHasher::new();
    match (&f.kind) {
        (wast::FuncKind::Import(_)) => {
            // In this case, we have an InlineImport of the form:
            // (func (type 3) (import "foo" "bar"))
            panic!("InlineImport functions not yet implemented");
        },
        (wast::FuncKind::Inline{locals, expression}) => {
            for instr in expression.instrs.iter() {
                hash.write(format!("{:?}", instr).as_bytes());
            }
        }
    }

    return hash.finish();
}

impl<'a> OpenCLCWriter<'_> {
    fn check_param_equivalence(&self, f1: &wast::Func, f2: &wast::Func) -> bool {
        let f1_params = get_func_params(&self, &f1.ty);
        let f2_params = get_func_params(&self, &f2.ty);
        let f1_result = get_func_result(&self, &f1.ty);
        let f2_result = get_func_result(&self, &f2.ty);
        f1_params == f2_params && f1_result == f2_result
    }
    pub fn patch_binary(&mut self) -> () {
        let mut fnames: Vec<String> = vec![];
        for (name, _) in &self.func_map {
            fnames.push(name.clone());
        }

        let hash_strip = Regex::new(r#"::\w+$"#).unwrap();

        for func_name in &fnames {
            let pb = Box::leak(Box::new(ParseBuffer::new(PATCH_FILE).unwrap()));
            let module = parser::parse::<Wat>(pb).unwrap();

            match module.module.kind {
                Text(t) => {
                    for item in t {
                        match item {
                            wast::ModuleField::Func(f) => {
                                let mut f_name = match f.id {
                                    Some(f_id) => {
                                        f_id.name().to_string()
                                    },
                                    // possible TODO: patch using hashed function values?
                                    None => String::from(""),
                                };
                                let demangle_name = match rustc_demangle::try_demangle(&func_name) {
                                    Ok(name) => {
                                        name.to_string()
                                    },
                                    Err(_) => {
                                        func_name.to_string()
                                    },
                                };
                                /*
                                 * We check both the name and type signature here
                                 */
                                match &f_name as &str {
                                    "alloc::raw_vec::RawVec<T_A>::reserve::do_reserve_and_handle::two" |
                                    "alloc::raw_vec::RawVec<T_A>::reserve::do_reserve_and_handle::three"
                                        => {
                                        // remove the function hash for the name check
                                        let f_name_prefix = format_fn_name(&f_name);
                                        let demangle_name_prefix = format_fn_name(&demangle_name);

                                        // check if function params & return value are equivalent
                                        let current_func = self.func_map.get(&func_name as &str).unwrap();
                                        if demangle_name_prefix.contains(&f_name_prefix) && self.check_param_equivalence(&current_func, &f) {
                                            println!("Patching {:?}", demangle_name);
                                            //self.func_map.insert(demangle_name.to_string(), f);
                                        }
                                    },
                                    "alloc::raw_vec::finish_grow::custom" |
                                    "__rust_realloc" |
                                    "__rust_alloc"
                                        => {
                                        // Inject missing helper functions when needed
                                        if !self.func_map.contains_key(&f_name) {
                                            //self.func_map.insert(f_name.to_string(), f);
                                        }
                                    }
                                    _ => (),
                                }
                            },
                            _ => (),
                        }
                    }
                },
                Binary(_) => (),
            }
        }
    }
}