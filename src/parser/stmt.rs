use crate::error_handeling::Loc;
use crate::lexer::{Lexer, TokenType};
use crate::nemet_macros::MacroCall;
use crate::parser::block::Block;
use crate::parser::expr::Expr;

use super::assign::Assign;
use super::block::block;
use super::expr::expr;
use super::variable_decl::VariableDeclare;

#[derive(Debug, Clone)]
pub struct Stmt {
    pub stype: StmtType,
    pub loc: Loc,
}

#[derive(Debug, Clone)]
pub enum StmtType {
    // expr
    Expr(Expr),
    VariableDecl(VariableDeclare),
    // expr = expr
    Assign(Assign),
    Print(Expr),
    While(WhileStmt),
    If(IFStmt),
    Return(Expr),
    InlineAsm(Vec<String>),
    MacroCall(MacroCall),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct IFStmt {
    pub condition: Expr,
    pub then_block: Block,
    pub else_block: Box<ElseBlock>,
}

#[derive(Debug, Clone)]
pub enum ElseBlock {
    Elif(IFStmt),
    Else(Block),
    None,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub block: Block,
}

/*
 * Stmt := {declare | expr { = expr}} ;
 * declare := let Ident = expr;
*/

pub fn if_stmt(lexer: &mut Lexer) -> IFStmt {
    lexer.match_token(TokenType::If);
    let condition = expr(lexer);
    let then_block = block(lexer);
    if lexer.get_token_type() == TokenType::Else {
        lexer.match_token(TokenType::Else);
        if lexer.get_token_type() == TokenType::If {
            let else_block = Box::new(ElseBlock::Elif(if_stmt(lexer)));
            IFStmt {
                condition,
                then_block,
                else_block,
            }
        } else {
            let else_block = Box::new(ElseBlock::Else(block(lexer)));
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

pub fn while_stmt(lexer: &mut Lexer) -> WhileStmt {
    lexer.match_token(TokenType::While);
    let condition = expr(lexer);
    let block = block(lexer);
    WhileStmt { condition, block }
}
