use std::collections::HashMap;

use crate::{
    asm,
    output_generator::x86_64_nasm_generator,
    parser::{function::Function, types::VariableType, block::BlockType, structs::StructDef},
};

#[derive(Debug, Clone)]
pub struct VariableMap {
    pub offset_inner: usize, 
    pub offset: usize,
    pub vtype: VariableType,
    pub vtype_inner: VariableType,
    pub is_mut: bool,
}


pub type BLocation = (usize,usize);

pub struct ScopeBlock {
    pub id: usize,
    pub block_type: BlockType,
}
impl ScopeBlock {
    pub fn new(id: usize, block_type: BlockType) -> Self {
        Self {
            id,
            block_type,
        }
    }
}

pub struct CompilerContext {
    pub instruct_buf: Vec<String>,
    pub data_buf: Vec<String>,
    pub bss_buf: Vec<String>,
    pub scoped_blocks: Vec<ScopeBlock>,
    pub block_id: usize,
    pub variables_map: HashMap<String, VariableMap>,
    pub functions_map: HashMap<String, Function>,
    pub structs_map: HashMap<String, StructDef>,
    pub mem_offset: usize,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            instruct_buf: Vec::new(),
            data_buf: Vec::new(),
            bss_buf: Vec::new(),
            scoped_blocks: Vec::new(),
            block_id: 0,
            variables_map: HashMap::new(),
            functions_map: HashMap::new(),
            structs_map: HashMap::new(),
            mem_offset: 0,
        }
    }
}

pub fn compile_to_asm(path: String) {
    let mut compiler_context = CompilerContext::new();

    let (instr_buf, bss_buf, data_buf) =
        asm::compile(&mut compiler_context, path.clone()).expect("Can not Compile Program");
    x86_64_nasm_generator(path, instr_buf, bss_buf, data_buf).unwrap();
}

// pub fn compile_to_llvm(path: String) {
//     let mut compiler_context = CompilerContext::new();
//     let (instr_buf, data_buf) =
//         llvm::compile(&mut compiler_context, path.clone()).expect("Can not Compile Program");
//     llvm_generator(path, instr_buf, data_buf).unwrap();
// }
