mod expr;
mod function;
mod stmts;
mod variables;

use crate::asm::function::compile_function;
use crate::codegen::{Mnemonic::*, Reg};
use crate::compiler::{CompilerContext, ScopeBlock};
use crate::error_handeling::error;
use crate::parser::block::{Block, BlockType};
use crate::parser::parse_file;
use crate::parser::program::ProgramItem;
use crate::parser::stmt::StmtType;
use crate::parser::types::VariableType;

use self::stmts::compile_stmt;

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

fn frame_size(mem_offset: usize) -> usize {
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
pub fn compile(cc: &mut CompilerContext, path: String) {
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
            ProgramItem::Import(next_path, idents) => {
                let mut new_path = String::new();
                new_path.push_str(next_path.as_str());
                new_path.push_str(".nmt");
                compile_lib(cc, new_path, idents);
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
                        cc.codegen.instr1(Jmp, format!(".LE{}", loc.1));
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
                        cc.codegen.instr1(Jmp, format!(".L{}", loc.0));
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
