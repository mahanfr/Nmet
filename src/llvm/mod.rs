mod expr;
mod function;
mod stmts;
mod variables;

use std::error::Error;

use crate::compiler::CompilerContext;
use crate::llvm::function::compile_function;
use crate::output_generator::llvm_generator;
use crate::parser::block::Block;
use crate::parser::parse_file;
use crate::parser::program::ProgramItem;

use self::stmts::compile_stmt;

pub fn compile_to_llvm(path: String) {
    let mut compiler_context = CompilerContext::new();

    let (instr_buf, data_buf) =
        compile(&mut compiler_context, path.clone()).expect("Can not Compile Program");
    llvm_generator(path, instr_buf, data_buf).unwrap();
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
