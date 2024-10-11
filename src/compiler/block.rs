use crate::log_cerror;
use crate::parser::block::Block;

use super::stmts::compile_stmt;
use super::CompilerContext;

/*
 *  keep in mind there could be a problem when a variable wants to access
 *  somthing that added after in code but it could be a feature too :)
 */
pub fn compile_block(cc: &mut CompilerContext, block: &Block) {
    cc.scoped_blocks.push(block.clone());
    for stmt in &block.stmts {
        compile_stmt(cc, stmt, block.id).unwrap_or_else(|e| {
            cc.error();
            log_cerror!(stmt.loc, "{e}");
        });
    }
    cc.scoped_blocks.pop().unwrap();
}

pub fn compile_function_block_alrady_scoped(cc: &mut CompilerContext, block: &Block) {
    for stmt in &block.stmts {
        compile_stmt(cc, stmt, block.id).unwrap_or_else(|e| {
            cc.error();
            log_cerror!(stmt.loc, "{e}");
        });
    }
}
