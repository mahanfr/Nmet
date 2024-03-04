/**********************************************************************************************
*
*   compiler/mod: Compiler Context and compile from file
*
*   LICENSE: MIT
*
*   Copyright (c) 2023-2024 Mahan Farzaneh (@mahanfr)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/
pub mod bif;
mod block;
mod expr;
mod function;
mod stmts;
mod variables;

use crate::codegen::instructions::Opr;
use crate::codegen::mnemonic::Mnemonic;
use crate::codegen::{register::Reg, Codegen};
use crate::compiler::{bif::Bif, function::compile_function};

use crate::log_error;
use crate::parser::{
    block::BlockType, function::Function, parse_file, structs::StructDef,
    types::VariableType,
};
use crate::type_check::Identifier;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::process::exit;

use self::block::ScopeBlock;

#[derive(Debug, Clone)]
pub struct VariableMap {
    pub offset_inner: usize,
    pub offset: usize,
    pub vtype: VariableType,
    pub vtype_inner: VariableType,
    pub is_mut: bool,
}

impl VariableMap {
    pub fn stack_offset(&self) -> i32 {
        -((self.offset + self.vtype.size()) as i32)
    }
}

pub type BLocation = (usize, usize);

pub struct CompilerContext {
    pub codegen: Codegen,
    pub scoped_blocks: Vec<ScopeBlock>,
    pub block_id: usize,
    pub variables_map: HashMap<String, VariableMap>,
    pub functions_map: BTreeMap<String, Function>,
    pub structs_map: HashMap<String, StructDef>,
    pub bif_set: HashSet<Bif>,
    pub mem_offset: usize,
    pub program_file: String,
    errors: usize,
}

impl CompilerContext {
    pub fn new(program_file: String) -> Self {
        Self {
            program_file,
            codegen: Codegen::new(),
            scoped_blocks: Vec::new(),
            block_id: 0,
            bif_set: HashSet::new(),
            variables_map: HashMap::new(),
            functions_map: BTreeMap::new(),
            structs_map: HashMap::new(),
            mem_offset: 0,
            errors: 0,
        }
    }
    pub fn error(&mut self) {
        self.errors += 1;
    }

    pub fn last_main_label(&self) -> String {
        for block in self.scoped_blocks.iter().rev() {
            let BlockType::Function(lab) = block.block_type.clone() else {
                continue;
            };
            return lab;
        }
        String::new()
    }
}

pub fn impl_bifs(cc: &mut CompilerContext) {
    for bif in cc.bif_set.iter() {
        bif.implement(&mut cc.codegen);
    }
}

pub fn function_args_register_sized(arg_numer: usize, vtype: &VariableType) -> Reg {
    match arg_numer {
        0 => Reg::RDI.convert(vtype.item_size()),
        1 => Reg::RSI.convert(vtype.item_size()),
        2 => Reg::RDX.convert(vtype.item_size()),
        3 => Reg::RCX.convert(vtype.item_size()),
        4 => Reg::R8.convert(vtype.item_size()),
        5 => Reg::R9.convert(vtype.item_size()),
        _ => unreachable!(),
    }
}

pub fn function_args_register(arg_numer: usize) -> Reg {
    match arg_numer {
        0 => Reg::RDI,
        1 => Reg::RSI,
        2 => Reg::RDX,
        3 => Reg::RCX,
        4 => Reg::R8,
        5 => Reg::R9,
        _ => unreachable!(),
    }
}

fn _frame_size(mem_offset: usize) -> usize {
    2 << mem_offset.ilog2() as usize
}

// TODO: Handle Compilation Error
pub fn compile(cc: &mut CompilerContext, path: String) {
    let _program = parse_file(cc, path);
    let functions = cc.functions_map.clone();
    compile_init_function(cc);
    for (k, f) in functions.iter() {
        if cc.codegen.ffi_map.get(k).is_none() {
            compile_function(cc, f);
        }
    }
    if cc.errors > 0 {
        log_error!("Compilation Failed due to {} previous errors!", cc.errors);
        exit(-1);
    }
    assert!(
        cc.scoped_blocks.is_empty(),
        "Somting went wrong: Scope has not been cleared"
    );
}

fn compile_init_function(cc: &mut CompilerContext) {
    cc.codegen.set_lable("_start");
    // TODO: Add a condition for compiling libraries
    if cc.functions_map.get("main").is_none() {
        log_error!("Executable programs should have an entry point");
        exit(-1);
    }
    cc.codegen.instr1(Mnemonic::Call, Opr::Loc("main".to_owned()));
    cc.codegen.instr2(Mnemonic::Mov, Opr::R64(Reg::RAX), 60);
    cc.codegen.instr2(Mnemonic::Mov, Opr::R64(Reg::RDI), 0);
    cc.codegen.instr0(Mnemonic::Syscall);
}

