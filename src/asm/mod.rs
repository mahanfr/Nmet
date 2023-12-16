mod expr;
mod function;
mod stmts;
mod variables;

use std::error::Error;

use crate::asm::function::compile_function;
use crate::codegen::{Codegen, R};
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

pub fn function_args_register_sized(arg_numer: usize, vtype: &VariableType) -> R {
    match arg_numer {
        0 => R::DI_s(vtype),
        1 => R::Si_s(vtype),
        2 => R::D_s(vtype),
        3 => R::C_s(vtype),
        4 => R::R8_s(vtype),
        5 => R::R9_s(vtype),
        _ => unreachable!(),
    }
}

pub fn function_args_register(arg_numer: usize) -> R {
    match arg_numer {
        0 => R::RDI,
        1 => R::RSI,
        2 => R::RDX,
        3 => R::RCX,
        4 => R::R8,
        5 => R::R9,
        _ => unreachable!(),
    }
}

fn frame_size(mem_offset: usize) -> usize {
    2 << mem_offset.ilog2() as usize
}

pub fn compile_lib(
    cc: &mut CompilerContext,
    path: String,
    exports: Vec<String>,
) -> Result<Codegen, Box<dyn Error>> {
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
                compile_lib(cc, new_path, idents)?;
            }
        }
    }
    Ok(cc.codegen.clone())
}

// TODO: Handle Compilation Error
pub fn compile(cc: &mut CompilerContext, path: String) -> Result<Codegen, Box<dyn Error>> {
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
    Ok(cc.codegen.clone())
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
                        cc.codegen.jmp(format!(".LE{}", loc.1));
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
                        cc.codegen.jmp(format!(".L{}", loc.0));
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
