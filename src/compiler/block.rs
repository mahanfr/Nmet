use crate::codegen::instructions::Opr;

use crate::log_cerror;
use crate::parser::{
    block::{Block, BlockType},
    stmt::StmtType,
};

use super::stmts::compile_stmt;
use super::CompilerContext;

pub struct ScopeBlock {
    pub id: usize,
    pub block_type: BlockType,
}
impl ScopeBlock {
    pub fn new(id: usize, block_type: BlockType) -> Self {
        Self { id, block_type }
    }
}

pub struct BlockIR {}

/*
 *  keep in mind there could be a problem when a variable wants to access
 *  somthing that added after in code but it could be a feature too :)
 */
pub fn compile_block(cc: &mut CompilerContext, block: &Block, block_type: BlockType) -> BlockIR {
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
                            Opr::Loc(format!("{}.LE{}", cc.last_main_label(), loc.1)),
                        );
                        did_break = true;
                        break;
                    }
                }
                if !did_break {
                    cc.error();
                    log_cerror!(stmt.loc, "Can not break in non-loop blocks!");
                }
            }
            StmtType::Continue => {
                let mut did_cont: bool = false;
                for s_block in cc.scoped_blocks.iter().rev() {
                    if let BlockType::Loop(loc) = s_block.block_type {
                        cc.codegen.instr1(
                            crate::codegen::mnemonic::Mnemonic::Jmp,
                            Opr::Loc(format!("{}.L{}", cc.last_main_label(), loc.1)),
                        );
                        did_cont = true;
                        break;
                    }
                }
                if !did_cont {
                    cc.error();
                    log_cerror!(stmt.loc, "Can not continue in non-loop blocks!");
                }
            }
            _ => {
                compile_stmt(cc, stmt).unwrap_or_else(|e| {
                    cc.error();
                    log_cerror!(stmt.loc, "{e}");
                });
            }
        }
    }
    cc.block_id -= 1;
    cc.scoped_blocks.pop().unwrap();
    BlockIR {}
}
