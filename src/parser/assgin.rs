use std::process::exit;

use crate::lexer::{Lexer, TokenType};

use super::{
    expr::{expr, Expr},
    stmt::Stmt,
};

#[derive(Debug, Clone)]
pub struct Assgin {
    pub left: Expr,
    pub right: Expr,
    pub op: AssginOp,
}

#[derive(Debug, Clone)]
pub enum AssginOp {
    Eq,
    PlusEq,
    SubEq,
    MultiEq,
    DevideEq,
    ModEq,
}
impl AssginOp {
    pub fn from_token_type(ttype: &TokenType) -> Self {
        match ttype {
            TokenType::Eq => Self::Eq,
            TokenType::PlusEq => Self::PlusEq,
            TokenType::SubEq => Self::SubEq,
            TokenType::MultiEq => Self::MultiEq,
            TokenType::DivEq => Self::DevideEq,
            TokenType::ModEq => Self::ModEq,
            _ => {
                unreachable!();
            }
        }
    }
}

pub fn assgin(lexer: &mut Lexer) -> Stmt {
    let left_expr = expr(lexer);
    let token_type = lexer.get_token_type();
    if token_type == TokenType::SemiColon {
        lexer.match_token(TokenType::SemiColon);
        return Stmt::Expr(left_expr);
    } else if token_type.is_assgin_token() {
        let op_type = AssginOp::from_token_type(&token_type);
        lexer.match_token(token_type);
        let right_expr = expr(lexer);
        lexer.match_token(TokenType::SemiColon);
        return Stmt::Assgin(Assgin {
            left: left_expr,
            right: right_expr,
            op: op_type,
        });
    } else {
        eprintln!(
            "Error: Expected Semicolon at {}:{}",
            lexer.file_path,
            lexer.get_token_loc()
        );
        exit(-1);
    }
}
