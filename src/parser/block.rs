use crate::{
    lexer::{Lexer, TokenType},
    parser::stmt::Stmt, compiler::BLocation,
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
    Function
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
