use std::process::exit;

use crate::{error_handeling::Loc, lexer::{Lexer, TokenType}};

use super::{expr::{expr, Expr}, stmt::Stmt};


pub fn parse_pre_functions(lexer: &mut Lexer, loc: Loc, master: &String) -> Vec<Stmt> {
    lexer.match_token(TokenType::Hash);
    match lexer.get_token_type() {
        TokenType::If => {
            parse_pre_condition(lexer, loc, master)
        },
        _ => {
            eprintln!("{loc}: Unknown pre-processing function!");
            exit(-1);
        }
    }
} 

fn parse_pre_condition(lexer: &mut Lexer, loc: Loc, master: &String) -> Vec<Stmt> {
    lexer.match_token(TokenType::If);
    let cond_expr = expr(lexer);
    let result = compile_pre_expr(cond_expr);

    todo!()
}

fn compile_pre_expr(expr : Expr) {
    todo!()
}
