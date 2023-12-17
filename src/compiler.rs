use std::collections::{HashMap, HashSet};

use crate::{
    asm,
    bif::Bif,
    codegen::Codegen,
    output_generator::x86_64_nasm_generator,
    parser::{block::BlockType, function::Function, structs::StructDef, types::VariableType},
};

#[derive(Debug, Clone)]
pub struct VariableMap {
    pub offset_inner: usize,
    pub offset: usize,
    pub vtype: VariableType,
    pub vtype_inner: VariableType,
    pub is_mut: bool,
}

pub type BLocation = (usize, usize);

pub struct ScopeBlock {
    pub id: usize,
    pub block_type: BlockType,
}
impl ScopeBlock {
    pub fn new(id: usize, block_type: BlockType) -> Self {
        Self { id, block_type }
    }
}

pub struct CompilerContext {
    pub codegen: Codegen,
    pub scoped_blocks: Vec<ScopeBlock>,
    pub block_id: usize,
    pub variables_map: HashMap<String, VariableMap>,
    pub functions_map: HashMap<String, Function>,
    pub structs_map: HashMap<String, StructDef>,
    pub bif_set: HashSet<Bif>,
    pub mem_offset: usize,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            codegen: Codegen::new(),
            scoped_blocks: Vec::new(),
            block_id: 0,
            bif_set: HashSet::new(),
            variables_map: HashMap::new(),
            functions_map: HashMap::new(),
            structs_map: HashMap::new(),
            mem_offset: 0,
        }
    }
}

pub fn compile_to_asm(path: String) {
    let mut compiler_context = CompilerContext::new();

    asm::compile(&mut compiler_context, path.clone());
    x86_64_impl_bifs(&mut compiler_context);
    x86_64_nasm_cleanup(&mut compiler_context.codegen);
    x86_64_nasm_generator(path, compiler_context.codegen).unwrap();
}

pub fn x86_64_impl_bifs(cc: &mut CompilerContext) {
    for bif in cc.bif_set.iter() {
        bif.implement(&mut cc.codegen);
    }
}

fn x86_64_nasm_cleanup(code: &mut Codegen) {
    for i in 0..(code.instruct_buf.len() - 2) {
        if code.instruct_buf[i].trim_start().starts_with("push")
            && code.instruct_buf[i + 1].trim_start().starts_with("pop")
        {
            let merged: String = merge_instr(&code.instruct_buf[i], &code.instruct_buf[i + 1]);
            code.instruct_buf[i].clear();
            code.instruct_buf[i + 1] = merged;
        }
    }
    code.instruct_buf.retain(|x| !x.is_empty());
}

fn merge_instr(ins1: &str, inst2: &str) -> String {
    let data1 = ins1.split(' ').last().unwrap();
    let data2 = inst2.split(' ').last().unwrap();
    if data1 == data2 {
        String::new()
    } else {
        format!("    mov {data2}, {data1}")
    }
}
