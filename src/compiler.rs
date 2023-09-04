use std::collections::HashMap;

use crate::{parser::{types::VariableType, function::Function}, asm::compile, output_generator::x86_64_nasm_generator};

#[derive(Debug, Clone)]
pub struct VariableMap {
    pub _ident: String,
    pub offset: usize,
    pub vtype: VariableType,
    pub is_mut: bool,
}

pub fn find_variable(cc: &CompilerContext, ident: String) -> Option<VariableMap> {
    for block_id in &cc.scoped_blocks {
        let map_ident = format!("{ident}%{}", block_id);
        let map = cc.variables_map.get(&map_ident);
        if let Some(map) = map {
            return Some(map.clone());
        }
    }
    None
}

pub enum OutputFormat {
    X86_64Linux,
    LLVM,
}

pub struct CompilerContext {
    pub instruct_buf: Vec<String>,
    pub data_buf: Vec<String>,
    pub scoped_blocks: Vec<usize>,
    pub block_id: usize,
    pub variables_map: HashMap<String, VariableMap>,
    pub functions_map: HashMap<String, Function>,
    pub mem_offset: usize,
    pub output_foramt: OutputFormat,
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

pub fn compile_to_asm(path: String) {
    let mut compiler_context = CompilerContext::new();

    let (instr_buf, data_buf) =
        compile(&mut compiler_context, path.clone()).expect("Can not Compile Program");
    x86_64_nasm_generator(path, instr_buf, data_buf).unwrap();
}

