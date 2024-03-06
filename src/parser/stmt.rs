/**********************************************************************************************
*
*   parser/stmt: parsing statements that are parts that directly compile to instructions
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
use crate::error_handeling::Loc;
use crate::lexer::{Lexer, TokenType};
use crate::parser::block::Block;
use crate::parser::expr::Expr;

use super::assign::Assign;
use super::block::{parse_block, BlockType};
use super::expr::expr;
use super::variable_decl::VariableDeclare;

/// Statment
/// * stype: Type of the statment
/// * loc: Location of the statment
#[derive(Debug, Clone)]
pub struct Stmt {
    pub stype: StmtType,
    pub loc: Loc,
}

/// All Supported Stmt Types
#[derive(Debug, Clone)]
pub enum StmtType {
    /// Single expr usually a function call with void return type
    Expr(Expr),
    /// Variable Declaration
    VariableDecl(VariableDeclare),
    /// Assginment
    Assign(Assign),
    /// Prints the expression
    /// WILL BE REMOVED IN THE FUTURE
    Print(Expr),
    /// While loops
    While(WhileStmt),
    /// If Stmts
    If(IFStmt),
    /// Return Stmts
    Return(Expr),
    /// Inline Assembly
    InlineAsm(Vec<String>),
    /// Break Stmts
    /// NOT IMPLEMENTED YET
    Break,
    /// CONTINUE Stmts
    /// NOT IMPLEMENTED YET
    Continue,
}

/// If Stmt Information
/// * condition - if stmt condition expression
/// * then_block - first block after condition
/// * else_block - else block or elif stmt
#[derive(Debug, Clone)]
pub struct IFStmt {
    pub condition: Expr,
    pub then_block: Block,
    pub else_block: Box<ElseBlock>,
}

/// Else Block Types
#[derive(Debug, Clone)]
pub enum ElseBlock {
    /// Else If
    Elif(IFStmt),
    /// Else
    Else(Block),
    /// If Stmt with no else block
    None,
}

/// While Statment Information
/// * condition - conditional expr runs until not true
/// * block - while loop body
#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub block: Block,
}

/// Parse If Stmts
pub fn if_stmt(lexer: &mut Lexer, master: &String) -> IFStmt {
    lexer.match_token(TokenType::If);
    let condition = expr(lexer);
    let then_block = Block::new(
        master.to_owned(),
        BlockType::Condition,
        parse_block(lexer, master),
    );
    if lexer.get_token_type() == TokenType::Else {
        lexer.match_token(TokenType::Else);
        if lexer.get_token_type() == TokenType::If {
            let else_block = Box::new(ElseBlock::Elif(if_stmt(lexer, master)));
            IFStmt {
                condition,
                then_block,
                else_block,
            }
        } else {
            let else_block = Box::new(ElseBlock::Else(Block::new(
                master.to_owned(),
                BlockType::Condition,
                parse_block(lexer, master),
            )));
            IFStmt {
                condition,
                then_block,
                else_block,
            }
        }
    } else {
        IFStmt {
            condition,
            then_block,
            else_block: Box::new(ElseBlock::None),
        }
    }
}

/// Parse While Stmts
pub fn while_stmt(lexer: &mut Lexer, master: &String) -> WhileStmt {
    lexer.match_token(TokenType::While);
    let condition = expr(lexer);
    let block = Block::new(
        master.to_owned(),
        BlockType::Loop,
        parse_block(lexer, master),
    );
    WhileStmt { condition, block }
}
