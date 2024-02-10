/**********************************************************************************************
*
*   parser/block: parse blocks syntax (e.g: if/while block)
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
use crate::{
    compiler::BLocation,
    lexer::{Lexer, TokenType},
    parser::stmt::Stmt,
};

use super::{
    assign::assign,
    expr::expr,
    stmt::{if_stmt, while_stmt, StmtType},
    variable_decl::variable_declare,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    Condition,
    Loop(BLocation),
    Function(String),
}

/// Block Stmt
/// Holds a list of stmt in a block of code
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

/// Parse Blocks
/// # Argumenrs
/// * lexer - address of mutable lexer
/// Returns a Block
pub fn block(lexer: &mut Lexer) -> Block {
    lexer.match_token(TokenType::OCurly);
    let mut stmts = Vec::<Stmt>::new();
    loop {
        if lexer.get_token_type() == TokenType::CCurly {
            break;
        }
        match lexer.get_token_type() {
            TokenType::Var => {
                let loc = lexer.get_token_loc();
                stmts.push(Stmt {
                    stype: StmtType::VariableDecl(variable_declare(lexer)),
                    loc,
                });
                lexer.match_token(TokenType::SemiColon);
            }
            TokenType::Print => {
                let loc = lexer.get_token_loc();
                lexer.match_token(TokenType::Print);
                let expr = expr(lexer);
                stmts.push(Stmt {
                    stype: StmtType::Print(expr),
                    loc,
                });
                lexer.match_token(TokenType::SemiColon);
            }
            TokenType::Break => {
                let loc = lexer.get_token_loc();
                lexer.match_token(TokenType::Break);
                stmts.push(Stmt {
                    stype: StmtType::Break,
                    loc,
                });
                lexer.match_token(TokenType::SemiColon);
            }
            TokenType::Continue => {
                let loc = lexer.get_token_loc();
                lexer.match_token(TokenType::Continue);
                stmts.push(Stmt {
                    stype: StmtType::Continue,
                    loc,
                });
                lexer.match_token(TokenType::SemiColon);
            }
            TokenType::If => {
                let loc = lexer.get_token_loc();
                stmts.push(Stmt {
                    stype: StmtType::If(if_stmt(lexer)),
                    loc,
                });
            }
            TokenType::While => {
                let loc = lexer.get_token_loc();
                stmts.push(Stmt {
                    stype: StmtType::While(while_stmt(lexer)),
                    loc,
                });
            }
            TokenType::Return => {
                let loc = lexer.get_token_loc();
                lexer.match_token(TokenType::Return);
                stmts.push(Stmt {
                    stype: StmtType::Return(expr(lexer)),
                    loc,
                });
                lexer.match_token(TokenType::SemiColon);
            }
            TokenType::Identifier => {
                //Assgin Op
                stmts.push(assign(lexer));
            }
            TokenType::Asm => {
                let loc = lexer.get_token_loc();
                lexer.match_token(TokenType::Asm);
                lexer.match_token(TokenType::OCurly);
                let mut instructs = Vec::<String>::new();
                while lexer.get_token_type() == TokenType::String {
                    instructs.push(lexer.get_token().literal);
                    lexer.match_token(TokenType::String);
                }
                lexer.match_token(TokenType::CCurly);
                stmts.push(Stmt {
                    stype: StmtType::InlineAsm(instructs),
                    loc,
                });
            }
            _ => {
                todo!();
            }
        }
    }
    lexer.match_token(TokenType::CCurly);
    Block { stmts }
}
