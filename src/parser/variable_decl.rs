/**********************************************************************************************
*
*   parser/variable_decl: parsing variable declearation and initial assginemnt
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
    error_handeling::error,
    lexer::{Lexer, TokenType},
    parser::types::type_def,
};

use super::{
    expr::{expr, Expr},
    types::VariableType,
};

/// Variable Declearation
/// * mutable - if the value of the variable can be changed
/// * static - if the variable is global
/// * ident - variable identifier
/// * v_type - Variable Type
/// * init_value - Expr of initial value
#[derive(Debug, Clone)]
pub struct VariableDeclare {
    pub mutable: bool,
    pub is_static: bool,
    pub ident: String,
    pub v_type: Option<VariableType>,
    pub init_value: Option<Expr>,
}

/// parse variable declare
pub fn inline_variable_declare(lexer: &mut Lexer) -> VariableDeclare {
    let ident_token = lexer.get_token();
    lexer.match_token(TokenType::Identifier);
    let mut is_mutable: bool = true;
    let mut is_static: bool = false;
    let mut v_type: Option<VariableType> = None;
    let mut init_value: Option<Expr> = None;
    if lexer.get_token_type() == TokenType::ATSign {
        v_type = Some(type_def(lexer));
    }
    let loc = lexer.get_current_loc();
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
        TokenType::SemiColon | TokenType::To => (),
        _ => {
            error(
                format!(
                    "Expected \"=\" or \":=\" found ({})",
                    lexer.get_token_type()
                ),
                loc,
            );
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


/// Parse Variable Declaration
pub fn variable_declare(lexer: &mut Lexer) -> VariableDeclare {
    lexer.match_token(TokenType::Var);
    inline_variable_declare(lexer)
}
