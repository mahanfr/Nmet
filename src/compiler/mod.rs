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
mod expr;
mod function;
mod stmts;
mod variables;

use crate::codegen::instructions::Opr;
use crate::codegen::{register::Reg, Codegen};
use crate::compiler::{bif::Bif, function::compile_function};

use crate::codegen::text::x86_64_nasm_generator;
use crate::error_handeling::error;
use crate::parser::{
    block::{Block, BlockType},
    function::Function,
    parse_file,
    program::ProgramItem,
    stmt::StmtType,
    structs::StructDef,
    types::VariableType,
};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Display;
use std::path::PathBuf;

use self::stmts::compile_stmt;

pub fn compile(input: String, output: PathBuf, output_type: OutputType) {
    let mut compiler_context = CompilerContext::new(input.clone());

    _compile(&mut compiler_context, input.clone());
    impl_bifs(&mut compiler_context);
    let prefix = output.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    match output_type {
        OutputType::Elf => {
            println!("[info] Generating elf object file...");
            // generate_elf(output.as_path(), &mut compiler_context);
            crate::codegen::elf::generate_elf(output.as_path(), &mut compiler_context);
        }
        OutputType::Asm => {
            println!("[info] Generating asm text file...");
            x86_64_nasm_generator(output.as_path(), &compiler_context).unwrap();
        }
    }
}

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
    pub functions_map: BTreeMap<String, Function>,
    pub structs_map: HashMap<String, StructDef>,
    pub bif_set: HashSet<Bif>,
    pub ffi_map: BTreeMap<String, String>,
    pub mem_offset: usize,
    pub program_file: String,
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
            ffi_map: BTreeMap::new(),
            structs_map: HashMap::new(),
            mem_offset: 0,
        }
    }
    pub fn last_main_label(&self) -> String {
        for block in self.scoped_blocks.iter().rev() {
            let BlockType::Function(lab) = block.block_type.clone() else {
                continue;
            };
            if lab == "main" {
                return "_start".to_string();
            }
            return lab;
        }
        String::new()
    }
}

pub enum OutputType {
    Elf,
    Asm,
}
impl Display for OutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Elf => write!(f, "elf"),
            Self::Asm => write!(f, "asm"),
        }
    }
}

pub fn impl_bifs(cc: &mut CompilerContext) {
    for bif in cc.bif_set.iter() {
        bif.implement(&mut cc.codegen);
    }
}

pub fn function_args_register_sized(arg_numer: usize, vtype: &VariableType) -> Reg {
    match arg_numer {
        0 => Reg::DI_sized(vtype),
        1 => Reg::Si_sized(vtype),
        2 => Reg::DX_sized(vtype),
        3 => Reg::CX_sized(vtype),
        4 => Reg::R8_sized(vtype),
        5 => Reg::R9_sized(vtype),
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

pub fn compile_lib(cc: &mut CompilerContext, path: String, exports: Vec<String>) {
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
            ProgramItem::Struct(_) => {
                todo!();
            }
            ProgramItem::StaticVar(_s) => {
                todo!();
                // self.insert_variable(&s);
            }
            ProgramItem::Func(f) => {
                if is_importable(&f.ident) && !cc.functions_map.contains_key(&f.ident) {
                    cc.functions_map.insert(f.ident.clone(), f.clone());
                }
            }
            ProgramItem::FFI(mod_name, f) => {
                if is_importable(&f.ident) && !cc.functions_map.contains_key(&f.ident) {
                    cc.ffi_map.insert(f.ident.clone(), mod_name);
                    cc.functions_map.insert(f.ident.clone(), f.clone());
                }
            }
            ProgramItem::Import(next_path, idents) => {
                let mut new_path = String::new();
                new_path.push_str(next_path.as_str());
                new_path.push_str(".nmt");
                compile_lib(cc, new_path, idents);
            }
        }
    }
}

// TODO: Handle Compilation Error
pub fn _compile(cc: &mut CompilerContext, path: String) {
    let program = parse_file(path);
    for item in program.items {
        match item {
            ProgramItem::Struct(s) => {
                cc.structs_map.insert(s.ident.clone(), s.clone());
            }
            ProgramItem::StaticVar(_s) => {
                todo!();
                // self.insert_variable(&s);
            }
            ProgramItem::Func(f) => {
                cc.functions_map.insert(f.ident.clone(), f.clone());
            }
            ProgramItem::FFI(mod_name, f) => {
                cc.ffi_map.insert(f.ident.clone(), mod_name);
                cc.functions_map.insert(f.ident.clone(), f.clone());
            }
            ProgramItem::Import(next_path, idents) => {
                let mut new_path = String::new();
                new_path.push_str(next_path.as_str());
                new_path.push_str(".nmt");
                compile_lib(cc, new_path, idents);
            }
        }
    }
    let functions = cc.functions_map.clone();
    compile_function(cc, functions.get("main").unwrap());
    for (k, f) in functions.iter() {
        if k != "main" || cc.ffi_map.get(k).is_some() {
            compile_function(cc, f);
        }
    }
    assert!(
        cc.scoped_blocks.is_empty(),
        "Somting went wrong: Scope has not been cleared"
    );
}

/*
 *  keep in mind there could be a problem when a variable wants to access
 *  somthing that added after in code but it could be a feature too :)
 */
fn compile_block(cc: &mut CompilerContext, block: &Block, block_type: BlockType) {
    cc.block_id += 1;
    cc.scoped_blocks
        .push(ScopeBlock::new(cc.block_id, block_type));
    for stmt in &block.stmts {
        match stmt.stype {
            StmtType::Break => {
                let mut did_break: bool = false;
                for s_block in cc.scoped_blocks.iter().rev() {
                    if let BlockType::Loop(loc) = s_block.block_type {
                        cc.codegen.instr1(
                            crate::codegen::mnemonic::Mnemonic::Jmp,
                            Opr::Rel(format!("{}.LE{}", cc.last_main_label(), loc.1)),
                        );
                        did_break = true;
                        break;
                    }
                }
                if !did_break {
                    error("Can not break out of non-loop blocks!", stmt.loc.clone());
                }
            }
            StmtType::Continue => {
                let mut did_cont: bool = false;
                for s_block in cc.scoped_blocks.iter().rev() {
                    if let BlockType::Loop(loc) = s_block.block_type {
                        // assert!(false, "Not Implemented yet!");
                        cc.codegen.instr1(
                            crate::codegen::mnemonic::Mnemonic::Jmp,
                            Opr::Rel(format!("{}.L{}", cc.last_main_label(), loc.1)),
                        );
                        //cc.codegen.push_instr(Instr::jmp(0));
                        did_cont = true;
                        break;
                    }
                }
                if !did_cont {
                    error("Can not continue in non-loop blocks!", stmt.loc.clone());
                }
            }
            _ => {
                compile_stmt(cc, stmt);
            }
        }
    }
    cc.block_id -= 1;
    cc.scoped_blocks.pop().unwrap();
}
