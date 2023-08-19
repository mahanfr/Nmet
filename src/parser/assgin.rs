use crate::{
    error_handeling::error,
    lexer::{Lexer, TokenType},
};

use super::{
    expr::{expr, Expr},
    stmt::{Stmt, StmtType},
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
        Stmt {
            stype: StmtType::Expr(left_expr),
            loc: lexer.get_token_loc(),
        }
    } else if token_type.is_assgin_token() {
        let op_type = AssginOp::from_token_type(&token_type);
        lexer.match_token(token_type);
        let right_expr = expr(lexer);
        lexer.match_token(TokenType::SemiColon);
        return Stmt {
            stype: StmtType::Assgin(Assgin {
                left: left_expr,
                right: right_expr,
                op: op_type,
            }),
            loc: lexer.get_token_loc(),
        };
    } else {
        error(
            format!("Expected Semicolon found ({})", lexer.get_token_type(),),
            lexer.get_token_loc(),
        );
    }
}
