use crate::{
    lexer::{Lexer, TokenType},
    parser::stmt::Stmt,
};

use super::{
    assgin::assgin,
    expr::expr,
    stmt::{if_stmt, while_stmt},
    variable_decl::variable_declare,
};

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

pub fn block(lexer: &mut Lexer) -> Block {
    lexer.match_token(TokenType::OCurly);
    let mut stmts = Vec::<Stmt>::new();
    loop {
        if lexer.get_token_type() == TokenType::CCurly {
            break;
        }
        match lexer.get_token_type() {
            TokenType::Var => {
                stmts.push(Stmt::VariableDecl(variable_declare(lexer)));
                lexer.match_token(TokenType::SemiColon);
            }
            TokenType::Print => {
                lexer.match_token(TokenType::Print);
                let expr = expr(lexer);
                stmts.push(Stmt::Print(expr));
                lexer.match_token(TokenType::SemiColon);
            }
            TokenType::Break => {
                lexer.match_token(TokenType::Break);
                stmts.push(Stmt::Break);
                lexer.match_token(TokenType::SemiColon);
            }
            TokenType::Continue => {
                lexer.match_token(TokenType::Continue);
                stmts.push(Stmt::Continue);
                lexer.match_token(TokenType::SemiColon);
            }
            TokenType::If => {
                stmts.push(Stmt::If(if_stmt(lexer)));
            }
            TokenType::While => {
                stmts.push(Stmt::While(while_stmt(lexer)));
            }
            TokenType::Return => {
                lexer.match_token(TokenType::Return);
                stmts.push(Stmt::Return(expr(lexer)));
                lexer.match_token(TokenType::SemiColon);
            }
            TokenType::Identifier => {
                //Assgin Op
                stmts.push(assgin(lexer));
            }
            TokenType::Asm => {
                lexer.match_token(TokenType::Asm);
                lexer.match_token(TokenType::OCurly);
                let mut instructs = Vec::<String>::new();
                while lexer.get_token_type() == TokenType::String {
                    instructs.push(lexer.get_token().literal);
                    lexer.match_token(TokenType::String);
                }
                lexer.match_token(TokenType::CCurly);
                stmts.push(Stmt::InlineAsm(instructs));
            }
            _ => {
                todo!();
            }
        }
    }
    lexer.match_token(TokenType::CCurly);
    Block { stmts }
}
