use std::process::exit;

use crate::{
    lexer::{Lexer, TokenType},
    parser::types::type_def,
};

use super::{
    expr::{expr, Expr},
    types::VariableType,
};

#[derive(Debug, Clone)]
pub struct VariableDeclare {
    pub mutable: bool,
    pub is_static: bool,
    pub ident: String,
    pub v_type: Option<VariableType>,
    pub init_value: Option<Expr>,
}

pub fn variable_declare(lexer: &mut Lexer) -> VariableDeclare {
    lexer.match_token(TokenType::Var);
    let ident_token = lexer.get_token();
    lexer.match_token(TokenType::Identifier);
    let mut is_mutable: bool = true;
    let mut is_static: bool = false;
    let mut v_type: Option<VariableType> = None;
    let mut init_value: Option<Expr> = None;
    if lexer.get_token_type() == TokenType::ATSign {
        v_type = Some(type_def(lexer));
    }
    match lexer.get_token_type() {
        TokenType::DoubleColon => {
            is_static = true;
            is_mutable = false;
            lexer.match_token(TokenType::ColonEq);
            init_value = Some(expr(lexer));
        }
        TokenType::ColonEq => {
            is_mutable = false;
            lexer.match_token(TokenType::ColonEq);
            init_value = Some(expr(lexer));
        }
        TokenType::Eq => {
            is_mutable = true;
            lexer.match_token(TokenType::Eq);
            init_value = Some(expr(lexer));
        }
        TokenType::SemiColon => {}
        _ => {
            eprintln!(
                "Error: Expected \"=\" or \":=\" found ({:?}) at {}:{}",
                lexer.get_token_type(),
                lexer.file_path,
                lexer.get_token_loc()
            );
            exit(-1);
        }
    }
    VariableDeclare {
        mutable: is_mutable,
        is_static,
        ident: ident_token.literal,
        v_type,
        init_value,
    }
}
