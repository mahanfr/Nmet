mod expr;
mod function;
mod stmts;
mod variables;

use std::collections::HashMap;
use std::error::Error;

use crate::llvm::function::compile_function;
use crate::output_generator::x86_64_nasm_generator;
use crate::parser::block::Block;
use crate::parser::function::Function;
use crate::parser::parse_file;
use crate::parser::program::ProgramItem;
use crate::parser::types::VariableType;

use self::stmts::compile_stmt;
use self::variables::VariableMap;

pub fn compile_to_asm(path: String) {
    let mut compiler_context = CompilerContext::new();

    let (instr_buf, data_buf) =
        compile(&mut compiler_context, path.clone()).expect("Can not Compile Program");
    x86_64_nasm_generator(path, instr_buf, data_buf).unwrap();
}

pub fn mem_word(vtype: &VariableType) -> String {
    let size = vtype.item_size();
    match size {
        1 => "byte".to_string(),
        2 => "word".to_string(),
        4 => "dword".to_string(),
        8 => "qword".to_string(),
        _ => {
            unreachable!("Incurrect Size")
        }
    }
}

pub fn rbs(register: &str, vtype: &VariableType) -> String {
    let size = vtype.item_size();
    match register {
        "a" | "b" | "c" | "d" => match size {
            1 => format!("{register}l"),
            2 => format!("{register}x"),
            4 => format!("e{register}x"),
            8 => format!("r{register}x"),
            _ => {
                unreachable!("Incurrect Size")
            }
        },
        "sp" | "bp" => match size {
            1 => format!("{register}l"),
            2 => register.to_string(),
            4 => format!("e{register}"),
            8 => format!("r{register}"),
            _ => {
                unreachable!("Incurrect Size")
            }
        },
        "si" | "di" => match size {
            1 => format!("{register}l"),
            2 => register.to_string(),
            4 => format!("e{register}"),
            8 => format!("r{register}"),
            _ => {
                unreachable!("Incurrect Size")
            }
        },
        "r8" | "r9" | "r10" | "r11" => match size {
            1 => format!("{register}b"),
            2 => format!("{register}w"),
            4 => format!("{register}d"),
            8 => register.to_string(),
            _ => {
                unreachable!("Incurrect Size")
            }
        },
        _ => {
            panic!("Wrong register identifier!");
        }
    }
}

pub fn function_args_register_sized(arg_numer: usize, vtype: &VariableType) -> String {
    match arg_numer {
        0 => rbs("di", vtype),
        1 => rbs("si", vtype),
        2 => rbs("d", vtype),
        3 => rbs("c", vtype),
        4 => rbs("r8", vtype),
        5 => rbs("r9", vtype),
        _ => unreachable!(),
    }
}

pub fn function_args_register(arg_numer: usize) -> String {
    match arg_numer {
        0 => "rdi".to_string(),
        1 => "rsi".to_string(),
        2 => "rdx".to_string(),
        3 => "rcx".to_string(),
        4 => "r8".to_string(),
        5 => "r9".to_string(),
        _ => unreachable!(),
    }
}

pub enum OutputFormat {
    X86_64Linux,
    LLVM,
}

pub struct CompilerContext {
    instruct_buf: Vec<String>,
    data_buf: Vec<String>,
    scoped_blocks: Vec<usize>,
    block_id: usize,
    variables_map: HashMap<String, VariableMap>,
    functions_map: HashMap<String, Function>,
    mem_offset: usize,
    output_foramt: OutputFormat,
}

impl CompilerContext {
    // TODO: handle Error for Parsing
    pub fn new() -> Self {
        Self {
            instruct_buf: Vec::new(),
            data_buf: Vec::new(),
            scoped_blocks: Vec::new(),
            block_id: 0,
            variables_map: HashMap::new(),
            functions_map: HashMap::new(),
            mem_offset: 0,
            output_foramt: OutputFormat::X86_64Linux,
        }
    }
}
fn frame_size(mem_offset: usize) -> usize {
    2 << mem_offset.ilog2() as usize
}

pub fn compile_lib(
    cc: &mut CompilerContext,
    path: String,
    exports: Vec<String>,
) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let program = parse_file(path);
    let is_importable = |ident: &String| {
        if !exports.is_empty() {
            exports.contains(ident)
        } else {
            true
        }
    };
    for item in program.items {
        match item {
            ProgramItem::StaticVar(_s) => {
                todo!();
                // self.insert_variable(&s);
            }
            ProgramItem::Func(f) => {
                if is_importable(&f.ident) && !cc.functions_map.contains_key(&f.ident) {
                    cc.functions_map.insert(f.ident.clone(), f.clone());
                }
            }
            ProgramItem::Import(next_path, idents) => {
                let mut new_path = String::new();
                new_path.push_str(next_path.as_str());
                new_path.push_str(".nmt");
                compile_lib(cc, new_path, idents)?;
            }
        }
    }
    Ok((cc.instruct_buf.clone(), cc.data_buf.clone()))
}

// TODO: Handle Compilation Error
pub fn compile(
    cc: &mut CompilerContext,
    path: String,
) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let program = parse_file(path);
    for item in program.items {
        match item {
            ProgramItem::StaticVar(_s) => {
                todo!();
                // self.insert_variable(&s);
            }
            ProgramItem::Func(f) => {
                cc.functions_map.insert(f.ident.clone(), f.clone());
            }
            ProgramItem::Import(next_path, idents) => {
                let mut new_path = String::new();
                new_path.push_str(next_path.as_str());
                new_path.push_str(".nmt");
                compile_lib(cc, new_path, idents)?;
            }
        }
    }
    let functions = cc.functions_map.clone();
    for f in functions.values() {
        compile_function(cc, f);
    }
    assert!(
        cc.scoped_blocks.is_empty(),
        "Somting went wrong: Scope has not been cleared"
    );
    Ok((cc.instruct_buf.clone(), cc.data_buf.clone()))
}

/*
 *  keep in mind there could be a problem when a variable wants to access
 *  somthing that added after in code but it could be a feature too :)
 */
fn compile_block(cc: &mut CompilerContext, block: &Block) {
    cc.block_id += 1;
    cc.scoped_blocks.push(cc.block_id);
    for stmt in &block.stmts {
        compile_stmt(cc, stmt);
    }
    cc.block_id -= 1;
    cc.scoped_blocks.pop().unwrap();
}
